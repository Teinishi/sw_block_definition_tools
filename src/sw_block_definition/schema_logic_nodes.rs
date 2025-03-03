use super::{
    attribute_specifier::GetAttributeValueRoot, GetAttributeValue, AttributeSpecifier, AttributeValue,
    Definition, Position,
};
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
pub enum LogicNodeAttribute {
    X,
    Y,
    Z,
    Orientation,
    Label,
    Mode,
    NodeType,
    Description,
    Flags,
}

impl GetAttributeValueRoot for LogicNodeAttribute {
    fn get_value_root(&self, d: &Definition) -> Vec<AttributeValue> {
        if let Some(logic_nodes) = d.logic_nodes.last() {
            logic_nodes
                .logic_node
                .iter()
                .filter_map(|item| self.get_value(item))
                .collect()
        } else {
            vec![]
        }
    }
}

impl GetAttributeValue<LogicNode> for LogicNodeAttribute {
    fn get_value(&self, d: &LogicNode) -> Option<AttributeValue> {
        match self {
            Self::X => Some(d.position.last()?.x?.into()),
            Self::Y => Some(d.position.last()?.y?.into()),
            Self::Z => Some(d.position.last()?.z?.into()),
            Self::Orientation => Some(d.orientation?.into()),
            Self::Label => Some(d.label.clone()?.into()),
            Self::Mode => Some(d.mode?.into()),
            Self::NodeType => Some(d.node_type?.into()),
            Self::Description => Some(d.description.clone()?.into()),
            Self::Flags => Some(d.flags?.into()),
        }
    }
}

impl From<LogicNodeAttribute> for AttributeSpecifier {
    fn from(value: LogicNodeAttribute) -> Self {
        Self::LogicNode(value)
    }
}
