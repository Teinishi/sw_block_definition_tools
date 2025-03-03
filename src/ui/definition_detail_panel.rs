use super::{
    attribute_value::AttributeValueAction, ui_attribute_value, AttributeDetailWindow, State,
};
use crate::sw_block_definition::{
    AttributeEnum, AttributeSpecifier, AttributeValue, DefinitionAttribute, SfxData,
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

    fn check(&self, value: &Option<AttributeValue>) -> bool {
        let is_default = value.as_ref().is_some_and(|v| v.is_default());
        (self.show_all || value.is_some()) && !(self.hide_default && is_default)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct DefinitionDetailPanel {}

impl DefinitionDetailPanel {
    pub fn ui(&mut self, ui: &mut Ui, state: &mut State) -> Option<AttributeDetailWindow> {
        let attribute_filter = AttributeFilter::from_state(state);

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

                #[cfg(not(target_arch = "wasm32"))]
                {
                    ui.add_space(10.0);
                    if ui.button("Open").clicked() {
                        let _ = open::that(definition.path());
                    }
                }
            });

            ui.add_space(4.0);

            let mut clicked_attribute: Option<AttributeSpecifier> = None;
            let mut action: Option<AttributeValueAction> = None;

            egui::CollapsingHeader::new("definition attributes")
                .default_open(true)
                .show_unindented(ui, |ui| {
                    let mut clicked = None;
                    attribute_list(
                        ui,
                        state,
                        Id::new("definition_attribute_table"),
                        &attribute_filter,
                        DefinitionAttribute::VARIANTS,
                        &data,
                        &mut clicked,
                        &mut action,
                    );
                    if let Some(c) = clicked {
                        clicked_attribute = Some(c.into());
                    }
                });

            if let Some(sfx_datas) = data.sfx_datas.last() {
                for (i, item) in sfx_datas.sfx_data.iter().enumerate() {
                    let title = match &item.sfx_name {
                        Some(name) => format!("sfx_data ({})", name),
                        None => "sfx_data".to_string(),
                    };
                    ui.collapsing(title, |ui| {
                        sfx_data_table(
                            ui,
                            state,
                            Id::new(format!("sfx_data_table_{}", i)),
                            &attribute_filter,
                            item,
                            &mut clicked_attribute,
                            &mut action,
                        );
                    });
                }
            }

            if let Some(action) = action {
                AttributeValueAction::do_action(action, state);
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

#[allow(clippy::too_many_arguments)]
fn attribute_list<T: AttributeEnum<S> + Clone, S>(
    ui: &mut Ui,
    state: &State,
    id: Id,
    attribute_filter: &AttributeFilter,
    attributes: &[T],
    data: &S,
    clicked_attribute: &mut Option<T>,
    action: &mut Option<AttributeValueAction>,
) {
    Grid::new(id)
        .num_columns(3)
        .min_col_width(0.0)
        .spacing([10.0, 4.0])
        .striped(true)
        .show(ui, |ui| {
            for attr in attributes {
                let value = attr.get_value(data);
                if attribute_filter.check(&value) {
                    if ui.button("...").clicked() {
                        *clicked_attribute = Some(attr.clone());
                    }
                    ui.label(attr.to_string());
                    ui_attribute_value(ui, state, attr.property(), value.as_ref(), action);
                    ui.end_row();
                }
            }
        });
}

fn attribute_table<T: AttributeEnum<S>, S>(
    ui: &mut Ui,
    state: &State,
    id: Id,
    attribute_filter: &AttributeFilter,
    attrs: &[T],
    items: &[S],
    action: &mut Option<AttributeValueAction>,
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
                    ui_attribute_value(
                        ui,
                        state,
                        attr.property(),
                        attr.get_value(item).as_ref(),
                        action,
                    );
                }
                ui.end_row();
            }
        });
}

fn sfx_data_table(
    ui: &mut Ui,
    state: &State,
    id: Id,
    attribute_filter: &AttributeFilter,
    sfx_data: &SfxData,
    clicked_attribute: &mut Option<AttributeSpecifier>,
    action: &mut Option<AttributeValueAction>,
) {
    let mut clicked = None;
    attribute_list(
        ui,
        state,
        id,
        attribute_filter,
        SfxDataAttribute::VARIANTS,
        sfx_data,
        &mut clicked,
        action,
    );
    if let Some(c) = clicked {
        *clicked_attribute = Some(c.into());
    }

    if let Some(layers) = sfx_data.sfx_layers.last() {
        ui.add_space(4.0);
        attribute_table(
            ui,
            state,
            id.with("layer_table"),
            attribute_filter,
            SfxLayerAttribute::VARIANTS,
            &layers.sfx_layer,
            action,
        );
    }
}
