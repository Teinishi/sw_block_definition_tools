use super::{play_stop_audio, State};
use crate::sw_block_definition::{AttributeType, AttributeValue, DisplayAttributeValue};
use egui::{Align, Button, Layout, RichText};

pub fn ui_attribute_value(
    ui: &mut egui::Ui,
    state: &mut State,
    attribute_type: &AttributeType,
    value: Option<&AttributeValue>,
    number_right: bool,
    margin: Option<(f32, f32)>,
) {
    ui.horizontal(|ui| {
        let is_r2l = number_right && attribute_type.is_number();

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

        if let Some(value) = value {
            // 音声ファイルのとき、再生ボタン
            if attribute_type.is_audio_file() {
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
        }

        ui.with_layout(layout, |ui| {
            if let Some(m) = margin1 {
                ui.add_space(m);
            }
            value_display(ui, attribute_type, value);
            if let Some(m) = margin2 {
                ui.add_space(m);
            }
        });
    });
}

fn value_display(
    ui: &mut egui::Ui,
    attribute_type: &AttributeType,
    value: Option<&AttributeValue>,
) {
    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
        //let width = ui.fonts(|f| f.glyph_width(&TextStyle::Body.resolve(ui.style()), ''));
        ui.spacing_mut().item_spacing.x = 0.0;

        if let Some(value) = value {
            match value {
                AttributeValue::VecI32(vec) => {
                    ui.monospace("(");
                    for (i, x) in [vec.x, vec.y, vec.z].iter().enumerate() {
                        if i != 0 {
                            ui.monospace(", ");
                        }
                        if let Some(x) = x {
                            ui.monospace(format!("{:2}", x));
                        } else {
                            ui.monospace(RichText::new(" 0").weak());
                        }
                    }
                    ui.monospace(")");
                }
                AttributeValue::VecF32(vec) => {
                    ui.monospace("(");
                    for (i, x) in [vec.x, vec.y, vec.z].iter().enumerate() {
                        if i != 0 {
                            ui.monospace(", ");
                        }
                        if let Some(x) = x {
                            ui.monospace(format!("{:>4?}", x));
                        } else {
                            ui.monospace(RichText::new(" 0.0").weak());
                        }
                    }
                    ui.monospace(")");
                }
                _ => {
                    ui.monospace(value.display_string());
                }
            }
        } else {
            ui.monospace(RichText::new(attribute_type.undefined_text()).weak());
        }
    });
}
