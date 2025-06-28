use std::{
    fmt,
    sync::{Arc, Mutex},
    thread,
};

enum LoadState<T> {
    NotStarted,
    Loading,
    Ready(Result<Arc<T>, String>),
}

impl<T> fmt::Debug for LoadState<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotStarted => write!(f, "LoadState::NotStarted"),
            Self::Loading => write!(f, "LoadState::Loading"),
            Self::Ready(Ok(_)) => write!(f, "LoadState::Ready(Ok)"),
            Self::Ready(Err(mes)) => write!(f, "LoadState::Ready(Err({}))", mes),
        }
    }
}

type LazyLoaderFn<T> = dyn Fn() -> Result<T, String> + Send + Sync + 'static;

pub struct LazyLoad<T> {
    loader: Arc<LazyLoaderFn<T>>,
    state: Arc<Mutex<LoadState<T>>>,
}

impl<T> fmt::Debug for LazyLoad<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LazyLoad {{ state: {:?} }}", self.state)
    }
}

impl<T> Clone for LazyLoad<T> {
    fn clone(&self) -> Self {
        Self {
            loader: self.loader.clone(),
            state: Arc::clone(&self.state),
        }
    }
}

impl<T> LazyLoad<T>
where
    T: Send + Sync + 'static,
{
    pub fn new<F>(loader: F) -> Self
    where
        F: Fn() -> Result<T, String> + Send + Sync + 'static,
    {
        Self {
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
                let loader = Arc::clone(&self.loader);
                let state_clone = Arc::clone(&self.state);

                thread::spawn(move || {
                    let result = loader().map(Arc::new);
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
