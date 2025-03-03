use super::{AttributeEnum, AttributeSpecifier, AttributeValue, Definition, Position};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct Couplings {
    #[serde(default)]
    pub coupling: Vec<Coupling>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct Coupling {
    #[serde(rename = "@orientation")]
    pub orientation: Option<i32>,
    #[serde(rename = "@alignment")]
    pub alignment: Option<i32>,
    #[serde(rename = "@coupling_type")]
    pub coupling_type: Option<String>,
    #[serde(rename = "@coupling_name")]
    pub coupling_name: Option<String>,
    #[serde(rename = "@coupling_gender")]
    pub coupling_gender: Option<i32>,
    #[serde(rename = "@alignment_required")]
    pub alignment_required: Option<bool>,
    #[serde(rename = "@allow_bipolar_alignment")]
    pub allow_bipolar_alignment: Option<bool>,

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
pub enum CouplingAttribute {
    X,
    Y,
    Z,
    Orientation,
    Alignment,
    CouplingType,
    CouplingName,
    CouplingGender,
    AlignmentRequired,
    AllowBipolarALignment,
}

impl AttributeEnum<Coupling> for CouplingAttribute {
    fn get_value(&self, d: &Coupling) -> Option<AttributeValue> {
        match self {
            Self::X => Some(d.position.last()?.x?.into()),
            Self::Y => Some(d.position.last()?.y?.into()),
            Self::Z => Some(d.position.last()?.z?.into()),
            Self::Orientation => Some(d.orientation?.into()),
            Self::Alignment => Some(d.alignment?.into()),
            Self::CouplingType => Some(d.coupling_type.clone()?.into()),
            Self::CouplingName => Some(d.coupling_name.clone()?.into()),
            Self::CouplingGender => Some(d.coupling_gender?.into()),
            Self::AlignmentRequired => Some(d.alignment_required?.into()),
            Self::AllowBipolarALignment => Some(d.allow_bipolar_alignment?.into()),
        }
    }

    fn get_value_root(&self, d: &Definition) -> Vec<AttributeValue> {
        if let Some(couplings) = d.couplings.last() {
            couplings
                .coupling
                .iter()
                .filter_map(|item| self.get_value(item))
                .collect()
        } else {
            vec![]
        }
    }
}

impl From<CouplingAttribute> for AttributeSpecifier {
    fn from(value: CouplingAttribute) -> Self {
        Self::Coupling(value)
    }
}
