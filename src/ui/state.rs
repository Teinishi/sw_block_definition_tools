use crate::sw_block_definition::SwBlockDefinition;
use std::{fs, io, path::Path};

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct State {
    definitions: Vec<SwBlockDefinition>,
    selected_definition_index: Option<usize>,
}

impl State {
    pub fn definitions(&self) -> &Vec<SwBlockDefinition> {
        &self.definitions
    }

    pub fn selected_definition_index(&self) -> Option<usize> {
        self.selected_definition_index
    }

    pub fn selected_definition(&mut self) -> Option<&mut SwBlockDefinition> {
        self.definitions.get_mut(self.selected_definition_index?)
    }

    pub fn set_selected_definition_index(&mut self, index: Option<usize>) {
        self.selected_definition_index = index;
    }

    pub fn open_directory<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        // ディレクトリ内の .xml ファイルを列挙
        self.definitions = fs::read_dir(path)?
            .filter_map(|entry| {
                if entry.is_err() {
                    return None;
                }
                let entry_path = entry.unwrap().path();
                if entry_path.is_file() && entry_path.extension()? == "xml" {
                    return SwBlockDefinition::new(entry_path);
                }
                None
            })
            .collect();
        self.selected_definition_index = None;
        Ok(())
    }
}
