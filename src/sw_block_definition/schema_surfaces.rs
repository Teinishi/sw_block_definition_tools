use super::Position;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct Surfaces {
    #[serde(default)]
    pub surface: Vec<Surface>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct BuoyancySurfaces {
    #[serde(default)]
    pub surface: Vec<Surface>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct Surface {
    #[serde(rename = "@orientation")]
    pub orientation: Option<i32>,
    #[serde(rename = "@rotation")]
    pub rotation: Option<i32>,
    #[serde(rename = "@shape")]
    pub shape: Option<i32>,
    #[serde(rename = "@trans_type")]
    pub trans_type: Option<i32>,
    #[serde(rename = "@flags")]
    pub flags: Option<u64>,
    #[serde(rename = "@is_reverse_normals")]
    pub is_reverse_normals: Option<bool>,
    #[serde(rename = "@is_two_sided")]
    pub is_two_sided: Option<bool>,

    pub position: Vec<Position>,
}
