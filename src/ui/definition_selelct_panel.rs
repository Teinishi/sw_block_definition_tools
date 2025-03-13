use super::State;
use egui::{Layout, TextEdit};

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct DefinitionSelectPanel {
    search_text: String,
}

impl DefinitionSelectPanel {
    pub fn ui(&mut self, ui: &mut egui::Ui, state: &mut State) {
        ui.add_space(6.0);
        let search_box = ui.add_sized(
            egui::vec2(ui.available_width(), 20.0),
            TextEdit::singleline(&mut self.search_text).hint_text("Search"),
        );
        if search_box.changed() {
            state.start_search(self.search_text.clone());
        }
        ui.add_space(6.0);
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.allocate_space(egui::vec2(ui.available_width(), 0.0));
            ui.with_layout(Layout::top_down_justified(egui::Align::LEFT), |ui| {
                let selected_index = *state.selected_definition_index();
                let mut set_index = None;

                for (i, entry) in state.definitions().iter().enumerate() {
                    if ui
                        .selectable_label(Some(i) == selected_index, entry.filename())
                        .clicked()
                    {
                        set_index = Some(i);
                    }
                }

                if let Some(value) = set_index {
                    state.set_selected_definition_index(Some(value));
                }
            });
            ui.add_space(4.0);
        });
    }
}
