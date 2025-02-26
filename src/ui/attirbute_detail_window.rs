use egui::ScrollArea;

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
                    ScrollArea::vertical().show(ui, |ui| {
                        self.ui_content(ui, state, id);
                    });
                });
            self.open = open;
        }
    }

    fn ui_content(&mut self, ui: &mut egui::Ui, state: &mut State, id: egui::Id) {
        egui::Grid::new(id.with("all_values"))
            .num_columns(2)
            .spacing([10.0, 4.0])
            .show(ui, |ui| {
                state.load_all_definitions();
                let values = state.get_attribute_all(&self.specifier);

                let mut set_selected = None;

                for (definition, value) in values {
                    let i = state.definition_index(definition);
                    if ui
                        .selectable_label(
                            i.is_some_and(|i| Some(i) == *state.selected_definition_index()),
                            definition.filename(),
                        )
                        .clicked()
                    {
                        set_selected = i;
                    }
                    ui.label(value.debug_str);
                    ui.end_row();
                }

                if let Some(i) = set_selected {
                    state.set_selected_definition_index(Some(i));
                }
            });
    }
}
