use super::{ui_attribute_value, State};
use crate::sw_block_definition::{AttributeSpecifier, AttributeValue, SwBlockDefinition};
use egui::{CentralPanel, ScrollArea, TopBottomPanel};
use egui_extras::{Column, TableBuilder};
use std::collections::{BTreeMap, BTreeSet};

type DefinitionValuesItem<'a> = (usize, &'a SwBlockDefinition, AttributeValue);

#[derive(serde::Serialize, serde::Deserialize, PartialEq)]
enum AttributeDetailTabs {
    Definitions,
    Values,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AttributeDetailWindow {
    open: bool,
    id: Option<egui::Id>,
    specifier: AttributeSpecifier,
    tab: AttributeDetailTabs,
    hide_default_value: bool,
    #[serde(skip)]
    values_table_heights: Vec<f32>,
}

impl AttributeDetailWindow {
    pub fn new(specifier: AttributeSpecifier, hide_default_value: bool) -> Self {
        Self {
            open: true,
            id: None,
            specifier,
            tab: AttributeDetailTabs::Definitions,
            hide_default_value,
            values_table_heights: Vec::new(),
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
                .min_width(300.0)
                .open(&mut open)
                .show(ctx, |ui| {
                    TopBottomPanel::top(id.with("top_panel")).show_inside(ui, |ui| {
                        self.ui_top_panel(ui);
                        ui.add_space(4.0);
                    });

                    TopBottomPanel::bottom(id.with("bottom_panel"))
                        .show_separator_line(false)
                        .exact_height(0.0)
                        .frame(egui::Frame::NONE)
                        .show_inside(ui, |_| {});

                    CentralPanel::default().show_inside(ui, |ui| {
                        self.ui_central_panel(ui, state);
                    });
                });
            self.open = open;
        }
    }

    fn ui_top_panel(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.selectable_value(
                &mut self.tab,
                AttributeDetailTabs::Definitions,
                "Definition List",
            );
            ui.selectable_value(&mut self.tab, AttributeDetailTabs::Values, "Value List");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::LEFT), |ui| {
                ui.checkbox(&mut self.hide_default_value, "Hide default value");
            });
        });
    }

    fn ui_central_panel(&mut self, ui: &mut egui::Ui, state: &mut State) {
        ScrollArea::vertical().show(ui, |ui| {
            state.load_all_definitions();
            let mut definition_values = state.get_attribute_all_definitions(&self.specifier);
            if self.hide_default_value {
                definition_values.retain(|(_, _, v)| !v.is_default());
            }

            let mut selected_definition_index = *state.selected_definition_index();
            match self.tab {
                AttributeDetailTabs::Definitions => {
                    let map = definition_map(definition_values);
                    self.ui_definitions_table(ui, state, map, &mut selected_definition_index);
                }
                AttributeDetailTabs::Values => {
                    let map = value_map(definition_values);
                    self.ui_values_table(ui, state, map, &mut selected_definition_index)
                }
            }
            state.set_selected_definition_index(selected_definition_index);
        });
    }

    fn ui_definitions_table(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut State,
        definition_map: BTreeMap<usize, (String, BTreeSet<AttributeValue>)>,
        selected_definition_index: &mut Option<usize>,
    ) {
        TableBuilder::new(ui)
            .column(Column::exact(250.0))
            .column(Column::remainder())
            .striped(true)
            .body(|body| {
                body.rows(20.0, definition_map.len(), |mut row| {
                    if let Some((i, (filename, values))) = definition_map.iter().nth(row.index()) {
                        let i = *i;

                        let checked = Some(i) == *selected_definition_index;

                        row.col(|ui| {
                            let label =
                                ui.selectable_label(checked, filename.clone())
                                    .on_hover_ui(|ui| {
                                        ui.label(filename);
                                    });
                            if label.clicked() {
                                *selected_definition_index = Some(i);
                            }
                        });
                        row.col(|ui| {
                            ui.horizontal(|ui| {
                                for (i, value) in values.iter().enumerate() {
                                    if i != 0 {
                                        ui.add_space(8.0);
                                    }
                                    ui_attribute_value(
                                        ui,
                                        state,
                                        self.specifier.property(),
                                        Some(value),
                                    );
                                }
                            });
                        });
                    }
                });
            });
    }

    fn ui_values_table(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut State,
        value_map: BTreeMap<AttributeValue, BTreeMap<usize, String>>,
        selected_definition_index: &mut Option<usize>,
    ) {
        TableBuilder::new(ui)
            .column(Column::exact(250.0))
            .column(Column::remainder())
            .striped(true)
            .body(|body| {
                let keys: Vec<AttributeValue> = value_map.keys().cloned().collect();
                self.values_table_heights.resize(keys.len(), 20.0);

                body.heterogeneous_rows(
                    self.values_table_heights.clone().into_iter(),
                    |mut row| {
                        let row_index = row.index();
                        let key = &keys[row_index];
                        let definitions = value_map.get(key).unwrap();

                        row.col(|ui| {
                            let collapsing_response =
                                ui.collapsing(format!("{} definitions", definitions.len()), |ui| {
                                    for (i, filename) in definitions {
                                        let checked = Some(*i) == *selected_definition_index;
                                        if ui.selectable_label(checked, filename).clicked() {
                                            *selected_definition_index = Some(*i);
                                        }
                                    }
                                });

                            let mut rect = collapsing_response.header_response.rect;
                            if let Some(body_res) = collapsing_response.body_response {
                                rect = rect.union(body_res.rect);
                            }
                            if row_index >= self.values_table_heights.len() {
                                self.values_table_heights.resize(row_index, 20.0);
                            }
                            self.values_table_heights.insert(row_index, rect.height());
                        });

                        row.col(|ui| {
                            ui_attribute_value(ui, state, self.specifier.property(), Some(key));
                        });
                    },
                );
            });
    }
}

fn definition_map(
    definition_values: Vec<DefinitionValuesItem<'_>>,
) -> BTreeMap<usize, (String, BTreeSet<AttributeValue>)> {
    let mut map: BTreeMap<usize, (String, BTreeSet<AttributeValue>)> = BTreeMap::new();
    for (i, definition, value) in definition_values {
        if let Some(entry) = map.get_mut(&i) {
            entry.1.insert(value);
        } else {
            map.insert(i, (definition.filename(), BTreeSet::from([value])));
        }
    }
    map
}

fn value_map(
    definition_values: Vec<DefinitionValuesItem<'_>>,
) -> BTreeMap<AttributeValue, BTreeMap<usize, String>> {
    let mut map: BTreeMap<AttributeValue, BTreeMap<usize, String>> = BTreeMap::new();
    for (i, definition, value) in definition_values {
        if let Some(entries) = map.get_mut(&value) {
            entries.insert(i, definition.filename());
        } else {
            let mut entries = BTreeMap::new();
            entries.insert(i, definition.filename());
            map.insert(value, entries);
        }
    }
    map
}
