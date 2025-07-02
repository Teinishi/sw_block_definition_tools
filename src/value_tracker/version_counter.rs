use std::sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
};

// 値の変更を追跡するためのバージョンカウンタ
#[derive(Debug, Clone)]
pub enum VersionCounter {
    Unset,
    Plain(Arc<AtomicU32>),
}

impl Default for VersionCounter {
    fn default() -> Self {
        Self::Unset
    }
}

impl VersionCounter {
    pub fn zero() -> Self {
        Self::Plain(Arc::new(AtomicU32::new(0)))
    }

    pub fn bump(&self) {
        if let Self::Plain(counter) = self {
            counter.fetch_add(1, Ordering::SeqCst);
        }
    }

    pub fn current(&self) -> Option<u32> {
        if let Self::Plain(counter) = self {
            Some(counter.load(Ordering::Relaxed))
        } else {
            None
        }
    }
}
