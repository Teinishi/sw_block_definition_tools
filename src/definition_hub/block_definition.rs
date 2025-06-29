use super::{LazyMeshes, LazyXml, ModKey};
use crate::{state::State, sw_block_definition::Definition, sw_gl_3d::SwBlockMeshes};
use core::fmt;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

type LazyDataContent = Arc<Result<Definition, String>>;
type LazyMeshesContent = Arc<SwBlockMeshes>;

pub struct BlockDefinition {
    mod_key: ModKey,
    path: PathBuf,
    filename: String,
    data: LazyXml<Definition>,
    meshes: LazyMeshes,
}

impl fmt::Debug for BlockDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BlockDefinition {{ mod_key: {:?}, path: {:?}, filename: {:?}, data: {:?}, meshes: ... }}",
            self.mod_key, self.path, self.filename, self.data
        )
    }
}

impl Clone for BlockDefinition {
    fn clone(&self) -> Self {
        Self {
            mod_key: self.mod_key.clone(),
            path: self.path.clone(),
            filename: self.filename.clone(),
            data: self.data.clone(),
            meshes: self.meshes.clone(),
        }
    }
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
            meshes: Default::default(),
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

    pub fn is_data_loading(&self) -> bool {
        self.data.is_loading()
    }

    pub fn is_mesh_loading(&self) -> bool {
        self.meshes.is_loading()
    }

    pub fn is_data_ready(&self) -> bool {
        self.data.is_ready()
    }

    pub fn is_mesh_ready(&self) -> bool {
        self.meshes.is_ready()
    }

    pub fn load_data(&self) -> Option<LazyDataContent> {
        self.data.try_get()
    }

    pub fn use_data<R>(&self, f: impl FnOnce(&Definition) -> R) -> Option<R> {
        if let Some(data) = self.load_data() {
            if let Ok(data) = data.as_ref() {
                return Some(f(data));
            }
        }
        None
    }

    pub fn load_meshes(&self, state: &State) -> Option<LazyMeshesContent> {
        if !self.meshes.has_loader() {
            self.use_data(|data| {
                let path = self.mod_key.get_path(state).clone();
                self.meshes.set_loader(data, &path);
            });
        }
        self.meshes.try_get()
    }

    pub fn load_data_meshes(
        &mut self,
        state: &State,
    ) -> Option<(LazyDataContent, LazyMeshesContent)> {
        self.load_data().zip(self.load_meshes(state))
    }

    pub fn refresh(&self) {
        self.data.refresh()
    }
}
