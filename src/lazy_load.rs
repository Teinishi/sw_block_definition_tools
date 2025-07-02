use super::value_tracker::{AttachVersion, VersionCounter};
use std::{
    fmt,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
};

struct LazyLoadInner<T> {
    content: Mutex<Option<Arc<T>>>,
    is_loading: AtomicBool,
    is_ready: AtomicBool,
    loader: Mutex<Option<Arc<dyn Fn() -> T + Send + Sync>>>,
}

impl<T> fmt::Debug for LazyLoadInner<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LazyLoadInner {{ content: ")?;
        if self.content.lock().unwrap().is_some() {
            write!(f, "Some(_)")?;
        } else {
            write!(f, "None")?;
        }
        write!(
            f,
            ", is_loading: {:?}, is_ready: {:?}, loader: ",
            self.is_loading, self.is_ready
        )?;
        if self.loader.lock().unwrap().is_some() {
            write!(f, "Some(_)")?;
        } else {
            write!(f, "None")?;
        }
        write!(f, " }}")
    }
}

impl<T> Default for LazyLoadInner<T> {
    fn default() -> Self {
        Self {
            content: Mutex::new(None),
            is_loading: AtomicBool::new(false),
            is_ready: AtomicBool::new(false),
            loader: Mutex::new(None),
        }
    }
}

#[derive(Debug)]
pub struct LazyLoad<T> {
    inner: Arc<LazyLoadInner<T>>,
    version: VersionCounter,
}

impl<T> Clone for LazyLoad<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            version: self.version.clone(),
        }
    }
}

impl<T> Default for LazyLoad<T> {
    fn default() -> Self {
        Self {
            inner: Arc::new(LazyLoadInner::default()),
            version: Default::default(),
        }
    }
}

impl<T> AttachVersion for LazyLoad<T> {
    fn attach_version(&mut self, version: &VersionCounter) {
        self.version = version.clone();
    }
}

impl<T> LazyLoad<T>
where
    T: Send + Sync + 'static,
{
    pub fn with_loader<F>(loader: F) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static,
    {
        let s = Self::default();
        s.attach_loader(loader);
        s
    }

    pub fn attach_loader<F>(&self, loader: F)
    where
        F: Fn() -> T + Send + Sync + 'static,
    {
        let mut guard = self.inner.loader.lock().unwrap();
        *guard = Some(Arc::new(loader));
    }

    pub fn get(&self) -> Option<Arc<T>> {
        let guard = self.inner.content.lock().unwrap();
        guard.as_ref().cloned()
    }

    pub fn try_load(&self) {
        if self.inner.is_ready.load(Ordering::SeqCst) {
            return;
        }
        if self.inner.is_loading.swap(true, Ordering::SeqCst) {
            return;
        }

        let state = Arc::clone(&self.inner);
        let version = self.version.clone();
        thread::spawn(move || {
            let loader = {
                let guard = state.loader.lock().unwrap();
                guard.clone()
            };
            if let Some(func) = loader {
                let result = Arc::new(func());
                let mut guard = state.content.lock().unwrap();
                *guard = Some(result);
                state.is_ready.store(true, Ordering::SeqCst);
            }
            state.is_loading.store(false, Ordering::SeqCst);
            version.bump();
        });
    }

    pub fn try_get(&self) -> Option<Arc<T>> {
        self.try_load();
        self.get()
    }

    pub fn refresh(&self) {
        self.inner.is_ready.store(false, Ordering::SeqCst);
        self.inner.is_loading.store(false, Ordering::SeqCst);
        let mut guard = self.inner.content.lock().unwrap();
        *guard = None;
        self.version.bump();
        self.try_load();
    }

    pub fn has_loader(&self) -> bool {
        self.inner.loader.lock().unwrap().is_some()
    }

    pub fn is_loading(&self) -> bool {
        self.inner.is_loading.load(Ordering::SeqCst)
    }

    pub fn is_ready(&self) -> bool {
        self.inner.is_ready.load(Ordering::SeqCst)
    }
}
