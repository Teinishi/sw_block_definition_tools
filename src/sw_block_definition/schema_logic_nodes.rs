use super::Position;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct LogicNodes {
    #[serde(default)]
    pub logic_node: Vec<LogicNode>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct LogicNode {
    #[serde(rename = "@orientation")]
    pub orientation: Option<i32>,
    #[serde(rename = "@label")]
    pub label: Option<String>,
    #[serde(rename = "@mode")]
    pub mode: Option<i32>,
    #[serde(rename = "@type")]
    pub node_type: Option<i32>,
    #[serde(rename = "@description")]
    pub description: Option<String>,
    #[serde(rename = "@flags")]
    pub flags: Option<u64>,

    pub position: Vec<Position>,
}
