use crate::state::State;
use std::path::PathBuf;

#[derive(
    serde::Serialize, serde::Deserialize, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Clone,
)]
pub enum ModKey {
    Stormworks,
    Local(String),
    Workshop(String),
}

impl ModKey {
    pub fn get_path(&self, state: &State) -> Option<PathBuf> {
        match self {
            Self::Stormworks => state.rom_path.clone(),
            Self::Local(folder_name) => state.mods_path.as_ref().map(|p| p.join(folder_name)),
            Self::Workshop(folder_name) => {
                state.workshop_path.as_ref().map(|p| p.join(folder_name))
            }
        }
    }

    pub fn get_folder_name(&self) -> String {
        match self {
            Self::Stormworks => "stormworks".to_string(),
            Self::Local(folder_name) => folder_name.to_string(),
            Self::Workshop(folder_name) => format!("workshop {}", folder_name),
        }
    }
}
