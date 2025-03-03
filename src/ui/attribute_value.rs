use super::{play_stop_audio, State};
use crate::sw_block_definition::{AttributeProperty, AttributeValue};
use egui::Button;

pub fn ui_attribute_value(
    ui: &mut egui::Ui,
    state: &State,
    attribute_property: AttributeProperty,
    value: Option<&AttributeValue>,
    action: &mut Option<AttributeValueAction>,
) {
    if let Some(value) = value {
        ui.horizontal(|ui| {
            // 音声ファイルのとき、再生ボタン
            if attribute_property.is_audio_file {
                if let AttributeValue::String(path) = value {
                    let is_playing = state
                        .playing_audio()
                        .as_ref()
                        .map_or(false, |(playing_path, _, _)| playing_path == path);

                    let button = ui.add_sized(
                        [20.0, 20.0],
                        Button::new(if is_playing { "\u{23F8}" } else { "\u{25B6}" }).truncate(),
                    );
                    if button.clicked() {
                        *action = Some(AttributeValueAction::PlayAudio(path.clone()));
                    }
                }
            }
            ui.label(value.debug_str());
        });
    } else {
        ui.weak("Not defined");
    }
}

#[derive(Debug)]
pub enum AttributeValueAction {
    PlayAudio(String),
}

impl AttributeValueAction {
    pub fn do_action(action: AttributeValueAction, state: &mut State) {
        match action {
            AttributeValueAction::PlayAudio(path) => {
                play_stop_audio(path, state).unwrap(); // TODO: エラー表示
            }
        }
    }
}
