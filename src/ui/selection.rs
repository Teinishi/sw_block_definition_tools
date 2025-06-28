use std::{cell::RefCell, collections::HashSet, hash::Hash, rc::Rc};

pub trait SelectionMut<T> {
    fn add(&mut self, value: T);
    fn remove(&mut self, value: &T);
    fn clear(&mut self);
    fn is_empty(&self) -> bool;
    fn is_selected(&self, value: &T) -> bool;
}

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
    selection: Option<T>,
    version: u64,
}

impl<T> SingleSelection<T>
where
    T: PartialEq,
{
    pub fn get(&self) -> &Option<T> {
        &self.selection
    }
    pub fn version(&self) -> u64 {
        self.version
    }
}

impl<T> SelectionMut<T> for SingleSelection<T>
where
    T: PartialEq,
{
    fn add(&mut self, value: T) {
        if !self.is_selected(&value) {
            self.selection = Some(value);
            self.version += 1;
        }
    }
    fn remove(&mut self, value: &T) {
        if self.is_selected(value) {
            self.clear();
        }
    }
    fn clear(&mut self) {
        if !self.is_empty() {
            self.selection = None;
            self.version += 1;
        }
    }
    fn is_empty(&self) -> bool {
        self.selection.is_none()
    }
    fn is_selected(&self, value: &T) -> bool {
        if let Some(selection) = &self.selection {
            return selection == value;
        }
        false
    }
}

impl<T> Default for SingleSelection<T>
where
    T: PartialEq,
{
    fn default() -> Self {
        Self {
            selection: None,
            version: 0,
        }
    }
}

#[derive(Debug)]
pub struct MultipleSelection<T: Eq + Hash> {
    selection: HashSet<T>,
    version: u64,
}

impl<T> MultipleSelection<T>
where
    T: Eq + Hash,
{
    pub fn set_selection(&mut self, values: impl Iterator<Item = T>) {
        self.selection = values.collect();
        self.version += 1;
    }
    pub fn get(&self) -> &HashSet<T> {
        &self.selection
    }
    pub fn count(&self) -> usize {
        self.selection.len()
    }
    pub fn version(&self) -> u64 {
        self.version
    }
}

impl<T> SelectionMut<T> for MultipleSelection<T>
where
    T: Eq + Hash,
{
    fn add(&mut self, value: T) {
        if self.selection.insert(value) {
            self.version += 1;
        }
    }
    fn remove(&mut self, value: &T) {
        if self.selection.remove(value) {
            self.version += 1;
        }
    }
    fn clear(&mut self) {
        if !self.is_empty() {
            self.selection.clear();
            self.version += 1;
        }
    }
    fn is_empty(&self) -> bool {
        self.selection.is_empty()
    }
    fn is_selected(&self, value: &T) -> bool {
        self.selection.contains(value)
    }
}

impl<T> Default for MultipleSelection<T>
where
    T: Eq + Hash,
{
    fn default() -> Self {
        Self {
            selection: HashSet::new(),
            version: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SharedSingleSelection<T: PartialEq> {
    inner: Rc<RefCell<SingleSelection<T>>>,
    last_version: u64,
}

impl<T> SharedSingleSelection<T>
where
    T: PartialEq,
{
    pub fn check_update(&mut self) -> bool {
        let version = self.inner.borrow().version();
        if version != self.last_version {
            self.last_version = version;
            true
        } else {
            false
        }
    }
    pub fn get(&self) -> Option<T>
    where
        T: Clone,
    {
        self.inner.borrow().get().clone()
    }
}

impl<T> Selection<T> for SharedSingleSelection<T>
where
    T: PartialEq,
{
    fn add(&self, value: T) {
        self.inner.borrow_mut().add(value);
    }
    fn remove(&self, value: &T) {
        self.inner.borrow_mut().remove(value);
    }
    fn clear(&self) {
        self.inner.borrow_mut().clear();
    }
    fn is_empty(&self) -> bool {
        self.inner.borrow().is_empty()
    }
    fn is_selected(&self, value: &T) -> bool {
        self.inner.borrow().is_selected(value)
    }
}

impl<T> Default for SharedSingleSelection<T>
where
    T: PartialEq,
{
    fn default() -> Self {
        Self {
            inner: Default::default(),
            last_version: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SharedMultipleSelection<T: Eq + Hash> {
    inner: Rc<RefCell<MultipleSelection<T>>>,
    last_version: u64,
}

impl<T> SharedMultipleSelection<T>
where
    T: Eq + Hash,
{
    pub fn check_update(&mut self) -> bool {
        let version = self.inner.borrow().version();
        if version != self.last_version {
            self.last_version = version;
            true
        } else {
            false
        }
    }
    pub fn set_selection(&self, values: impl Iterator<Item = T>) {
        self.inner.borrow_mut().set_selection(values);
    }
    pub fn get(&self) -> HashSet<T>
    where
        T: Clone,
    {
        self.inner.borrow().get().clone()
    }
    pub fn count(&self) -> usize {
        self.inner.borrow().count()
    }
}

impl<T> Selection<T> for SharedMultipleSelection<T>
where
    T: Eq + Hash,
{
    fn add(&self, value: T) {
        self.inner.borrow_mut().add(value);
    }
    fn remove(&self, value: &T) {
        self.inner.borrow_mut().remove(value);
    }
    fn clear(&self) {
        self.inner.borrow_mut().clear();
    }
    fn is_empty(&self) -> bool {
        self.inner.borrow().is_empty()
    }
    fn is_selected(&self, value: &T) -> bool {
        self.inner.borrow().is_selected(value)
    }
}

impl<T> Default for SharedMultipleSelection<T>
where
    T: Eq + Hash,
{
    fn default() -> Self {
        Self {
            inner: Default::default(),
            last_version: 0,
        }
    }
}
