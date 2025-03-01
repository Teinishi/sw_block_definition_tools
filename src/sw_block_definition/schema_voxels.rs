use super::Position;
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

fn one() -> i32 {
    1
}
