use super::State;

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct BottomPanel {}

impl BottomPanel {
    pub fn ui(&mut self, ui: &mut egui::Ui, state: &mut State) {
        let mut c = state.show_all();
        ui.checkbox(&mut c, "Show all");
        state.set_show_all(c);

        let mut c = state.hide_default();
        ui.checkbox(&mut c, "Hide default value");
        state.set_hide_default(c);
    }
}
