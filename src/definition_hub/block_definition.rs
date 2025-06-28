use super::LazyXml;
use crate::{
    definition_hub::ModKey, state::State, sw_block_definition::Definition, sw_gl_3d::SwBlockMeshes,
};
use core::fmt;
use std::{
    cell::RefCell,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};

pub struct BlockDefinition {
    mod_key: ModKey,
    path: PathBuf,
    filename: String,
    data: LazyXml<Definition>,
    meshes: Rc<RefCell<Option<SwBlockMeshes>>>,
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
            meshes: Rc::new(RefCell::new(None)),
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

    pub fn load_meshes(&self, state: &State) -> Rc<RefCell<Option<SwBlockMeshes>>> {
        if self.meshes.borrow().is_none() {
            if let Some(Ok(data)) = self.load_data() {
                let path = self.mod_key.get_path(state).clone();
                let meshes = SwBlockMeshes::new(&data, &move |name| {
                    <Option<PathBuf> as Clone>::clone(&path).map(|p| p.join(name))
                });
                *self.meshes.borrow_mut() = Some(meshes);
            }
        }
        self.meshes.clone()
    }

    pub fn load_data_meshes(
        &self,
        state: &State,
    ) -> (Option<Arc<Definition>>, Rc<RefCell<Option<SwBlockMeshes>>>) {
        (
            self.load_data().and_then(|d| d.ok()),
            self.load_meshes(state),
        )
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
            meshes: self.meshes.clone(),
        }
    }
}
