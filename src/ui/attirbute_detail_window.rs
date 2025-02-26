use super::State;
use crate::sw_block_definition::DefinitionAttribute;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AttributeDetailWindow {
    open: bool,
    id: Option<egui::Id>,
    specifier: DefinitionAttribute,
}

impl AttributeDetailWindow {
    pub fn new(specifier: DefinitionAttribute) -> Self {
        Self {
            open: true,
            id: None,
            specifier,
        }
    }

    pub fn set_id(&mut self, id: egui::Id) {
        self.id = Some(id);
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn ui(&mut self, ctx: &egui::Context, state: &mut State) {
        if let Some(id) = self.id {
            let mut open = self.open;
            egui::Window::new(self.specifier.to_string())
                .id(id)
                .open(&mut open)
                .show(ctx, |ui| {
                    self.ui_content(ui, state);
                });
            self.open = open;
        }
    }

    fn ui_content(&mut self, _ui: &mut egui::Ui, state: &mut State) {
        state.get_attribute_all(&self.specifier);
    }
}
