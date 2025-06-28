use super::LoadingState;
use crate::{
    store::{
        mod_definition::{DefinitionPointer, DefinitionsMap, WeakDefinitionPointer},
        mod_store::ModStore,
        ModKey, SwModDefinition,
    },
    sw_block_definition::{AttributeSpecifier, AttributeValue, GetAttributeValueRoot, IsDefault},
};
use std::fs::read_dir;
use std::{
    collections::{BTreeMap, BTreeSet},
    io,
    path::PathBuf,
    sync::Arc,
};

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct DefinitionsStore {
    rom_path: Option<PathBuf>,
    mods_path: Option<PathBuf>,
    workshop_path: Option<PathBuf>,
    #[serde(skip)]
    mods: ModStore,
}

impl DefinitionsStore {
    pub fn rom_path(&self) -> &Option<PathBuf> {
        &self.rom_path
    }

    pub fn mods(&self) -> &ModStore {
        &self.mods
    }

    pub fn get(&self, mod_key: ModKey, name: &str) -> Option<DefinitionPointer> {
        self.mods
            .mods_map
            .borrow()
            .get(&mod_key)?
            .lock()
            .ok()?
            .get_definition(name)
    }

    pub fn loading_state(&self) -> Option<LoadingState> {
        /*for (filename, definition) in self.definitions.borrow().iter() {
            if let Ok(definition) = definition.lock() {
                if definition.is_loading_data() {
                    return Some(LoadingState::Data(filename.clone()));
                } else if definition.is_lodading_meshes() {
                    return Some(LoadingState::Mesh(filename.clone()));
                }
            }
        }*/
        // TODO: modのロード判定
        None
    }

    pub fn open_rom_directory(&mut self, _pathbuf: Option<PathBuf>) -> io::Result<()> {
        /*let pathbuf = pathbuf.or_else(|| self.rom_path.clone());
        if pathbuf.is_none() {
            return Ok(());
        }
        let pathbuf = pathbuf.unwrap();

        let mut definitions = self.definitions.borrow_mut();
        definitions.clear();
        self.rom_path = None;
        // ディレクトリ内の .xml ファイルを列挙
        let dir = read_dir(pathbuf.join("data").join("definitions"))?;
        for entry in dir {
            if let Some(entry_path) = entry
                .ok()
                .map(|e| e.path())
                .filter(|e| e.is_file() && e.extension().is_some_and(|x| x == "xml"))
            {
                if let Some(def) = SwBlockDefinition::new(entry_path) {
                    definitions.insert(def.filename().to_string(), Arc::new(Mutex::new(def)));
                }
            }
        }
        self.rom_path = Some(pathbuf);*/
        Ok(())
    }

    pub fn open_mods_directory(&mut self, pathbuf: Option<PathBuf>) -> io::Result<()> {
        let pathbuf = pathbuf.or_else(|| self.mods_path.clone());
        if pathbuf.is_none() {
            return Ok(());
        }
        let pathbuf = pathbuf.unwrap();

        self.mods.clear_local_mods();

        self.mods_path = None;
        for entry in read_dir(&pathbuf)? {
            if let Some(entry_path) = entry.ok().map(|e| e.path()).filter(|e| e.is_dir()) {
                if let Some(mod_definition) = SwModDefinition::new(entry_path) {
                    self.mods.add_local_mod(mod_definition);
                }
            }
        }
        self.mods_path = Some(pathbuf);

        Ok(())
    }

    pub fn open_workshop_directory(&mut self, pathbuf: Option<PathBuf>) -> io::Result<()> {
        let pathbuf = pathbuf.or_else(|| self.workshop_path.clone());
        if pathbuf.is_none() {
            return Ok(());
        }
        let _pathbuf = pathbuf.unwrap();

        Ok(())
    }

    pub fn load_all_definitions(&mut self) -> i32 {
        /*let mut loading_count = 0;
        for definition in self.definitions.borrow().values() {
            if let Ok(mut definition) = definition.lock() {
                if definition.load_data().is_none() {
                    loading_count += 1;
                }
            }
        }
        loading_count*/
        0
    }

    /*pub fn get_attribute_from_all(
        &self,
        specifier: &AttributeSpecifier,
        hide_default: bool,
    ) -> AttributeValueContainer {
        AttributeValueContainer::new(&self.definitions, specifier, hide_default)
    }*/
}

pub type AttributeDefinitionMap =
    BTreeMap<String, (WeakDefinitionPointer, BTreeSet<AttributeValue>)>;
pub type AttributeValueMap = BTreeMap<AttributeValue, BTreeMap<String, WeakDefinitionPointer>>;

pub struct AttributeValueContainer {
    values: Vec<(String, WeakDefinitionPointer, AttributeValue)>,
}

impl AttributeValueContainer {
    pub fn new(
        definitions: &DefinitionsMap,
        specifier: &AttributeSpecifier,
        hide_defalt: bool,
    ) -> Self {
        let mut values = Vec::new();

        for (filename, definition) in definitions.borrow().iter() {
            if let Some(Ok(data)) = definition.lock().ok().and_then(|d| d.data()) {
                for value in specifier.get_value_root(data.as_ref()) {
                    if !hide_defalt || !value.is_default() {
                        values.push((filename.clone(), Arc::downgrade(definition), value));
                    }
                }
            }
        }

        Self { values }
    }

    // ファイル名と値の集合のペア
    pub fn definition_map(&self) -> AttributeDefinitionMap {
        let mut map: AttributeDefinitionMap = BTreeMap::new();
        for (filename, definition, value) in &self.values {
            if let Some(entry) = map.get_mut(filename) {
                entry.1.insert(value.clone());
            } else {
                map.insert(
                    filename.clone(),
                    (definition.clone(), BTreeSet::from([value.clone()])),
                );
            }
        }
        map
    }

    // 値とファイル名の集合のペア
    pub fn value_map(&self) -> AttributeValueMap {
        let mut map: AttributeValueMap = BTreeMap::new();
        for (filename, definition, value) in &self.values {
            if let Some(entries) = map.get_mut(value) {
                entries.insert(filename.clone(), definition.clone());
            } else {
                let mut entries = BTreeMap::new();
                entries.insert(filename.clone(), definition.clone());
                map.insert(value.clone(), entries);
            }
        }
        map
    }
}
