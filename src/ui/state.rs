use crate::sw_block_definition::SwBlockDefinition;
use std::{fs, io, path::Path};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct State {
    pub definitions: Vec<SwBlockDefinition>,
    pub selected_definition_index: Option<usize>,
    pub show_mesh_data: bool,
    pub show_mesh_0: bool,
    pub show_mesh_1: bool,
    pub show_mesh_2: bool,
    pub show_mesh_editor_only: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            definitions: Vec::new(),
            selected_definition_index: None,
            show_mesh_data: true,
            show_mesh_0: true,
            show_mesh_1: true,
            show_mesh_2: true,
            show_mesh_editor_only: true,
        }
    }
}

impl State {
    pub fn selected_definition(&mut self) -> Option<&mut SwBlockDefinition> {
        self.definitions.get_mut(self.selected_definition_index?)
    }

    pub fn open_rom_directory<P: AsRef<Path>>(&mut self, rom_path: P) -> io::Result<()> {
        // ディレクトリ内の .xml ファイルを列挙
        match fs::read_dir(rom_path.as_ref().join("data\\definitions")) {
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
                Ok(())
            }
            Err(err) => {
                self.definitions = Vec::new();
                self.selected_definition_index = None;
                Err(err)
            }
        }
    }
}
