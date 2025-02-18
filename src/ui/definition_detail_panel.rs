use super::State;

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct DefinitionDetailPanel {}

impl DefinitionDetailPanel {
    pub fn ui(&mut self, ui: &mut egui::Ui, state: &mut State) {
        let definition = state.selected_definition();
        if definition.is_none() {
            ui.label("No data");
            return;
        }
        let data = definition.unwrap().data();
        if data.is_err() {
            ui.label(data.unwrap_err().to_string());
            return;
        }
        let data = data.unwrap();

        ui.label(data.name.clone());
    }
}
