use std::{
    fmt,
    path::PathBuf,
    sync::{mpsc, Arc},
};

pub struct PlayingAudio {
    pub path: PathBuf,
    pub sink: Arc<rodio::Sink>,
    pub rx_done: mpsc::Receiver<bool>,
}

impl fmt::Debug for PlayingAudio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PlayingAudio {{ path: {:?} }}", self.path)
    }
}

impl PartialEq for PlayingAudio {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl PlayingAudio {
    pub fn new(path: PathBuf, sink: Arc<rodio::Sink>, rx_done: mpsc::Receiver<bool>) -> Self {
        Self {
            path,
            sink,
            rx_done,
        }
    }

    pub fn stop(&self) {
        self.sink.stop();
    }
}
