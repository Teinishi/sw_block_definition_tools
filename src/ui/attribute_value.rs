use crate::sw_block_definition::{AttributeProperty, AttributeValue, SwBlockDefinition};

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
                    let _audio_path = definition.rom_path().join(s);
                }
            }
        }
    }
}
