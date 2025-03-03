use super::{
    AttributeValue, CouplingAttribute, Definition, DefinitionAttribute, LogicNodeAttribute,
    SfxDataAttribute, SfxLayerAttribute, SurfaceAttribute,
};

#[derive(Default)]
pub struct AttributeProperty {
    pub is_audio_file: bool,
}

pub trait AttributeEnum<T>: std::fmt::Display + Clone + Copy + Into<AttributeSpecifier> {
    fn get_value(&self, d: &T) -> Option<AttributeValue>;
    fn get_value_root(&self, d: &Definition) -> Vec<AttributeValue>;
    fn property(&self) -> AttributeProperty {
        Default::default()
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum AttributeSpecifier {
    Definition(DefinitionAttribute),
    SfxData(SfxDataAttribute),
    SfxLayer(SfxLayerAttribute),
    Surface(SurfaceAttribute),
    LogicNode(LogicNodeAttribute),
    Coupling(CouplingAttribute),
}

impl AttributeSpecifier {
    pub fn get_value_root(&self, d: &Definition) -> Vec<AttributeValue> {
        match self {
            Self::Definition(attr) => attr.get_value_root(d),
            Self::SfxData(attr) => attr.get_value_root(d),
            Self::SfxLayer(attr) => attr.get_value_root(d),
            Self::Surface(attr) => attr.get_value_root(d),
            Self::LogicNode(attr) => attr.get_value_root(d),
            Self::Coupling(attr) => attr.get_value_root(d),
        }
    }

    pub fn property(&self) -> AttributeProperty {
        match self {
            Self::Definition(attr) => attr.property(),
            Self::SfxData(attr) => attr.property(),
            Self::SfxLayer(attr) => attr.property(),
            Self::Surface(attr) => attr.property(),
            Self::LogicNode(attr) => attr.property(),
            Self::Coupling(attr) => attr.property(),
        }
    }
}

impl std::fmt::Display for AttributeSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Definition(attr) => attr.fmt(f),
            Self::SfxData(attr) => attr.fmt(f),
            Self::SfxLayer(attr) => attr.fmt(f),
            Self::Surface(attr) => attr.fmt(f),
            Self::LogicNode(attr) => attr.fmt(f),
            Self::Coupling(attr) => attr.fmt(f),
        }
    }
}
