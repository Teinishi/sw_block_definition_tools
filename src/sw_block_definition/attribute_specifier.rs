use super::{
    Definition, DefinitionAttribute, DefinitionAttributeValue, SfxDataAttribute, SfxLayerAttribute,
};

pub trait AttributeEnum<T>: std::fmt::Display {
    fn get_value(&self, d: &T) -> Option<DefinitionAttributeValue>;
    fn get_value_root(&self, d: &Definition) -> Vec<DefinitionAttributeValue>;
    fn is_audio_file(&self) -> bool {
        false
    }
    fn ui_value(&self, ui: &mut egui::Ui, value: Option<&DefinitionAttributeValue>) {
        if let Some(value) = value {
            ui.horizontal(|ui| {
                // 音声ファイルのとき、再生ボタン
                if self.is_audio_file() && ui.button("\u{25B6}").clicked() {
                    todo!()
                }
                ui.label(value.debug_str());
            });
        } else {
            ui.weak("Not defined");
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum AttributeSpecifier {
    Definition(DefinitionAttribute),
    SfxData(SfxDataAttribute),
    SfxLayer(SfxLayerAttribute),
}

impl AttributeSpecifier {
    pub fn get_value_root(&self, d: &Definition) -> Vec<DefinitionAttributeValue> {
        match self {
            Self::Definition(attr) => attr.get_value_root(d),
            Self::SfxData(attr) => attr.get_value_root(d),
            Self::SfxLayer(attr) => attr.get_value_root(d),
        }
    }

    pub fn ui_value(&self, ui: &mut egui::Ui, value: Option<&DefinitionAttributeValue>) {
        match self {
            Self::Definition(attr) => attr.ui_value(ui, value),
            Self::SfxData(attr) => attr.ui_value(ui, value),
            Self::SfxLayer(attr) => attr.ui_value(ui, value),
        }
    }
}

impl std::fmt::Display for AttributeSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Definition(attr) => attr.fmt(f),
            Self::SfxData(attr) => attr.fmt(f),
            Self::SfxLayer(attr) => attr.fmt(f),
        }
    }
}

impl From<DefinitionAttribute> for AttributeSpecifier {
    fn from(value: DefinitionAttribute) -> Self {
        Self::Definition(value)
    }
}

impl From<SfxDataAttribute> for AttributeSpecifier {
    fn from(value: SfxDataAttribute) -> Self {
        Self::SfxData(value)
    }
}

impl From<SfxLayerAttribute> for AttributeSpecifier {
    fn from(value: SfxLayerAttribute) -> Self {
        Self::SfxLayer(value)
    }
}
