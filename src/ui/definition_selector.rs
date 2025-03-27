use super::{definitions_store::DefinitionPointer, WeakDefinitionPointer};
use std::{collections::HashMap, sync::Arc};

pub trait DefinitionSelect {
    fn is_selected(&self, definition: &DefinitionPointer) -> bool {
        self.is_selected_weak(&Arc::downgrade(definition))
    }
    fn is_selected_weak(&self, definition: &WeakDefinitionPointer) -> bool;
    fn clear(&mut self);
    fn select(&mut self, definition: &DefinitionPointer) {
        self.select_weak(&Arc::downgrade(definition));
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
    fn register_observer(&mut self) -> u32;
    fn check_update(&mut self, observer_id: u32) -> Option<bool>;
}

#[derive(Default)]
pub struct DefinitionSingleSelect {
    selected: Option<WeakDefinitionPointer>,
    subject: ChangeObserverSubject,
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
            self.subject.changed();
        }
    }

    fn select_weak(&mut self, definition: &WeakDefinitionPointer) {
        if !self.is_selected_weak(definition) {
            self.selected = Some(definition.clone());
            self.subject.changed();
        }
    }

    fn unselect(&mut self, definition: &DefinitionPointer) {
        if self.is_selected(definition) {
            self.clear();
        }
    }

    fn register_observer(&mut self) -> u32 {
        self.subject.register()
    }

    fn check_update(&mut self, observer_id: u32) -> Option<bool> {
        self.subject.check(observer_id)
    }
}

impl DefinitionSingleSelect {
    pub fn selected(&self) -> Option<DefinitionPointer> {
        let selected = self.selected.as_ref()?;
        selected.upgrade()
    }
}

#[derive(Default)]
pub struct DefinitionMultiSelect {
    selected: Vec<WeakDefinitionPointer>,
    subject: ChangeObserverSubject,
}

impl DefinitionSelect for DefinitionMultiSelect {
    fn is_selected_weak(&self, definition: &WeakDefinitionPointer) -> bool {
        self.selected.iter().any(|s| s.ptr_eq(definition))
    }

    fn clear(&mut self) {
        self.selected.clear();
    }

    fn select_weak(&mut self, definition: &WeakDefinitionPointer) {
        if !self.is_selected_weak(definition) {
            self.selected.push(definition.clone());
        }
    }

    fn unselect(&mut self, definition: &DefinitionPointer) {
        let target = Arc::downgrade(definition);
        self.selected.retain(|s| !s.ptr_eq(&target));
    }

    fn register_observer(&mut self) -> u32 {
        self.subject.register()
    }

    fn check_update(&mut self, observer_id: u32) -> Option<bool> {
        self.subject.check(observer_id)
    }
}

impl DefinitionMultiSelect {
    pub fn count(&self) -> usize {
        self.selected.len()
    }

    pub fn selection_weak(&self) -> &Vec<WeakDefinitionPointer> {
        &self.selected
    }

    pub fn selection(&self) -> Vec<DefinitionPointer> {
        self.selected
            .iter()
            .filter_map(|ptr| ptr.upgrade())
            .collect()
    }

    pub fn set_selection_weak(&mut self, selection: impl Iterator<Item = WeakDefinitionPointer>) {
        self.selected = selection.collect();
    }

    pub fn set_selection<'a>(&mut self, selection: impl Iterator<Item = &'a DefinitionPointer>) {
        self.set_selection_weak(selection.map(Arc::downgrade));
    }
}

// 複数箇所から毎フレーム変更を追跡できるようにする
#[derive(serde::Serialize, serde::Deserialize, Default)]
struct ChangeObserverSubject {
    observer_id: u32,
    observers: HashMap<u32, bool>,
}

impl ChangeObserverSubject {
    // 追跡者にIDを配る
    fn register(&mut self) -> u32 {
        let observer_id = self.observer_id;
        self.observer_id += 1;
        observer_id
    }

    // 変更をチェックされたら返してフラグを下ろす
    fn check(&mut self, observer_id: u32) -> Option<bool> {
        if let Some(changed) = self.observers.get(&observer_id).cloned() {
            self.observers.insert(observer_id, false);
            Some(changed)
        } else {
            None
        }
    }

    // 変更があったときすべてフラグ立てる
    fn changed(&mut self) {
        for key in 0..self.observer_id {
            self.observers.insert(key, true);
        }
    }
}
