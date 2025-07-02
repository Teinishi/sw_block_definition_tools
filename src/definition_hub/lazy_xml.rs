use crate::{
    lazy_load::LazyLoad,
    utils::check_xml_root_tag,
    value_tracker::{AttachVersion, VersionCounter},
};
use quick_xml::de::from_str;
use serde::de::DeserializeOwned;
use std::{path::PathBuf, sync::Arc};

#[derive(Debug)]
pub struct LazyXml<T> {
    inner: LazyLoad<Result<T, String>>,
}

impl<T> Clone for LazyXml<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> AttachVersion for LazyXml<T> {
    fn attach_version(&mut self, version: &VersionCounter) {
        self.inner.attach_version(version);
    }
}

impl<T> LazyXml<T>
where
    T: DeserializeOwned + Send + Sync + 'static,
{
    pub fn new(path: PathBuf, root_tag: String) -> Self {
        Self {
            inner: LazyLoad::with_loader(move || load_xml_file(&path, root_tag.as_bytes())),
        }
    }

    pub fn try_get(&self) -> Option<Arc<Result<T, String>>> {
        self.inner.try_get()
    }

    pub fn refresh(&self) {
        self.inner.refresh();
    }

    pub fn is_loading(&self) -> bool {
        self.inner.is_loading()
    }

    pub fn is_ready(&self) -> bool {
        self.inner.is_ready()
    }
}

fn load_xml_file<T>(path: &PathBuf, root_tag: &[u8]) -> Result<T, String>
where
    T: DeserializeOwned,
{
    let xml = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    check_xml_root_tag(&xml, root_tag)?;
    from_str::<T>(&xml).map_err(|e| e.to_string())
}
