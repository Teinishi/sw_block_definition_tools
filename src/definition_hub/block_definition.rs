use crate::{
    definition_hub::ModKey, lazy_load::LazyXml, sw_block_definition::Definition,
    sw_gl_3d::SwBlockMeshes,
};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Debug)]
pub struct BlockDefinition {
    mod_key: ModKey,
    path: PathBuf,
    filename: String,
    data: LazyXml<Definition>,
}

impl BlockDefinition {
    pub fn new<P: AsRef<Path>>(mod_key: ModKey, path: P) -> Self {
        let pathbuf = path.as_ref().to_path_buf();
        let filename = pathbuf
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();
        let data = LazyXml::new(pathbuf.clone(), "definition".to_string());
        Self {
            mod_key,
            path: pathbuf,
            filename,
            data,
        }
    }

    pub fn mod_key(&self) -> &ModKey {
        &self.mod_key
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn key(&self) -> (ModKey, String) {
        (self.mod_key.clone(), self.filename().to_string())
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }

    pub fn load_data(&self) -> Option<Result<Arc<Definition>, String>> {
        self.data.get()
    }

    pub fn load_meshes(&self) -> Option<Arc<SwBlockMeshes>> {
        None
    }

    pub fn load_data_meshes(&self) -> (Option<Arc<Definition>>, Option<Arc<SwBlockMeshes>>) {
        (self.load_data().and_then(|d| d.ok()), self.load_meshes())
    }

    pub fn refresh(&self) {
        self.data.refresh()
    }
}

impl Clone for BlockDefinition {
    fn clone(&self) -> Self {
        Self {
            mod_key: self.mod_key.clone(),
            path: self.path.clone(),
            filename: self.filename.clone(),
            data: self.data.clone(),
        }
    }
}
