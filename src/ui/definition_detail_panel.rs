use crate::sw_block_definition::{DefinitionAttribute, DefinitionAttributeValue};
use strum::VariantArray;

use super::{AttributeDetailWindow, State};

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct DefinitionDetailPanel {}

impl DefinitionDetailPanel {
    pub fn ui(&mut self, ui: &mut egui::Ui, state: &mut State) -> Option<AttributeDetailWindow> {
        let definition = state.selected_definition();
        definition.as_ref()?;

        if let Some(data) = definition.unwrap().load_data() {
            if let Err(err) = data {
                ui.collapsing("Error", |ui| {
                    ui.label(err.to_string());
                });
                return None;
            }
            let data = data.unwrap();

            if let Some(name) = &data.name {
                ui.heading(name);
            }

            let clicked = attribute_table(
                ui,
                "definition_detail_table",
                state.show_all_attributes(),
                state.hide_default_attributes(),
                DefinitionAttribute::VARIANTS
                    .iter()
                    .map(|attr| (attr.clone(), attr.get_value(&data))),
            );

            Some(AttributeDetailWindow::new(
                clicked?,
                state.hide_default_attributes(),
            ))
        } else {
            None
        }
    }
}

fn attribute_table(
    ui: &mut egui::Ui,
    id: &str,
    show_all: bool,
    hide_default: bool,
    items: impl IntoIterator<Item = (DefinitionAttribute, Option<DefinitionAttributeValue>)>,
) -> Option<DefinitionAttribute> {
    let mut clicked = None;

    egui::Grid::new(id)
        .num_columns(3)
        .min_col_width(0.0)
        .spacing([10.0, 4.0])
        .striped(true)
        .show(ui, |ui| {
            for (attr, value) in items {
                let is_default = value.as_ref().is_some_and(|v| v.is_default());
                if (show_all || value.is_some()) && !(hide_default && is_default) {
                    if ui.button("...").clicked() {
                        clicked = Some(attr.clone());
                    }

                    ui.label(attr.to_string());

                    if let Some(val) = value {
                        ui.label(val.debug_str());
                    } else {
                        ui.weak("Not defined");
                    }

                    ui.end_row();
                }
            }
        });

    clicked
}
