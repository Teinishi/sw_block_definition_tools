use super::{BlockDefinition, LazyXml};
use crate::{
    definition_hub::ModKey,
    sw_schema_lib::Mod,
    value_tracker::{AttachVersion, TrackableBTreeMap, VersionCounter},
};
use std::{
    fs::read_dir,
    io,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct ModDefinition {
    mod_key: ModKey,
    path: PathBuf,
    manifest: LazyXml<Mod>,
    definitions: TrackableBTreeMap<String, BlockDefinition>,
    version: VersionCounter,
}

impl AttachVersion for ModDefinition {
    fn attach_version(&mut self, version: &VersionCounter) {
        self.version = version.clone();
    }
}

impl ModDefinition {
    pub fn new<P: AsRef<Path>>(mod_key: ModKey, path: P) -> Self {
        let pathbuf = path.as_ref().to_path_buf();
        let version = VersionCounter::default();
        let mut manifest = LazyXml::new(pathbuf.join("mod.xml"), "mod".to_string());
        manifest.attach_version(&version);
        manifest.try_get();
        let mut s = Self {
            mod_key,
            path: pathbuf,
            manifest,
            definitions: Default::default(),
            version,
        };
        let _ = s.scan_definitions();
        s
    }

    pub fn mod_key(&self) -> &ModKey {
        &self.mod_key
    }

    pub fn definitions(&self) -> &TrackableBTreeMap<String, BlockDefinition> {
        &self.definitions
    }

    pub fn refresh(&mut self) {
        self.manifest.refresh();
        let _ = self.scan_definitions();
        self.version.bump();
    }

    pub fn load_all(&self) -> usize {
        let mut count = 0;
        for definition in self.definitions.values() {
            if definition.load_data().is_none() {
                count += 1;
            }
        }
        count
    }

    pub fn use_manifest<R>(&self, f: impl FnOnce(&Mod) -> R) -> Option<R> {
        if let Some(data) = self.manifest.try_get() {
            if let Ok(data) = data.as_ref() {
                return Some(f(data));
            }
        }
        None
    }

    pub fn is_loading_manifest(&self) -> bool {
        self.manifest.is_loading()
    }

    fn scan_definitions(&mut self) -> io::Result<()> {
        self.definitions.clear();
        for entry in (read_dir(self.path.join("data\\definitions"))?).flatten() {
            let path = entry.path();
            if !path.is_file() || path.extension().is_none_or(|x| x != "xml") {
                continue;
            }
            if let Ok(filename) = entry.file_name().into_string() {
                self.definitions
                    .insert(filename, BlockDefinition::new(self.mod_key.clone(), path));
            }
        }
        self.version.bump();
        Ok(())
    }
}
