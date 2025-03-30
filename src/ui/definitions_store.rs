use super::LoadingState;
use crate::sw_block_definition::{
    AttributeSpecifier, AttributeValue, GetAttributeValueRoot, IsDefault, SwBlockDefinition,
};
use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet},
    io,
    path::PathBuf,
    rc::Rc,
    sync::{Arc, Mutex, Weak},
};

pub type DefinitionPointer = Arc<Mutex<SwBlockDefinition>>;
pub type WeakDefinitionPointer = Weak<Mutex<SwBlockDefinition>>;
pub type DefinitionsMap = Rc<RefCell<BTreeMap<String, DefinitionPointer>>>;

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct DefinitionsStore {
    rom_path: Option<PathBuf>,
    #[serde(skip)]
    definitions: DefinitionsMap,
}

impl DefinitionsStore {
    pub fn rom_path(&self) -> &Option<PathBuf> {
        &self.rom_path
    }

    pub fn definitions(&self) -> &DefinitionsMap {
        &self.definitions
    }

    pub fn loading_state(&self) -> Option<LoadingState> {
        for (filename, definition) in self.definitions.borrow().iter() {
            if let Ok(definition) = definition.lock() {
                if definition.loading_data() {
                    return Some(LoadingState::Data(filename.clone()));
                } else if definition.loading_mesh() {
                    return Some(LoadingState::Mesh(filename.clone()));
                }
            }
        }
        None
    }

    pub fn open_rom_directory(&mut self, rom_path: Option<PathBuf>) -> io::Result<()> {
        let rom_path = rom_path.or_else(|| self.rom_path.clone());
        if rom_path.is_none() {
            return Ok(());
        }
        let rom_path = rom_path.unwrap();

        let mut definitions = self.definitions.borrow_mut();
        definitions.clear();
        self.rom_path = None;
        // ディレクトリ内の .xml ファイルを列挙
        let dir = std::fs::read_dir(rom_path.join("data").join("definitions"))?;
        for entry in dir {
            if let Some(entry_path) = entry
                .ok()
                .map(|e| e.path())
                .filter(|e| e.is_file() && e.extension().is_some_and(|x| x == "xml"))
            {
                if let Some(def) = SwBlockDefinition::new(&rom_path, entry_path) {
                    definitions.insert(def.filename().to_string(), Arc::new(Mutex::new(def)));
                }
            }
        }
        self.rom_path = Some(rom_path);
        Ok(())
    }

    pub fn load_all_definitions(&mut self) -> i32 {
        let mut loading_count = 0;
        for definition in self.definitions.borrow().values() {
            if let Ok(mut definition) = definition.lock() {
                if definition.load_data().is_none() {
                    loading_count += 1;
                }
            }
        }
        loading_count
    }

    pub fn get_attribute_from_all(
        &self,
        specifier: &AttributeSpecifier,
        hide_default: bool,
    ) -> AttributeValueContainer {
        AttributeValueContainer::new(&self.definitions, specifier, hide_default)
    }
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
                for value in specifier.get_value_root(&data) {
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
