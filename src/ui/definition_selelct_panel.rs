use super::State;
use egui::Layout;

#[derive(Default)]
pub struct DefinitionSelectPanel {}

impl DefinitionSelectPanel {
    pub fn ui(&mut self, ui: &mut egui::Ui, state: &mut State) {
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
    }
}
