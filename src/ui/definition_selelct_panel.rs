use super::State;
use egui::Layout;

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct DefinitionSelectPanel {}

impl DefinitionSelectPanel {
    pub fn ui(&mut self, ui: &mut egui::Ui, state: &mut State) {
        let selected_index = state.selected_definition_index();
        let mut select = None;

        ui.with_layout(Layout::top_down_justified(egui::Align::LEFT), |ui| {
            ui.add_space(4.0);
            for (i, entry) in state.definitions().iter().enumerate() {
                if ui
                    .selectable_label(Some(i) == selected_index, entry.filename())
                    .clicked()
                {
                    select = Some(i);
                }
            }
            ui.add_space(4.0);
        });

        if select.is_some() {
            state.set_selected_definition_index(select);
        }
    }
}
