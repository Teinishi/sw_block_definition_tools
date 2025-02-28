use egui::ScrollArea;
use egui_extras::{Column, TableBuilder};

use super::State;
use crate::sw_block_definition::{
    DefinitionAttribute, DefinitionAttributeValue, SwBlockDefinition,
};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AttributeDetailWindow {
    open: bool,
    id: Option<egui::Id>,
    specifier: DefinitionAttribute,
    hide_default_value: bool,
}

impl AttributeDetailWindow {
    pub fn new(specifier: DefinitionAttribute) -> Self {
        Self {
            open: true,
            id: None,
            specifier,
            hide_default_value: false,
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
                .default_width(500.0)
                .open(&mut open)
                .show(ctx, |ui| {
                    egui::TopBottomPanel::bottom(id.with("bottom_panel")).show_inside(ui, |ui| {
                        ui.add_space(4.0);
                        self.ui_bottom_panel(ui);
                    });

                    egui::CentralPanel::default()
                        .frame(
                            egui::Frame::default().inner_margin(egui::Margin::symmetric(4, 0)),
                        )
                        .show_inside(ui, |ui| {
                            ScrollArea::vertical().show(ui, |ui| {
                                self.ui_all_definitions(ui, state);
                            });
                        });
                });
            self.open = open;
        }
    }

    fn ui_bottom_panel(&mut self, ui: &mut egui::Ui) {
        ui.checkbox(&mut self.hide_default_value, "Hide default value");
    }

    fn ui_all_definitions(&mut self, ui: &mut egui::Ui, state: &mut State) {
        state.load_all_definitions();
        let mut values = state.get_attribute_all(&self.specifier);

        if self.hide_default_value {
            values.retain(|(_, _, v)| !v.is_default);
        }

        let set_selected_definition =
            self.ui_all_definitions_table(ui, values, *state.selected_definition_index());
        if let Some(i) = set_selected_definition {
            state.set_selected_definition_index(Some(i));
        }
    }

    fn ui_all_definitions_table(
        &mut self,
        ui: &mut egui::Ui,
        values: Vec<(usize, &SwBlockDefinition, DefinitionAttributeValue)>,
        selected_definition_index: Option<usize>,
    ) -> Option<usize> {
        let mut set_selected = None;
        TableBuilder::new(ui)
            .column(Column::exact(250.0).resizable(true))
            .column(Column::remainder())
            .striped(true)
            .body(|body| {
                body.rows(20.0, values.len(), |mut row| {
                    let (i, definition, value) = &values[row.index()];
                    let i = *i;
                    let definition = *definition;

                    let filename = definition.filename();
                    let checked = Some(i) == selected_definition_index;

                    row.col(|ui| {
                        let label =
                            ui.selectable_label(checked, filename.clone())
                                .on_hover_ui(|ui| {
                                    ui.label(filename);
                                });
                        if label.clicked() {
                            set_selected = Some(i);
                        }
                    });
                    row.col(|ui| {
                        ui.label(value.debug_str.clone());
                    });
                });
            });
        set_selected
    }
}
