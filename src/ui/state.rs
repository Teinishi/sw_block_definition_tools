use crate::sw_block_definition::{
    AttributeSpecifier, AttributeValue, GetAttributeValueRoot, SwBlockDefinition,
    SwBlockDefinitionMeshes,
};
use paste::paste;
use std::{
    fs, io,
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
    definitions: Vec<SwBlockDefinition>,
    selected_definition_index: Option<usize>,
    selected_definition_changed: bool,
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
            definitions: Vec::new(),
            selected_definition_index: None,
            selected_definition_changed: false,
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
        self.selected_definition_changed = false;
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

    pub fn definitions(&self) -> &Vec<SwBlockDefinition> {
        &self.definitions
    }

    pub fn selected_definition_index(&self) -> &Option<usize> {
        &self.selected_definition_index
    }

    pub fn set_selected_definition_index(&mut self, value: Option<usize>) {
        if self.selected_definition_index != value {
            self.selected_definition_changed = true;
            self.selected_definition_index = value;
        }
    }

    pub fn selected_definition_changed(&self) -> bool {
        self.selected_definition_changed
    }

    pub fn selected_definition(&mut self) -> Option<&mut SwBlockDefinition> {
        self.definitions.get_mut(self.selected_definition_index?)
    }

    pub fn selected_meshes(&mut self) -> Option<std::rc::Rc<SwBlockDefinitionMeshes>> {
        self.definitions
            .get_mut(self.selected_definition_index?)?
            .load_meshes()
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

    pub fn open_rom_directory(&mut self, rom_path: PathBuf) -> io::Result<()> {
        // ディレクトリ内の .xml ファイルを列挙
        match fs::read_dir(rom_path.join("data\\definitions")) {
            Ok(dir) => {
                self.definitions = dir
                    .filter_map(|entry| {
                        if entry.is_err() {
                            return None;
                        }
                        let entry_path = entry.unwrap().path();
                        if entry_path.is_file() && entry_path.extension()? == "xml" {
                            return SwBlockDefinition::new(&rom_path, entry_path);
                        }
                        None
                    })
                    .collect();
                self.selected_definition_index = None;
                self.rom_path = Some(rom_path);
                Ok(())
            }
            Err(err) => {
                self.definitions = Vec::new();
                self.selected_definition_index = None;
                self.rom_path = None;
                Err(err)
            }
        }
    }

    pub fn reset_search(&mut self) {
        for definition in &mut self.definitions {
            definition.clear_search();
        }
    }

    pub fn search(&mut self, search_text: &str) {
        if search_text.is_empty() {
            self.reset_search();
        } else {
            for definition in &mut self.definitions {
                definition.search(search_text);
            }
        }
    }
}

getter_setter!(State, show_all, bool);
getter_setter!(State, hide_default, bool);
getter_setter!(State, audio_volume, f32);

impl State {
    pub fn load_all_definitions(&mut self) -> i32 {
        let mut loading_count = 0;
        for definition in &mut self.definitions {
            if definition.load_data().is_none() {
                loading_count += 1;
            }
        }
        loading_count
    }

    pub fn get_attribute_all_definitions(
        &self,
        specifier: &AttributeSpecifier,
    ) -> Vec<(usize, &SwBlockDefinition, AttributeValue)> {
        let mut values: Vec<(usize, &SwBlockDefinition, AttributeValue)> = Vec::new();
        for (i, definition) in self.definitions.iter().enumerate() {
            if let Some(Ok(data)) = definition.data() {
                for value in specifier.get_value_root(&data) {
                    values.push((i, definition, value));
                }
            }
        }
        values
    }
}
