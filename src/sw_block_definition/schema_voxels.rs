use super::{
    attribute_specifier::GetAttributeValueRoot, GetAttributeValue, AttributeSpecifier, AttributeValue,
    Definition, Position,
};
use serde::{Deserialize, Serialize};

fn one() -> i32 {
    1
}

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

    pub position: Vec<Position>,
    pub physics_shape_rotation: Vec<PhysicsShapeRotation>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct PhysicsShapeRotation {
    #[serde(rename = "@00", default = "one")]
    pub r00: i32,
    #[serde(rename = "@01")]
    pub r01: i32,
    #[serde(rename = "@02")]
    pub r02: i32,
    #[serde(rename = "@10")]
    pub r10: i32,
    #[serde(rename = "@11", default = "one")]
    pub r11: i32,
    #[serde(rename = "@12")]
    pub r12: i32,
    #[serde(rename = "@20")]
    pub r20: i32,
    #[serde(rename = "@21")]
    pub r21: i32,
    #[serde(rename = "@22", default = "one")]
    pub r22: i32,
}

impl Default for PhysicsShapeRotation {
    fn default() -> Self {
        Self {
            r00: 1,
            r01: 0,
            r02: 0,
            r10: 0,
            r11: 1,
            r12: 0,
            r20: 0,
            r21: 0,
            r22: 1,
        }
    }
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
    X,
    Y,
    Z,
    //PhysicsShapeRotation,
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
}

impl GetAttributeValue<Voxel> for VoxelAttribute {
    fn get_value(&self, d: &Voxel) -> Option<AttributeValue> {
        match self {
            Self::X => Some(d.position.last()?.x?.into()),
            Self::Y => Some(d.position.last()?.y?.into()),
            Self::Z => Some(d.position.last()?.z?.into()),
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
