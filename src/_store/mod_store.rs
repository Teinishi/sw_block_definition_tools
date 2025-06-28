use crate::store::SwModDefinition;
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    sync::{Arc, Mutex},
};

#[derive(serde::Serialize, serde::Deserialize, PartialOrd, Ord, PartialEq, Eq, Hash, Clone)]
pub enum ModKey {
    Stormworks,
    Local(String),
    Workshop(String),
}

pub type ModPointer = Arc<Mutex<SwModDefinition>>;
pub type ModsMap = Rc<RefCell<HashMap<ModKey, ModPointer>>>;

#[derive(Default)]
pub struct ModStore {
    pub mods_map: ModsMap,
}

impl ModStore {
    pub fn add_local_mod(&mut self, mod_definition: SwModDefinition) {
        self.mods_map.borrow_mut().insert(
            ModKey::Local(mod_definition.folder_name().to_string()),
            Arc::new(Mutex::new(mod_definition)),
        );
    }

    pub fn add_workshop_mod(&mut self, mod_definition: SwModDefinition) {
        self.mods_map.borrow_mut().insert(
            ModKey::Workshop(mod_definition.folder_name().to_string()),
            Arc::new(Mutex::new(mod_definition)),
        );
    }

    pub fn clear_local_mods(&mut self) {
        self.mods_map
            .borrow_mut()
            .retain(|key, _| !matches!(key, ModKey::Local(_)));
    }

    pub fn clear_workshop_mods(&mut self) {
        self.mods_map
            .borrow_mut()
            .retain(|key, _| !matches!(key, ModKey::Workshop(_)));
    }
}
