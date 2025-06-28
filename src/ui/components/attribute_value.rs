use crate::{
    audio::play_stop_audio,
    state::State,
    sw_block_definition::{AttributeType, AttributeValue, DisplayAttributeValue},
};
use egui::{
    text::LayoutJob, Align, Button, FontFamily, FontId, Layout, RichText, TextFormat, TextStyle,
};

pub fn ui_attribute_value(
    ui: &mut egui::Ui,
    state: &mut State,
    attribute_type: &AttributeType,
    value: Option<&AttributeValue>,
    number_right: bool,
    margin: Option<(f32, f32)>,
) {
    let reverse = number_right && attribute_type.is_number();

    let (margin1, margin2) = margin
        .map(|(l, r)| (Some(l), Some(r)))
        .unwrap_or((None, None));

    let halign = if reverse { Align::RIGHT } else { Align::LEFT };
    ui.with_layout(Layout::top_down(halign), |ui| {
        ui.horizontal(|ui| {
            if reverse {
                add_space(ui, margin2);
                value_display(ui, attribute_type, value);
                add_space(ui, margin1);
            } else {
                add_space(ui, margin1);
                if attribute_type.is_audio_file() {
                    if let Some(AttributeValue::String(path)) = value {
                        audio_play_button(ui, state, path);
                    }
                }
                value_display(ui, attribute_type, value);
                add_space(ui, margin2);
            }
        });
    });
}

fn audio_play_button(ui: &mut egui::Ui, state: &mut State, path: &String) {
    let is_playing = state
        .playing_audio()
        .as_ref()
        .is_some_and(|playing_audio| playing_audio.path == *path);

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

fn add_space(ui: &mut egui::Ui, amount: Option<f32>) {
    if let Some(a) = amount {
        ui.add_space(a);
    }
}

fn value_display(
    ui: &mut egui::Ui,
    attribute_type: &AttributeType,
    value: Option<&AttributeValue>,
) {
    if let Some(value) = value {
        match value {
            AttributeValue::VecI32(vec) => {
                let mut label = BlendColorLabel::with_capacity(7).monospace();
                label.add("(");
                for (i, x) in [vec.x, vec.y, vec.z].iter().enumerate() {
                    if i != 0 {
                        label.add(", ");
                    }
                    if let Some(x) = x {
                        label.add(format!("{:2}", x));
                    } else {
                        label.add_weak(" 0");
                    }
                }
                label.add(")");
                label.show(ui);
            }
            AttributeValue::VecF32(vec) => {
                let mut label = BlendColorLabel::with_capacity(7).monospace();
                label.add("(");
                for (i, x) in [vec.x, vec.y, vec.z].iter().enumerate() {
                    if i != 0 {
                        label.add(", ");
                    }
                    if let Some(x) = x {
                        label.add(format!("{:>4?}", x));
                    } else {
                        label.add_weak(" 0.0");
                    }
                }
                label.add(")");
                label.show(ui);
            }
            AttributeValue::U64(value) if matches!(attribute_type, AttributeType::Flags) => {
                ui.monospace(format!("{:10} (0b{:033b})", value, value));
            }
            _ => {
                ui.monospace(value.display_string());
            }
        }
    } else {
        ui.monospace(RichText::new(attribute_type.undefined_text()).weak());
    }
}

enum LabelText {
    Text(String),
    Weak(String),
}

struct BlendColorLabel {
    text: Vec<LabelText>,
    font_family: FontFamily,
}

impl BlendColorLabel {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            text: Vec::with_capacity(capacity),
            font_family: FontFamily::Proportional,
        }
    }

    fn monospace(mut self) -> Self {
        self.font_family = FontFamily::Monospace;
        self
    }

    fn add(&mut self, text: impl Into<String>) {
        self.text.push(LabelText::Text(text.into()));
    }

    fn add_weak(&mut self, text: impl Into<String>) {
        self.text.push(LabelText::Weak(text.into()));
    }

    fn show(&self, ui: &mut egui::Ui) {
        let visuals = ui.visuals();
        let text_color = visuals.text_color();
        let weak_text_color = visuals.weak_text_color();

        let size = ui
            .style()
            .text_styles
            .iter()
            .find_map(|(text_style, font_id)| {
                if matches!(text_style, TextStyle::Body) {
                    Some(font_id.size)
                } else {
                    None
                }
            })
            .unwrap_or(12.5);

        let mut job = LayoutJob::default();
        for t in &self.text {
            let (s, color) = match t {
                LabelText::Text(s) => (s, text_color),
                LabelText::Weak(s) => (s, weak_text_color),
            };
            job.append(
                s,
                0.0,
                TextFormat {
                    font_id: FontId::new(size, FontFamily::Monospace),
                    color,
                    ..Default::default()
                },
            )
        }

        ui.label(job);
    }
}
