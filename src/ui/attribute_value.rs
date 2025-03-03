use super::State;
use crate::sw_block_definition::{AttributeProperty, AttributeValue};

pub fn ui_attribute_value(
    ui: &mut egui::Ui,
    attribute_property: AttributeProperty,
    value: Option<&AttributeValue>,
    action: &mut Option<AttributeValueAction>,
) {
    if let Some(value) = value {
        ui.horizontal(|ui| {
            // 音声ファイルのとき、再生ボタン
            if attribute_property.is_audio_file && ui.button("\u{25B6}").clicked() {
                *action = Some(AttributeValueAction::PlayAudio(value.clone()));
            }
            ui.label(value.debug_str());
        });
    } else {
        ui.weak("Not defined");
    }
}

#[derive(Debug)]
pub enum AttributeValueAction {
    PlayAudio(AttributeValue),
}

impl AttributeValueAction {
    pub fn do_action(action: AttributeValueAction, state: &State) {
        match action {
            AttributeValueAction::PlayAudio(value) => {
                if let AttributeValue::String(s) = value {
                    if let Some(rom_path) = state.rom_path() {
                        play_audio(rom_path.join(s), 0.5); // TODO: rx でエラーを拾う 音量調節もできるようにする
                    }
                }
            }
        }
    }
}

fn play_audio(
    path: std::path::PathBuf,
    volume: f32,
) -> std::sync::mpsc::Receiver<Result<(), PlayAudioErr>> {
    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        let r: Result<(), PlayAudioErr> = match std::fs::File::open(path) {
            Err(err) => Err(err.into()),
            Ok(file) => match rodio::OutputStream::try_default() {
                Err(err) => Err(err.into()),
                Ok((_stream, stream_handle)) => {
                    match stream_handle.play_once(std::io::BufReader::new(file)) {
                        Err(err) => Err(err.into()),
                        Ok(sink) => {
                            sink.set_volume(volume);
                            sink.sleep_until_end();
                            Ok(())
                        }
                    }
                }
            },
        };
        tx.send(r).unwrap_or_default();
    });

    rx
}

enum PlayAudioErr {
    #[allow(dead_code)]
    Io(std::io::Error),
    #[allow(dead_code)]
    Stream(rodio::StreamError),
    #[allow(dead_code)]
    Play(rodio::PlayError),
}

impl From<std::io::Error> for PlayAudioErr {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<rodio::StreamError> for PlayAudioErr {
    fn from(value: rodio::StreamError) -> Self {
        Self::Stream(value)
    }
}

impl From<rodio::PlayError> for PlayAudioErr {
    fn from(value: rodio::PlayError) -> Self {
        Self::Play(value)
    }
}

impl std::fmt::Display for PlayAudioErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => err.fmt(f),
            Self::Stream(err) => err.fmt(f),
            Self::Play(err) => err.fmt(f),
        }
    }
}
