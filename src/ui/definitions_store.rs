use crate::sw_block_definition::{
    AttributeSpecifier, AttributeValue, GetAttributeValueRoot, IsDefault, SwBlockDefinition,
};
use std::collections::BTreeSet;
use std::io;
use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    path::PathBuf,
    rc::{Rc, Weak},
};

type DefinitionPointer = Rc<RefCell<SwBlockDefinition>>;
pub type WeakDefinitionPointer = Weak<RefCell<SwBlockDefinition>>;
type DefinitionsMap = Rc<RefCell<BTreeMap<String, DefinitionPointer>>>;

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
                    definitions.insert(def.filename().to_string(), Rc::new(RefCell::new(def)));
                }
            }
        }
        self.rom_path = Some(rom_path);
        Ok(())
    }

    pub fn load_all_definitions(&mut self) -> i32 {
        let mut loading_count = 0;
        for definition in self.definitions.borrow().values() {
            if definition.borrow_mut().load_data().is_none() {
                loading_count += 1;
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
            if let Some(Ok(data)) = definition.borrow().data() {
                for value in specifier.get_value_root(&data) {
                    if !hide_defalt || !value.is_default() {
                        values.push((filename.clone(), Rc::downgrade(definition), value));
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

pub trait DefinitionSelect {
    fn is_selected(&self, definition: &DefinitionPointer) -> bool {
        self.is_selected_weak(&Rc::downgrade(definition))
    }
    fn is_selected_weak(&self, definition: &WeakDefinitionPointer) -> bool;
    fn clear(&mut self);
    fn select(&mut self, definition: &DefinitionPointer) {
        self.select_weak(&Rc::downgrade(definition));
    }
    fn select_weak(&mut self, definition: &WeakDefinitionPointer);
    #[allow(dead_code)]
    fn unselect(&mut self, definition: &DefinitionPointer);
    #[allow(dead_code)]
    fn toggle_select(&mut self, definition: &DefinitionPointer) {
        if self.is_selected(definition) {
            self.select(definition);
        } else {
            self.unselect(definition);
        }
    }
    fn register_tracker(&mut self) -> u32;
    fn check_update(&mut self, tracker_id: u32) -> Option<bool>;
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct DefinitionSingleSelect {
    selected: Option<WeakDefinitionPointer>,
    change_tracker: ChangeTracker,
}

impl DefinitionSingleSelect {
    pub fn selected(&self) -> Option<Rc<RefCell<SwBlockDefinition>>> {
        let selected = self.selected.as_ref()?;
        selected.upgrade()
    }
}

impl DefinitionSelect for DefinitionSingleSelect {
    fn is_selected_weak(&self, definition: &WeakDefinitionPointer) -> bool {
        self.selected
            .as_ref()
            .map(|s| s.ptr_eq(definition))
            .unwrap_or(false)
    }

    fn clear(&mut self) {
        if self.selected.is_some() {
            self.selected = None;
            self.change_tracker.changed();
        }
    }

    fn select_weak(&mut self, definition: &WeakDefinitionPointer) {
        if !self.is_selected_weak(definition) {
            self.selected = Some(definition.clone());
            self.change_tracker.changed();
        }
    }

    fn unselect(&mut self, definition: &DefinitionPointer) {
        if self.is_selected(definition) {
            self.clear();
        }
    }

    fn register_tracker(&mut self) -> u32 {
        self.change_tracker.register()
    }

    fn check_update(&mut self, tracker_id: u32) -> Option<bool> {
        self.change_tracker.check(tracker_id)
    }
}

// 複数箇所から毎フレーム変更を追跡できるようにする
#[derive(serde::Serialize, serde::Deserialize, Default)]
struct ChangeTracker {
    tracker_id: u32,
    trackers: HashMap<u32, bool>,
}

impl ChangeTracker {
    // 追跡者にIDを配る
    fn register(&mut self) -> u32 {
        let tracker_id = self.tracker_id;
        self.tracker_id += 1;
        tracker_id
    }

    // 変更をチェックされたら返してフラグを下ろす
    fn check(&mut self, tracker_id: u32) -> Option<bool> {
        if let Some(changed) = self.trackers.get(&tracker_id).cloned() {
            self.trackers.insert(tracker_id, false);
            Some(changed)
        } else {
            None
        }
    }

    // 変更があったときすべてフラグ立てる
    fn changed(&mut self) {
        for key in 0..self.tracker_id {
            self.trackers.insert(key, true);
        }
    }
}
