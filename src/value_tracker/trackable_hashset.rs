use super::{AttachVersion, CheckUpdate, VersionCounter};
use std::{collections::HashSet, hash::Hash};

// 追跡可能な HashMap
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct TrackableHashSet<T: Eq + Hash> {
    inner: HashSet<T>,
    #[serde(skip)]
    version: VersionCounter,
}

impl<T: Eq + Hash> Default for TrackableHashSet<T> {
    fn default() -> Self {
        Self {
            inner: HashSet::new(),
            version: Default::default(),
        }
    }
}

impl<T: Eq + Hash> AttachVersion for TrackableHashSet<T> {
    fn attach_version(&mut self, version: &VersionCounter) {
        self.version = version.clone();
    }
}

impl<T: Eq + Hash> CheckUpdate for TrackableHashSet<T> {
    fn check_update(&self, last_version: &mut Option<u32>) -> bool {
        self.version.check_update(last_version)
    }
}

impl<T: Eq + Hash> TrackableHashSet<T> {
    pub fn clear(&mut self) {
        if !self.is_empty() {
            self.inner.clear();
            self.version.bump();
        }
    }

    pub fn insert(&mut self, value: T) -> bool {
        if self.inner.insert(value) {
            self.version.bump();
            true
        } else {
            false
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        self.inner.contains(value)
    }

    pub fn remove(&mut self, value: &T) -> bool {
        let result = self.inner.remove(value);
        if result {
            self.version.bump();
        }
        result
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.inner.iter()
    }

    pub fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        self.inner.extend(iter);
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}
