use crate::store::{DefinitionPointer, DefinitionsStore};
use egui::{vec2, Align, Button, Layout, TextEdit};
use std::collections::BTreeMap;

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct DefinitionSearch {
    search_text: String,
    search_result: BTreeMap<String, bool>,
}

impl DefinitionSearch {
    pub fn is_empty(&self) -> bool {
        self.search_text.is_empty()
    }

    fn clear(&mut self) {
        self.search_text.clear();
        self.search_result.clear();
    }

    pub fn update_search(&mut self, definitions_store: &mut DefinitionsStore) {
        for (filename, definition) in definitions_store.definitions().borrow().iter() {
            if self.search_result.contains_key(filename) {
                continue;
            }
            if let Some(result) = definition
                .lock()
                .ok()
                .and_then(|mut d| d.search(&self.search_text))
            {
                self.search_result.insert(filename.clone(), result);
            }
        }
    }

    pub fn get_result(
        &self,
        definitions_store: &mut DefinitionsStore,
    ) -> Vec<(String, DefinitionPointer)> {
        let binding = definitions_store.definitions().borrow();
        binding
            .iter()
            .filter_map(|(filename, ptr)| {
                if self.is_empty() || self.search_result.get(filename) == Some(&true) {
                    Some((filename.clone(), ptr.clone()))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.allocate_ui_with_layout(
            vec2(ui.available_width(), 20.0),
            Layout::right_to_left(Align::Center),
            |ui| {
                if ui
                    .add_sized(vec2(20.0, 20.0), Button::new("\u{274C}"))
                    .clicked()
                {
                    self.clear();
                }

                let search = ui.add_sized(
                    vec2(ui.available_width(), 20.0),
                    TextEdit::singleline(&mut self.search_text).hint_text("Search"),
                );
                if search.changed() {
                    self.search_result.clear();
                }
            },
        );
    }
}
