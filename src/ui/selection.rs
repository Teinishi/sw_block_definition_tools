use crate::value_tracker::{AttachVersion, CheckUpdate, TrackableHashSet, VersionCounter};
use std::{cell::RefCell, collections::HashSet, hash::Hash, rc::Rc};

/*pub trait SelectionMut<T> {
    fn add(&mut self, value: T);
    fn remove(&mut self, value: &T);
    fn toggle(&mut self, value: T) {
        if self.is_selected(&value) {
            self.remove(&value);
        } else {
            self.add(value);
        }
    }
    fn clear(&mut self);
    fn is_empty(&self) -> bool;
    fn is_selected(&self, value: &T) -> bool;
}*/

pub trait Selection<T> {
    fn add(&self, value: T);
    fn remove(&self, value: &T);
    fn toggle(&self, value: T) {
        if self.is_selected(&value) {
            self.remove(&value);
        } else {
            self.add(value);
        }
    }
    fn clear(&self);
    fn is_empty(&self) -> bool;
    fn is_selected(&self, value: &T) -> bool;
}

#[derive(Debug)]
pub struct SingleSelection<T: PartialEq> {
    selection: Rc<RefCell<Option<T>>>,
    pub version: VersionCounter, //todo: pub外す
}

impl<T> Default for SingleSelection<T>
where
    T: PartialEq,
{
    fn default() -> Self {
        Self {
            selection: Rc::new(RefCell::new(None)),
            version: Default::default(),
        }
    }
}

impl<T: PartialEq> Clone for SingleSelection<T> {
    fn clone(&self) -> Self {
        Self {
            selection: self.selection.clone(),
            version: self.version.clone(),
        }
    }
}

impl<T: PartialEq> AttachVersion for SingleSelection<T> {
    fn attach_version(&mut self, version: &VersionCounter) {
        self.version = version.clone();
    }
}

impl<T: PartialEq> CheckUpdate for SingleSelection<T> {
    fn check_update(&self, last_version: &mut Option<u32>) -> bool {
        self.version.check_update(last_version)
    }
}

impl<T> SingleSelection<T>
where
    T: PartialEq,
{
    pub fn get(&self) -> &Rc<RefCell<Option<T>>> {
        &self.selection
    }
}

impl<T> Selection<T> for SingleSelection<T>
where
    T: PartialEq,
{
    fn add(&self, value: T) {
        if !self.is_selected(&value) {
            *self.selection.borrow_mut() = Some(value);
            self.version.bump();
        }
    }

    fn remove(&self, value: &T) {
        if self.is_selected(value) {
            self.clear();
        }
    }

    fn clear(&self) {
        if !self.is_empty() {
            *self.selection.borrow_mut() = None;
            self.version.bump();
        }
    }

    fn is_empty(&self) -> bool {
        self.selection.borrow().is_none()
    }

    fn is_selected(&self, value: &T) -> bool {
        match &*self.selection.borrow() {
            Some(s) => s == value,
            None => false,
        }
    }
}

#[derive(Debug)]
pub struct MultipleSelection<T: Eq + Hash> {
    selection: Rc<RefCell<TrackableHashSet<T>>>,
}

impl<T> Default for MultipleSelection<T>
where
    T: Eq + Hash,
{
    fn default() -> Self {
        Self {
            selection: Default::default(),
        }
    }
}

impl<T: Eq + Hash> AttachVersion for MultipleSelection<T> {
    fn attach_version(&mut self, version: &VersionCounter) {
        self.selection.borrow_mut().attach_version(version);
    }
}

impl<T> MultipleSelection<T>
where
    T: Eq + Hash,
{
    pub fn set_selection(&self, values: impl Iterator<Item = T>) {
        let mut selection = self.selection.borrow_mut();
        selection.clear();
        selection.extend(values);
    }

    pub fn inner_cloned(&self) -> HashSet<T>
    where
        T: Clone,
    {
        self.selection.borrow().iter().cloned().collect()
    }

    pub fn count(&self) -> usize {
        self.selection.borrow().len()
    }
}

impl<T> Selection<T> for MultipleSelection<T>
where
    T: Eq + Hash,
{
    fn add(&self, value: T) {
        self.selection.borrow_mut().insert(value);
    }

    fn remove(&self, value: &T) {
        self.selection.borrow_mut().remove(value);
    }

    fn clear(&self) {
        self.selection.borrow_mut().clear();
    }

    fn is_empty(&self) -> bool {
        self.selection.borrow().is_empty()
    }

    fn is_selected(&self, value: &T) -> bool {
        self.selection.borrow().contains(value)
    }
}
