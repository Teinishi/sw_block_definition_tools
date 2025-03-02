use super::{
    Definition, DefinitionAttribute, DefinitionAttributeValue, SfxDataAttribute, SfxLayerAttribute,
};

pub trait AttributeEnum<T>: std::fmt::Display {
    fn get_value(&self, d: &T) -> Option<DefinitionAttributeValue>;
    fn get_value_root(&self, d: &Definition) -> Vec<DefinitionAttributeValue>;
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
