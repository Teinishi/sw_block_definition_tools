use crate::sw_block_definition::{AttributeProperty, AttributeValue, SwBlockDefinition};
use std::{
    fmt::Display,
    fs::File,
    io::{self, BufReader},
    path::PathBuf,
    sync::mpsc,
    thread,
};

pub fn ui_attribute_value(
    ui: &mut egui::Ui,
    attribute_property: AttributeProperty,
    value: Option<&AttributeValue>,
) -> Option<AttributeValueAction> {
    let mut action = None;
    if let Some(value) = value {
        ui.horizontal(|ui| {
            // 音声ファイルのとき、再生ボタン
            if attribute_property.is_audio_file && ui.button("\u{25B6}").clicked() {
                action = Some(AttributeValueAction::PlayAudio(value.clone()));
            }
            ui.label(value.debug_str());
        });
    } else {
        ui.weak("Not defined");
    }
    action
}

#[derive(Debug)]
pub enum AttributeValueAction {
    PlayAudio(AttributeValue),
}

impl AttributeValueAction {
    pub fn do_action(action: AttributeValueAction, definition: &SwBlockDefinition) {
        match action {
            AttributeValueAction::PlayAudio(value) => {
                if let AttributeValue::String(s) = value {
                    let audio_path = definition.rom_path().join(s);
                    play_audio(audio_path, 0.5); // TODO: rx でエラーを拾う 音量調節もできるようにする
                }
            }
        }
    }
}

fn play_audio(path: PathBuf, volume: f32) -> mpsc::Receiver<Result<(), PlayAudioErr>> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let r: Result<(), PlayAudioErr> = match File::open(path) {
            Err(err) => Err(err.into()),
            Ok(file) => match rodio::OutputStream::try_default() {
                Err(err) => Err(err.into()),
                Ok((_stream, stream_handle)) => match stream_handle.play_once(BufReader::new(file))
                {
                    Err(err) => Err(err.into()),
                    Ok(sink) => {
                        sink.set_volume(volume);
                        sink.sleep_until_end();
                        Ok(())
                    }
                },
            },
        };
        tx.send(r).unwrap_or_default();
    });

    rx
}

enum PlayAudioErr {
    #[allow(dead_code)]
    Io(io::Error),
    #[allow(dead_code)]
    Stream(rodio::StreamError),
    #[allow(dead_code)]
    Play(rodio::PlayError),
}

impl From<io::Error> for PlayAudioErr {
    fn from(value: io::Error) -> Self {
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

impl Display for PlayAudioErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => err.fmt(f),
            Self::Stream(err) => err.fmt(f),
            Self::Play(err) => err.fmt(f),
        }
    }
}
