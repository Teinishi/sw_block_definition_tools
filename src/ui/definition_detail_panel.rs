use super::State;

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct DefinitionDetailPanel {}

impl DefinitionDetailPanel {
    pub fn ui(&mut self, ui: &mut egui::Ui, state: &mut State) {
        let definition = state.selected_definition();
        if definition.is_none() {
            return;
        }
        let data = definition.unwrap().data();
        if data.is_err() {
            ui.collapsing("Error", |ui| {
                ui.label(data.unwrap_err().to_string());
            });
            return;
        }
        let data = data.unwrap();

        if let Some(name) = &data.name {
            ui.heading(name);
        }
    }
}
