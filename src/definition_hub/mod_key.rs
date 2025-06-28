#[derive(
    serde::Serialize, serde::Deserialize, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Clone,
)]
pub enum ModKey {
    Stormworks,
    Local(String),
    Workshop(String),
}
