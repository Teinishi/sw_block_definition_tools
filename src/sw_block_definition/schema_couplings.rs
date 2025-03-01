use super::Position;
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
