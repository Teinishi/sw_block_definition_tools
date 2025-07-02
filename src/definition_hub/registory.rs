use super::{ModDefinition, ModKey};
use crate::{
    definition_hub::BlockDefinition,
    state::State,
    value_tracker::{AttachVersion, CheckUpdate, TrackableBTreeMap, VersionCounter},
};
use std::{fs::read_dir, io, path::Path};

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
pub enum LoadingState {
    ModManifest(String),
    Data(String),
    Mesh(String),
}

impl LoadingState {
    pub fn get_text(&self) -> String {
        match self {
            Self::ModManifest(name) => format!("{} manifest", name),
            Self::Data(name) => name.to_string(),
            Self::Mesh(name) => format!("{} mesh", name),
        }
    }
}

#[derive(Debug, Default)]
pub struct DefinitionRegistory {
    mods: TrackableBTreeMap<ModKey, ModDefinition>,
}

impl DefinitionRegistory {
    pub fn init(&mut self, state: &State) {
        if let Some(path) = &state.rom_path {
            self.add_mod(ModDefinition::new(ModKey::Stormworks, path));
        }
        if let Some(path) = &state.mods_path {
            let _ = self.add_mods_in_folder(path, ModKey::Local);
        }
        if let Some(path) = &state.workshop_path {
            let _ = self.add_mods_in_folder(path, ModKey::Workshop);
        }
        self.mods.attach_version(&VersionCounter::zero());
    }

    pub fn add_mods_in_folder<P: AsRef<Path>, F: FnMut(String) -> ModKey>(
        &mut self,
        path: P,
        mut mod_key_gen: F,
    ) -> io::Result<()> {
        // フォルダ内のmodをスキャンしてまとめて追加
        for entry in (read_dir(path)?).flatten() {
            let entry_path = entry.path();
            if !entry_path.is_dir() || !entry_path.join("mod.xml").is_file() {
                continue;
            }
            let folder_name = entry.file_name().into_string().unwrap();
            self.add_mod(ModDefinition::new(mod_key_gen(folder_name), entry_path));
        }

        Ok(())
    }

    pub fn add_mod(&mut self, mod_definition: ModDefinition) {
        self.mods
            .insert(mod_definition.mod_key().clone(), mod_definition);
    }

    pub fn mods(&self) -> &TrackableBTreeMap<ModKey, ModDefinition> {
        &self.mods
    }

    pub fn mods_mut(&mut self) -> &mut TrackableBTreeMap<ModKey, ModDefinition> {
        &mut self.mods
    }

    pub fn definitions(
        &self,
    ) -> impl Iterator<Item = ((&ModKey, &String), &ModDefinition, &BlockDefinition)> {
        self.mods.iter().flat_map(|(mod_key, mod_definition)| {
            mod_definition
                .definitions()
                .iter()
                .map(move |(filename, block)| ((mod_key, filename), mod_definition, block))
        })
    }

    pub fn get(&self, key: &(ModKey, String)) -> Option<&BlockDefinition> {
        self.mods
            .get(&key.0)
            .and_then(|mod_definition| mod_definition.definitions().get(&key.1))
    }

    pub fn resolve(&self, mod_key: &ModKey, name: &str) -> Option<&BlockDefinition> {
        let name = format!("{}.xml", name);
        self.get(&(mod_key.clone(), name.clone()))
            .or_else(|| self.get(&(ModKey::Stormworks, name)))
    }

    pub fn load_all(&self) -> usize {
        let mut loading_count = 0;
        for mod_definition in self.mods.values() {
            loading_count += mod_definition.load_all();
        }
        loading_count
    }

    pub fn loading_state(&self) -> Option<LoadingState> {
        for (mod_key, mod_definition) in self.mods.iter() {
            if mod_definition.is_loading_manifest() {
                return Some(LoadingState::ModManifest(mod_key.get_folder_name()));
            }
            for (filename, definition) in mod_definition.definitions().iter() {
                if definition.is_data_loading() {
                    return Some(LoadingState::Data(filename.to_string()));
                }
                if definition.is_mesh_loading() {
                    return Some(LoadingState::Mesh(filename.to_string()));
                }
            }
        }

        None
    }

    pub fn check_update(&self, last_version: &mut Option<u32>) -> bool {
        self.mods.check_update(last_version)
    }
}
