use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename = "definition", default, deny_unknown_fields)]
pub struct Mod {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@author")]
    pub author: String,
    #[serde(rename = "@desc")]
    pub desc: String,
}
