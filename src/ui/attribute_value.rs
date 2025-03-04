use super::{play_stop_audio, State};
use crate::sw_block_definition::{AttributeProperty, AttributeValue, DisplayAttributeValue};
use egui::{Align, Button, Layout};

pub fn ui_attribute_value(
    ui: &mut egui::Ui,
    state: &mut State,
    attribute_property: &AttributeProperty,
    value: Option<&AttributeValue>,
    number_right: bool,
    margin: Option<(f32, f32)>,
) {
    if let Some(value) = value {
        ui.horizontal(|ui| {
            // 音声ファイルのとき、再生ボタン
            if attribute_property.is_audio_file {
                if let AttributeValue::String(path) = value {
                    let is_playing = state
                        .playing_audio()
                        .as_ref()
                        .is_some_and(|(playing_path, _, _)| playing_path == path);

                    let button = ui.add_sized(
                        [20.0, 20.0],
                        Button::new(if is_playing { "\u{23F8}" } else { "\u{25B6}" }).truncate(),
                    );
                    if button.clicked() {
                        if let Err(err) = play_stop_audio(path.clone(), state) {
                            println!("{:?}", err); // TODO: GUI表示
                        }
                    }
                }
            }

            let is_r2l = number_right && attribute_property.is_number;

            let (mut margin1, mut margin2) = margin
                .map(|(l, r)| (Some(l), Some(r)))
                .unwrap_or((None, None));
            if is_r2l {
                (margin1, margin2) = (margin2, margin1);
            }

            let layout = if is_r2l {
                Layout::right_to_left(Align::Center)
            } else {
                Layout::left_to_right(Align::Center)
            };
            ui.with_layout(layout, |ui| {
                if let Some(m) = margin1 {
                    ui.add_space(m);
                }
                ui.label(value.display_string());
                if let Some(m) = margin2 {
                    ui.add_space(m);
                }
            });
        });
    } else {
        ui.weak("Not defined");
    }
}
