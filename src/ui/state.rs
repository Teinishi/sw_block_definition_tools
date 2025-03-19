use paste::paste;
use std::{
    path::PathBuf,
    sync::{mpsc, Arc},
};

macro_rules! getter_setter {
    ($target:ident, $name:ident, $type:ty, $change_fn:ident) => {
        impl $target {
            pub fn $name(&self) -> $type {
                self.$name
            }

            paste! {
                pub fn [<set_ $name>](&mut self, value: $type) {
                    if (self.$name != value) {
                        self.$name = value;
                        self.$change_fn();
                    }
                }
            }
        }
    };

    ($target:ident, $name:ident, $type:ty) => {
        impl $target {
            pub fn $name(&self) -> $type {
                self.$name
            }

            paste! {
                pub fn [<set_ $name>](&mut self, value: $type) {
                    if (self.$name != value) {
                        self.$name = value;
                    }
                }
            }
        }
    };
}

type PlayingAudio = (String, Arc<rodio::Sink>, mpsc::Receiver<bool>);

#[derive(serde::Deserialize, serde::Serialize)]
pub struct State {
    rom_path: Option<PathBuf>,
    show_all: bool,
    hide_default: bool,
    audio_volume: f32,
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
            playing_audio: None,
        }
    }
}

impl State {
    pub fn update(&mut self, ctx: &egui::Context) {
        // 描画フレームごとに1回呼ぶ
        if let Some((_, _, rx_done)) = &self.playing_audio {
            ctx.request_repaint();
            if rx_done.try_recv().is_ok() {
                self.set_playing_audio(None);
            }
        }
    }

    pub fn rom_path(&self) -> &Option<PathBuf> {
        &self.rom_path
    }

    pub fn set_rom_path(&mut self, rom_path: PathBuf) {
        self.rom_path = Some(rom_path);
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

getter_setter!(State, show_all, bool);
getter_setter!(State, hide_default, bool);
getter_setter!(State, audio_volume, f32);
