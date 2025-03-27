use super::DefinitionsStore;
use std::{
    path::PathBuf,
    sync::{mpsc, Arc},
};

type PlayingAudio = (String, Arc<rodio::Sink>, mpsc::Receiver<bool>);

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum LoadingState {
    Data(String),
    Mesh(String),
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct State {
    pub rom_path: Option<PathBuf>,
    pub show_all: bool,
    pub hide_default: bool,
    pub audio_volume: f32,
    loading: Option<LoadingState>,
    #[serde(skip)]
    playing_audio: Option<PlayingAudio>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            rom_path: None,
            show_all: false,
            hide_default: false,
            audio_volume: 0.5,
            loading: None,
            playing_audio: None,
        }
    }
}

impl State {
    pub fn start_frame(&mut self, definitions_store: &DefinitionsStore) {
        self.loading = definitions_store.loading_state();
    }

    pub fn end_frame(&mut self, ctx: &egui::Context) {
        // 描画フレームごとに1回呼ぶ
        if let Some((_, _, rx_done)) = &self.playing_audio {
            ctx.request_repaint();
            if rx_done.try_recv().is_ok() {
                self.set_playing_audio(None);
            }
        }
    }

    pub fn loading_state(&self) -> &Option<LoadingState> {
        &self.loading
    }

    pub fn playing_audio(&self) -> &Option<PlayingAudio> {
        &self.playing_audio
    }

    pub fn set_playing_audio(&mut self, value: Option<PlayingAudio>) {
        if let Some((_, sink, _)) = &self.playing_audio {
            sink.stop();
        }
        self.playing_audio = value;
    }
}
