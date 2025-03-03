use super::{
    AttributeValue, CouplingAttribute, Definition, DefinitionAttribute, LogicNodeAttribute,
    SfxDataAttribute, SfxLayerAttribute, SurfaceAttribute, VoxelAttribute,
};
use ambassador::{delegatable_trait, delegatable_trait_remote, Delegate};
use std::fmt::Display;

#[derive(Default)]
pub struct AttributeProperty {
    pub is_audio_file: bool,
}

#[delegatable_trait_remote]
trait Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>;
}

#[delegatable_trait]
pub trait GetAttributeValueRoot: Clone + Copy + Display {
    fn get_value_root(&self, d: &Definition) -> Vec<AttributeValue>;
    fn property(&self) -> AttributeProperty {
        Default::default()
    }
}

pub trait GetAttributeValue<T>: GetAttributeValueRoot + Into<AttributeSpecifier> {
    fn get_value(&self, d: &T) -> Option<AttributeValue>;
}

#[derive(serde::Serialize, serde::Deserialize, Delegate, Clone, Copy)]
#[delegate(Display)]
#[delegate(GetAttributeValueRoot)]
pub enum AttributeSpecifier {
    Definition(DefinitionAttribute),
    SfxData(SfxDataAttribute),
    SfxLayer(SfxLayerAttribute),
    Surface(SurfaceAttribute),
    LogicNode(LogicNodeAttribute),
    Coupling(CouplingAttribute),
    Voxel(VoxelAttribute),
}
