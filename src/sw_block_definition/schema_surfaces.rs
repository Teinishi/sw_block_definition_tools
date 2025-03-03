use super::{
    attribute_specifier::GetAttributeValueRoot, AttributeSpecifier, AttributeValue, Definition,
    GetAttributeValue, Position,
};
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
pub enum SurfaceAttribute {
    Position,
    Orientation,
    Rotation,
    Shape,
    TransType,
    Flags,
    IsReverseNormals,
    IsTwoSided,
}

impl GetAttributeValueRoot for SurfaceAttribute {
    fn get_value_root(&self, d: &Definition) -> Vec<AttributeValue> {
        if let Some(surfaces) = d.surfaces.last() {
            surfaces
                .surface
                .iter()
                .filter_map(|item| self.get_value(item))
                .collect()
        } else {
            vec![]
        }
    }
}

impl GetAttributeValue<Surface> for SurfaceAttribute {
    fn get_value(&self, d: &Surface) -> Option<AttributeValue> {
        match self {
            Self::Position => Some((*d.position.last()?).into()),
            Self::Orientation => Some(d.orientation?.into()),
            Self::Rotation => Some(d.rotation?.into()),
            Self::Shape => Some(d.shape?.into()),
            Self::TransType => Some(d.trans_type?.into()),
            Self::Flags => Some(d.flags?.into()),
            Self::IsReverseNormals => Some(d.is_reverse_normals?.into()),
            Self::IsTwoSided => Some(d.is_two_sided?.into()),
        }
    }
}

impl From<SurfaceAttribute> for AttributeSpecifier {
    fn from(value: SurfaceAttribute) -> Self {
        Self::Surface(value)
    }
}
