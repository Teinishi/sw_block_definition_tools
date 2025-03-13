use super::State;
use egui::{vec2, Align, Button, Layout, TextEdit};

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct DefinitionSelectPanel {
    search_text: String,
}

impl DefinitionSelectPanel {
    pub fn ui(&mut self, ui: &mut egui::Ui, state: &mut State) {
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

                let label = ui.add_sized(
                    egui::vec2(ui.available_width(), 20.0),
                    TextEdit::singleline(&mut self.search_text).hint_text("Search"),
                );
                if label.changed() {
                    state.reset_search();
                }
                state.search(&self.search_text);
            },
        );

        ui.add_space(6.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.allocate_space(egui::vec2(ui.available_width(), 0.0));
            ui.with_layout(Layout::top_down_justified(egui::Align::LEFT), |ui| {
                let selected_index = *state.selected_definition_index();
                let mut set_index = None;

                for (i, entry) in state.definitions().iter().enumerate() {
                    if !self.search_text.is_empty() && entry.search_result() != Some(true) {
                        continue;
                    }
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
