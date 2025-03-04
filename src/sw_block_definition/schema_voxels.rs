use super::{
    attribute_specifier::GetAttributeValueRoot, AttributeProperty, AttributeSpecifier,
    AttributeValue, Definition, DefinitionVec3, GetAttributeValue, Matrix,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct Voxels {
    #[serde(default)]
    pub voxel: Vec<Voxel>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct Voxel {
    #[serde(rename = "@flags")]
    pub flags: Option<i32>,
    #[serde(rename = "@physics_shape")]
    pub physics_shape: Option<i32>,
    #[serde(rename = "@buoy_pipes")]
    pub buoy_pipes: Option<i32>,

    pub position: Vec<DefinitionVec3<i32>>,
    pub physics_shape_rotation: Vec<Matrix>,
}

#[derive(
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    strum::Display,
    strum::VariantArray,
    Clone,
    Copy,
)]
#[strum(serialize_all = "snake_case")]
pub enum VoxelAttribute {
    Position,
    PhysicsShapeRotation,
    Flags,
    PhysicsShape,
    BuoyPipes,
}

impl GetAttributeValueRoot for VoxelAttribute {
    fn get_value_root(&self, d: &Definition) -> Vec<AttributeValue> {
        if let Some(voxels) = d.voxels.last() {
            voxels
                .voxel
                .iter()
                .filter_map(|item| self.get_value(item))
                .collect()
        } else {
            vec![]
        }
    }

    fn property(&self) -> AttributeProperty {
        let is_not_number = matches!(self, Self::Position | Self::PhysicsShapeRotation);
        AttributeProperty {
            is_audio_file: false,
            is_number: !is_not_number,
        }
    }
}

impl GetAttributeValue<Voxel> for VoxelAttribute {
    fn get_value(&self, d: &Voxel) -> Option<AttributeValue> {
        match self {
            Self::Position => Some((*d.position.last()?).into()),
            Self::PhysicsShapeRotation => Some((*d.physics_shape_rotation.last()?).into()),
            Self::Flags => Some(d.flags?.into()),
            Self::PhysicsShape => Some(d.physics_shape?.into()),
            Self::BuoyPipes => Some(d.buoy_pipes?.into()),
        }
    }
}

impl From<VoxelAttribute> for AttributeSpecifier {
    fn from(value: VoxelAttribute) -> Self {
        Self::Voxel(value)
    }
}
