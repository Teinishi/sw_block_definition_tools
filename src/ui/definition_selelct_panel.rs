use super::{definitions_store::DefinitionSelect, DefinitionSingleSelect, DefinitionsStore};
use crate::sw_block_definition::SwBlockDefinition;
use egui::{vec2, Align, Button, Layout, TextEdit};
use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct DefinitionSelectPanel {
    search_text: String,
    selector: DefinitionSingleSelect,
    search_result: BTreeMap<String, bool>,
}

impl DefinitionSelectPanel {
    pub fn ui(&mut self, ui: &mut egui::Ui, definitions_store: &mut DefinitionsStore) {
        ui.add_space(6.0);

        ui.allocate_ui_with_layout(
            egui::vec2(ui.available_width(), 20.0),
            Layout::right_to_left(Align::Center),
            |ui| {
                if ui
                    .add_sized(vec2(20.0, 20.0), Button::new("\u{274C}"))
                    .clicked()
                {
                    self.search_text.clear();
                }

                let search = ui.add_sized(
                    egui::vec2(ui.available_width(), 20.0),
                    TextEdit::singleline(&mut self.search_text).hint_text("Search"),
                );
                if search.changed() {
                    self.reset_search();
                }
            },
        );

        ui.add_space(6.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.allocate_space(egui::vec2(ui.available_width(), 0.0));
            ui.with_layout(Layout::top_down_justified(egui::Align::LEFT), |ui| {
                let no_search = self.search_text.is_empty();
                for (filename, definition) in definitions_store.definitions().borrow().iter() {
                    if !(no_search || self.search_result.get(filename) == Some(&true)) {
                        continue;
                    }
                    if ui
                        .selectable_label(self.selector.is_selected(definition), filename)
                        .clicked()
                    {
                        self.selector.select(definition);
                    }
                }
            });
            ui.add_space(4.0);
        });

        self.update_search(definitions_store);
    }

    pub fn selected_definition(&self) -> Option<Rc<RefCell<SwBlockDefinition>>> {
        self.selector.selected()
    }

    pub fn selector_mut(&mut self) -> &mut DefinitionSingleSelect {
        &mut self.selector
    }

    fn reset_search(&mut self) {
        self.search_result.clear();
    }

    fn update_search(&mut self, definitions_store: &mut DefinitionsStore) {
        for (filename, definition) in definitions_store.definitions().borrow().iter() {
            if self.search_result.contains_key(filename) {
                continue;
            }
            if let Some(result) = definition.borrow_mut().search(&self.search_text) {
                self.search_result.insert(filename.clone(), result);
            }
        }
    }
}
