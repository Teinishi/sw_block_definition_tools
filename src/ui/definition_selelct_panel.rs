use super::State;
use egui::Layout;

#[derive(Default)]
pub struct DefinitionSelectPanel {}

impl DefinitionSelectPanel {
    pub fn ui(&mut self, ui: &mut egui::Ui, state: &mut State) {
        ui.with_layout(Layout::top_down_justified(egui::Align::LEFT), |ui| {
            for (i, entry) in state.definitions.iter().enumerate() {
                if ui
                    .selectable_label(Some(i) == state.selected_definition_index, entry.filename())
                    .clicked()
                {
                    state.selected_definition_index = Some(i);
                }
            }
        });
    }
}
