use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename = "definition", default, deny_unknown_fields)]
pub struct Mod {
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "@author")]
    pub author: Option<String>,
    #[serde(rename = "@desc")]
    pub desc: Option<String>,
    #[serde(rename = "@workshop_id")]
    pub workshop_id: Option<String>,
}
