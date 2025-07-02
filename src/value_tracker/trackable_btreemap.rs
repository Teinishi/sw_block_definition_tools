use super::{AttachVersion, CheckUpdate, VersionCounter};
use std::collections::BTreeMap;

// 追跡可能な BTreeMap
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct TrackableBTreeMap<K: Ord, V> {
    inner: BTreeMap<K, V>,
    #[serde(skip)]
    version: VersionCounter,
}

impl<K: Ord, V> Default for TrackableBTreeMap<K, V> {
    fn default() -> Self {
        Self {
            inner: BTreeMap::new(),
            version: Default::default(),
        }
    }
}

impl<K: Ord, V: AttachVersion> AttachVersion for TrackableBTreeMap<K, V> {
    fn attach_version(&mut self, version: &VersionCounter) {
        self.version = version.clone();
        for value in self.inner.values_mut() {
            value.attach_version(version);
        }
    }
}

impl<K: Ord, V> CheckUpdate for TrackableBTreeMap<K, V> {
    fn check_update(&self, last_version: &mut Option<u32>) -> bool {
        self.version.check_update(last_version)
    }
}

impl<K: Ord, V: AttachVersion> TrackableBTreeMap<K, V> {
    pub fn clear(&mut self) {
        if !self.is_empty() {
            self.inner.clear();
            self.version.bump();
        }
    }

    pub fn insert(&mut self, key: K, mut value: V) {
        value.attach_version(&self.version);
        self.inner.insert(key, value);
        self.version.bump();
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.inner.get(key)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.inner.get_mut(key)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let result = self.inner.remove(key);
        if result.is_some() {
            self.version.bump();
        }
        result
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.inner.iter()
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.inner.keys()
    }

    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.inner.values()
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.inner.contains_key(key)
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}
