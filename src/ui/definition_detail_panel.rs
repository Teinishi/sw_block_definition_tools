use super::{AttributeDetailWindow, State};
use crate::sw_block_definition::{
    AttributeEnum, AttributeSpecifier, DefinitionAttribute, DefinitionAttributeValue, SfxData,
    SfxDataAttribute, SfxLayerAttribute,
};
use egui::{Grid, Id, Ui};
use strum::VariantArray;

struct AttributeFilter {
    show_all: bool,
    hide_default: bool,
}

impl AttributeFilter {
    fn from_state(state: &State) -> Self {
        Self {
            show_all: state.show_all_attributes(),
            hide_default: state.hide_default_attributes(),
        }
    }

    fn check(&self, value: &Option<DefinitionAttributeValue>) -> bool {
        let is_default = value.as_ref().is_some_and(|v| v.is_default());
        (self.show_all || value.is_some()) && !(self.hide_default && is_default)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct DefinitionDetailPanel {}

impl DefinitionDetailPanel {
    pub fn ui(&mut self, ui: &mut Ui, state: &mut State) -> Option<AttributeDetailWindow> {
        let definition = state.selected_definition();
        definition.as_ref()?;
        let definition = definition.unwrap();

        let filename = definition.filename();

        if let Some(data) = definition.load_data() {
            if let Err(err) = data {
                ui.collapsing("Error", |ui| {
                    ui.label(err.to_string());
                });
                return None;
            }
            let data = data.unwrap();

            ui.horizontal(|ui| {
                if let Some(name) = &data.name {
                    ui.heading(name);
                    ui.add_space(10.0);
                }
                ui.weak(filename);
                ui.add_space(10.0);
                if ui.button("Open").clicked() {
                    let _ = open::that(definition.filepath());
                }
            });

            ui.add_space(4.0);

            let attribute_filter = AttributeFilter::from_state(state);

            let mut clicked_attribute: Option<AttributeSpecifier> = None;
            egui::CollapsingHeader::new("definition attributes")
                .default_open(true)
                .show_unindented(ui, |ui| {
                    if let Some(clicked) = attribute_list(
                        ui,
                        Id::new("definition_attribute_table"),
                        &attribute_filter,
                        DefinitionAttribute::VARIANTS
                            .iter()
                            .map(|attr| (attr.clone(), attr.get_value(&data))),
                    ) {
                        clicked_attribute = Some(clicked.into());
                    }
                });

            if let Some(sfx_datas) = data.sfx_datas.last() {
                for (i, item) in sfx_datas.sfx_data.iter().enumerate() {
                    let title = match &item.sfx_name {
                        Some(name) => format!("sfx_data ({})", name),
                        None => "sfx_data".to_string(),
                    };
                    ui.collapsing(title, |ui| {
                        if let Some(clicked) = sfx_data_table(
                            ui,
                            Id::new(format!("sfx_data_table_{}", i)),
                            &attribute_filter,
                            item,
                        ) {
                            clicked_attribute = Some(clicked);
                        }
                    });
                }
            }

            Some(AttributeDetailWindow::new(
                clicked_attribute?,
                state.hide_default_attributes(),
            ))
        } else {
            None
        }
    }
}

fn attribute_list<T: AttributeEnum<S> + Clone, S>(
    ui: &mut Ui,
    id: Id,
    attribute_filter: &AttributeFilter,
    items: impl IntoIterator<Item = (T, Option<DefinitionAttributeValue>)>,
) -> Option<T> {
    let mut clicked = None;

    Grid::new(id)
        .num_columns(3)
        .min_col_width(0.0)
        .spacing([10.0, 4.0])
        .striped(true)
        .show(ui, |ui| {
            for (attr, value) in items {
                if attribute_filter.check(&value) {
                    if ui.button("...").clicked() {
                        clicked = Some(attr.clone());
                    }
                    ui.label(attr.to_string());
                    attr.ui_value(ui, value.as_ref());
                    ui.end_row();
                }
            }
        });

    clicked
}

fn attribute_table<T: AttributeEnum<S>, S>(
    ui: &mut Ui,
    id: Id,
    attribute_filter: &AttributeFilter,
    attrs: &[T],
    items: &[S],
) {
    let columns: Vec<&T> = attrs
        .iter()
        .filter(|attr| {
            items
                .iter()
                .any(|item| attribute_filter.check(&attr.get_value(item)))
        })
        .collect();

    Grid::new(id)
        .num_columns(columns.len())
        .spacing([10.0, 4.0])
        .striped(true)
        .show(ui, |ui| {
            for attr in &columns {
                ui.strong(attr.to_string());
            }
            ui.end_row();

            for item in items {
                for attr in &columns {
                    attr.ui_value(ui, attr.get_value(item).as_ref());
                }
                ui.end_row();
            }
        });
}

fn sfx_data_table(
    ui: &mut Ui,
    id: Id,
    attribute_filter: &AttributeFilter,
    sfx_data: &SfxData,
) -> Option<AttributeSpecifier> {
    let mut clicked_attribute: Option<AttributeSpecifier> = None;

    if let Some(clicked) = attribute_list(
        ui,
        id,
        attribute_filter,
        SfxDataAttribute::VARIANTS
            .iter()
            .map(|attr| (attr.clone(), attr.get_value(sfx_data))),
    ) {
        clicked_attribute = Some(clicked.into());
    }

    if let Some(layers) = sfx_data.sfx_layers.last() {
        ui.add_space(4.0);
        attribute_table(
            ui,
            id.with("layer_table"),
            attribute_filter,
            SfxLayerAttribute::VARIANTS,
            &layers.sfx_layer,
        );
    }

    clicked_attribute
}
