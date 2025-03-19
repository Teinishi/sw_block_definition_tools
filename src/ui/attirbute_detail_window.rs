use super::{
    ui_attribute_value, AttributeValueContainer, DefinitionSelect, DefinitionsStore, State,
    WeakDefinitionPointer,
};
use crate::sw_block_definition::{AttributeSpecifier, AttributeValue, GetAttributeValueRoot};
use egui::{CentralPanel, ScrollArea, TopBottomPanel};
use egui_extras::{Column, TableBuilder};
use std::collections::{BTreeMap, BTreeSet};

const DEFAULT_ROW_HEIGHT: f32 = 18.0;

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
    hide_default: bool,
    #[serde(skip)]
    values_table_heights: Vec<f32>,
    #[serde(skip)]
    changed: bool,
    #[serde(skip)]
    prev_loading_count: i32,
    #[serde(skip)]
    value_container: Option<AttributeValueContainer>,
}

impl AttributeDetailWindow {
    pub fn new(specifier: AttributeSpecifier, hide_default: bool) -> Self {
        Self {
            open: true,
            id: None,
            specifier,
            tab: AttributeDetailTabs::Definitions,
            hide_default,
            values_table_heights: Vec::new(),
            changed: false,
            prev_loading_count: 0,
            value_container: None,
        }
    }

    pub fn set_id(&mut self, id: egui::Id) {
        self.id = Some(id);
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn ui(
        &mut self,
        ctx: &egui::Context,
        state: &mut State,
        definitions_store: &mut DefinitionsStore,
        selector: &mut impl DefinitionSelect,
    ) {
        if let Some(id) = self.id {
            let loading_count = definitions_store.load_all_definitions();
            if loading_count != self.prev_loading_count {
                self.changed = true;
                self.prev_loading_count = loading_count;
            }
            if self.changed || self.value_container.is_none() {
                self.value_container = Some(AttributeValueContainer::new(
                    definitions_store.definitions(),
                    &self.specifier,
                    self.hide_default,
                ));
                self.changed = false;
            }

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
                        self.ui_central_panel(ui, state, selector);
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

            let prev_hide_default = self.hide_default;
            ui.with_layout(egui::Layout::right_to_left(egui::Align::LEFT), |ui| {
                ui.checkbox(&mut self.hide_default, "Hide zero/empty");
            });
            if prev_hide_default != self.hide_default {
                self.changed = true;
            }
        });
    }

    fn ui_central_panel(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut State,
        selector: &mut impl DefinitionSelect,
    ) {
        ScrollArea::vertical().show(ui, |ui| match self.tab {
            AttributeDetailTabs::Definitions => {
                self.ui_definitions_table(ui, state, selector);
            }
            AttributeDetailTabs::Values => {
                self.ui_values_table(ui, state, selector);
            }
        });
    }

    fn ui_definitions_table(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut State,
        selector: &mut impl DefinitionSelect,
    ) {
        if let Some(definition_map) = self.value_container.as_ref().map(|c| c.definition_map()) {
            TableBuilder::new(ui)
                .column(Column::exact(250.0))
                .column(Column::remainder())
                .striped(true)
                .body(|body| {
                    let entries: Vec<(
                        &String,
                        &(WeakDefinitionPointer, BTreeSet<AttributeValue>),
                    )> = definition_map.iter().collect();
                    body.rows(DEFAULT_ROW_HEIGHT, definition_map.len(), |mut row| {
                        let (filename, (definition, values)) = entries[row.index()];
                        let checked = definition
                            .upgrade()
                            .map(|d| selector.is_selected(&d))
                            .unwrap_or(false);

                        row.col(|ui| {
                            let label =
                                ui.selectable_label(checked, filename.clone())
                                    .on_hover_ui(|ui| {
                                        ui.label(filename);
                                    });
                            if label.clicked() {
                                if let Some(d) = definition.upgrade() {
                                    selector.select(&d);
                                }
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
                                        &self.specifier.get_type(),
                                        Some(value),
                                        false,
                                        None,
                                    );
                                }
                            });
                        });
                    });
                });
        }
    }

    fn ui_values_table(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut State,
        selector: &mut impl DefinitionSelect,
    ) {
        if let Some(value_map) = self.value_container.as_ref().map(|c| c.value_map()) {
            TableBuilder::new(ui)
                .column(Column::exact(250.0))
                .column(Column::remainder())
                .striped(true)
                .body(|body| {
                    let entries: Vec<(&AttributeValue, &BTreeMap<String, WeakDefinitionPointer>)> =
                        value_map.iter().collect();
                    self.values_table_heights
                        .resize(entries.len(), DEFAULT_ROW_HEIGHT);

                    body.heterogeneous_rows(
                        self.values_table_heights.clone().into_iter(),
                        |mut row| {
                            let row_index = row.index();
                            let (value, definitions) = entries[row_index];

                            row.col(|ui| {
                                let mut rect;
                                if definitions.len() == 1 {
                                    let (filename, definition) = definitions.iter().next().unwrap();
                                    let checked = selector.is_selected_weak(definition);
                                    let response = ui.selectable_label(checked, filename);
                                    rect = response.rect;
                                    if response.clicked() {
                                        selector.select_weak(definition);
                                    }
                                } else {
                                    let collapsing_response = ui.collapsing(
                                        format!("{} definitions", definitions.len()),
                                        |ui| {
                                            for (filename, definition) in definitions {
                                                let checked = selector.is_selected_weak(definition);
                                                if ui.selectable_label(checked, filename).clicked()
                                                {
                                                    selector.select_weak(definition);
                                                }
                                            }
                                        },
                                    );

                                    rect = collapsing_response.header_response.rect;
                                    if let Some(body_res) = collapsing_response.body_response {
                                        rect = rect.union(body_res.rect);
                                    }
                                }

                                if row_index >= self.values_table_heights.len() {
                                    self.values_table_heights
                                        .resize(row_index, DEFAULT_ROW_HEIGHT);
                                }
                                self.values_table_heights.insert(row_index, rect.height());
                            });

                            row.col(|ui| {
                                ui_attribute_value(
                                    ui,
                                    state,
                                    &self.specifier.get_type(),
                                    Some(value),
                                    false,
                                    None,
                                );
                            });
                        },
                    );
                });
        }
    }
}
