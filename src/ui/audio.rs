use std::{
    fs::File,
    io::{self, BufReader},
    path::PathBuf,
    sync::{mpsc, Arc},
    thread,
};

use super::State;

pub fn play_stop_audio(path: String, state: &mut State) -> Result<(), PlayAudioError> {
    if let Some((playing_path, sink, _)) = state.playing_audio() {
        if *playing_path == path {
            sink.stop();
            return Ok(());
        }
    }

    if let Some(rom_path) = &state.rom_path {
        match spawn_audio(rom_path.join(path.clone()), state.audio_volume) {
            Ok((sink, rx_done)) => {
                state.set_playing_audio(Some((path, sink, rx_done)));
                Ok(())
            }
            Err(err) => Err(err),
        }
    } else {
        Err(PlayAudioError::NoRomDirectory)
    }
}

fn spawn_audio(
    path: PathBuf,
    volume: f32,
) -> Result<(Arc<rodio::Sink>, mpsc::Receiver<bool>), PlayAudioError> {
    let (tx_init, rx_init) = mpsc::channel();
    let (tx_done, rx_done) = mpsc::channel();

    thread::spawn(move || {
        if let Err(err) = audio_thread(path, tx_init.clone(), tx_done) {
            tx_init.send(Err(err)).unwrap();
        }
    });

    match rx_init.recv().unwrap() {
        Ok(sink) => {
            sink.set_volume(volume);
            Ok((sink, rx_done))
        }
        Err(err) => Err(err),
    }
}

fn audio_thread(
    path: PathBuf,
    tx_init: mpsc::Sender<Result<Arc<rodio::Sink>, PlayAudioError>>,
    tx_done: mpsc::Sender<bool>,
) -> Result<(), PlayAudioError> {
    let file = BufReader::new(File::open(path)?);
    let (_stream, stram_handle) = rodio::OutputStream::try_default()?;
    let sink = Arc::new(stram_handle.play_once(file)?);
    tx_init.send(Ok(sink.clone())).unwrap();
    sink.sleep_until_end();
    tx_done.send(true).unwrap_or_default();
    Ok(())
}

#[derive(Debug)]
pub enum PlayAudioError {
    #[allow(dead_code)]
    Io(io::Error),
    #[allow(dead_code)]
    Stream(rodio::StreamError),
    #[allow(dead_code)]
    Play(rodio::PlayError),
    NoRomDirectory,
}

impl From<io::Error> for PlayAudioError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<rodio::StreamError> for PlayAudioError {
    fn from(value: rodio::StreamError) -> Self {
        Self::Stream(value)
    }
}

impl From<rodio::PlayError> for PlayAudioError {
    fn from(value: rodio::PlayError) -> Self {
        Self::Play(value)
    }
}

impl std::fmt::Display for PlayAudioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => err.fmt(f),
            Self::Stream(err) => err.fmt(f),
            Self::Play(err) => err.fmt(f),
            Self::NoRomDirectory => write!(f, "No rom directory"),
        }
    }
}
