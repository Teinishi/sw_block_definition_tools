use super::State;

#[derive(Default)]
pub struct BottomPanel {}

impl BottomPanel {
    pub fn ui(&mut self, ui: &mut egui::Ui, state: &mut State) {
        let mut c = state.show_all_attributes();
        ui.checkbox(&mut c, "Show all attributes");
        state.set_show_all_sttributes(c);

        let mut c = state.hide_default_attributes();
        ui.checkbox(&mut c, "Hide default value attributes");
        state.set_hide_default_attributes(c);
    }
}
