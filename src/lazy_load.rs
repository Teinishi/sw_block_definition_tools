use crate::utils::check_xml_root_tag;
use quick_xml::de::from_str;
use serde::de::DeserializeOwned;
use std::{
    fmt,
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
};

enum LoadState<T> {
    NotStarted,
    Loading,
    Ready(Result<Arc<T>, String>),
}

type LazyLoaderFn<T> = dyn Fn(&PathBuf) -> Result<T, String> + Send + Sync + 'static;

pub struct LazyLoad<T> {
    path: PathBuf,
    loader: Arc<LazyLoaderFn<T>>,
    state: Arc<Mutex<LoadState<T>>>,
}

impl<T> fmt::Debug for LazyLoad<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LazyLoad {{ path: {:?} }}", self.path)
    }
}

impl<T> Clone for LazyLoad<T> {
    fn clone(&self) -> Self {
        Self {
            path: self.path.clone(),
            loader: self.loader.clone(),
            state: Arc::clone(&self.state),
        }
    }
}

impl<T> LazyLoad<T>
where
    T: Send + Sync + 'static,
{
    pub fn new<F>(path: PathBuf, loader: F) -> Self
    where
        F: Fn(&PathBuf) -> Result<T, String> + Send + Sync + 'static,
    {
        Self {
            path,
            loader: Arc::new(loader),
            state: Arc::new(Mutex::new(LoadState::NotStarted)),
        }
    }

    pub fn get(&self) -> Option<Result<Arc<T>, String>> {
        let mut state = self.state.lock().unwrap();
        match &*state {
            LoadState::Ready(result) => Some(result.clone()),
            LoadState::Loading => None,
            LoadState::NotStarted => {
                let path = self.path.clone();
                let loader = Arc::clone(&self.loader);
                let state_clone = Arc::clone(&self.state);

                thread::spawn(move || {
                    let result = loader(&path).map(Arc::new);
                    let mut state = state_clone.lock().unwrap();
                    *state = LoadState::Ready(result);
                });

                *state = LoadState::Loading;
                None
            }
        }
    }

    pub fn refresh(&self) {
        let mut state = self.state.lock().unwrap();
        *state = LoadState::NotStarted;
        self.get();
    }
}

#[derive(Debug)]
pub struct LazyXml<T> {
    lazy_load: LazyLoad<T>,
}

impl<T> Clone for LazyXml<T> {
    fn clone(&self) -> Self {
        Self {
            lazy_load: self.lazy_load.clone(),
        }
    }
}

impl<T> LazyXml<T>
where
    T: DeserializeOwned + Send + Sync + 'static,
{
    pub fn new(path: PathBuf, root_tag: String) -> Self {
        Self {
            lazy_load: LazyLoad::new(path, move |path| load_xml_file(path, root_tag.as_bytes())),
        }
    }

    pub fn get(&self) -> Option<Result<Arc<T>, String>> {
        self.lazy_load.get()
    }

    pub fn refresh(&self) {
        self.lazy_load.refresh();
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
