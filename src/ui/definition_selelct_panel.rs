use crate::sw_block_definition::SwBlockDefinition;
use egui::Layout;
use std::{fs, io, path::Path, rc::Rc};

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct DefinitionSelectPanel {
    definitions: Vec<Rc<SwBlockDefinition>>,
    selected: Option<Rc<SwBlockDefinition>>,
}

impl DefinitionSelectPanel {
    pub fn set_directory<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        // ディレクトリ内の .xml ファイルを列挙
        self.definitions = fs::read_dir(path)?
            .filter_map(|entry| {
                if entry.is_err() {
                    return None;
                }
                let entry_path = entry.unwrap().path();
                if entry_path.is_file() && entry_path.extension()? == "xml" {
                    if let Some(d) = SwBlockDefinition::new(entry_path) {
                        return Some(Rc::new(d));
                    }
                }
                None
            })
            .collect();
        Ok(())
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(Layout::top_down_justified(egui::Align::LEFT), |ui| {
            ui.add_space(4.0);
            for entry in &self.definitions {
                let selected = self.selected == Some(entry.clone());
                if ui.selectable_label(selected, entry.filename()).clicked() {
                    self.selected = Some(entry.clone());
                }
            }
            ui.add_space(4.0);
        });
    }
}
