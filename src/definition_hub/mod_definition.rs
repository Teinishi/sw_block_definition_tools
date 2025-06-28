use super::BlockDefinition;
use crate::{definition_hub::ModKey, lazy_load::LazyXml, sw_schema_lib::Mod};
use std::{
    collections::BTreeMap,
    fs::read_dir,
    io,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct ModDefinition {
    mod_key: ModKey,
    path: PathBuf,
    pub manifest: LazyXml<Mod>,
    pub definitions: BTreeMap<String, BlockDefinition>,
}

impl ModDefinition {
    pub fn new<P: AsRef<Path>>(mod_key: ModKey, path: P) -> Self {
        let pathbuf = path.as_ref().to_path_buf();
        let manifest = LazyXml::new(pathbuf.join("mod.xml"), "mod".to_string());
        manifest.get();
        let mut s = Self {
            mod_key,
            path: pathbuf,
            manifest,
            definitions: Default::default(),
        };
        let _ = s.scan_definitions();
        s
    }

    pub fn mod_key(&self) -> &ModKey {
        &self.mod_key
    }

    pub fn load_all(&self) -> usize {
        0
    }

    fn scan_definitions(&mut self) -> io::Result<()> {
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

        Ok(())
    }
}
