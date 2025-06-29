use crate::{
    lazy_load::LazyLoad,
    sw_block_definition::Definition,
    sw_gl_3d::{MeshConstructData, SwBlockMeshes},
};
use std::{path::PathBuf, sync::Arc};

#[derive(Debug, Clone, Default)]
pub struct LazyMeshes {
    inner: LazyLoad<SwBlockMeshes>,
}

impl LazyMeshes {
    pub fn set_loader(&self, data: &Definition, paths: Vec<PathBuf>) {
        let data = MeshConstructData::from_definition(data);
        self.inner.set_loader(move || {
            let value = paths.clone();
            SwBlockMeshes::new(&data, &move |name| {
                for p in &value {
                    let path = p.join(name);
                    if path.is_file() {
                        return Some(path);
                    }
                }
                None
            })
        });
    }

    pub fn try_get(&self) -> Option<Arc<SwBlockMeshes>> {
        self.inner.try_get()
    }

    pub fn has_loader(&self) -> bool {
        self.inner.has_loader()
    }

    pub fn is_loading(&self) -> bool {
        self.inner.is_loading()
    }

    pub fn is_ready(&self) -> bool {
        self.inner.is_ready()
    }
}
