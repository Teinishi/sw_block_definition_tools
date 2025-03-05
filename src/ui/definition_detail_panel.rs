use super::{ui_attribute_value, AttributeDetailWindow, State};
use crate::sw_block_definition::{
    AttributeProperty, AttributeSpecifier, AttributeValue, CouplingAttribute, Definition,
    DefinitionAttribute, GetAttributeValue, GetAttributeValueRoot, IsDefault, LogicNodeAttribute,
    SfxData, SfxDataAttribute, SfxLayerAttribute, SurfaceAttribute, VoxelAttribute,
};
use egui::{Align, Button, CollapsingHeader, Grid, Id, Layout, RichText, Ui};
use strum::VariantArray;

struct AttributeFilter {
    show_all: bool,
    hide_default: bool,
}

impl AttributeFilter {
    fn from_state(state: &State) -> Self {
        Self {
            show_all: state.show_all(),
            hide_default: state.hide_default(),
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

            ui.separator();

            let mut clicked_attribute: Option<AttributeSpecifier> = None;

            // <definition> の属性リスト
            collapsing_heading(
                ui,
                "Definition Attributes",
                |ui| {
                    let mut clicked = None;
                    attribute_list(
                        ui,
                        state,
                        Id::new("definition_attribute_table"),
                        &attribute_filter,
                        &DefinitionAttribute::NON_ELEMENT_VARIANTS,
                        &data,
                        &mut clicked,
                    );
                    if let Some(c) = clicked {
                        clicked_attribute = Some(c.into());
                    }
                },
                true,
            );

            // <sfx_datas> のリスト
            if let Some(sfx_datas) = data.sfx_datas.last() {
                for (i, item) in sfx_datas.sfx_data.iter().enumerate() {
                    let title = match &item.sfx_name {
                        Some(name) => format!("Sfx data ({})", name),
                        None => "Sfx data".to_string(),
                    };
                    collapsing_heading(
                        ui,
                        title,
                        |ui| {
                            sfx_data_table(
                                ui,
                                state,
                                Id::new(format!("sfx_data_table_{}", i)),
                                &attribute_filter,
                                item,
                                &mut clicked_attribute,
                            );
                        },
                        false,
                    );
                }
            }

            // <surfaces> のリスト
            elements_table(
                ui,
                state,
                "Surfaces",
                SurfaceAttribute::VARIANTS,
                data.surfaces.last().map(|surfaces| &surfaces.surface),
                &attribute_filter,
                &mut clicked_attribute,
            );

            // <buoyancy_surfaces> のリスト
            elements_table(
                ui,
                state,
                "Buoyancy Surfaces",
                SurfaceAttribute::VARIANTS,
                data.buoyancy_surfaces
                    .last()
                    .map(|surfaces| &surfaces.surface),
                &attribute_filter,
                &mut clicked_attribute,
            );

            // <logic_nodes> のリスト
            elements_table(
                ui,
                state,
                "Logic Nodes",
                LogicNodeAttribute::VARIANTS,
                data.logic_nodes
                    .last()
                    .map(|logic_nodes| &logic_nodes.logic_node),
                &attribute_filter,
                &mut clicked_attribute,
            );

            // <couplings> のリスト
            elements_table(
                ui,
                state,
                "Couplings",
                CouplingAttribute::VARIANTS,
                data.couplings.last().map(|couplings| &couplings.coupling),
                &attribute_filter,
                &mut clicked_attribute,
            );

            // <voxels> のリスト
            elements_table(
                ui,
                state,
                "Voxels",
                VoxelAttribute::VARIANTS,
                data.voxels.last().map(|voxels| &voxels.voxel),
                &attribute_filter,
                &mut clicked_attribute,
            );

            // <voxel_min> <voxel_max> <voxel_physics_min> <voxel_physics_max> <bb_physics_min> <bb_physics_max>
            bounding_boxes_table(ui, state, &data, &attribute_filter, &mut clicked_attribute);

            // <compartment_sample_pos> <constraint_pos_parent> <constraint_pos_child> <voxel_location_child>
            positions_table(ui, state, &data, &attribute_filter, &mut clicked_attribute);

            // <seat_offset> <seat_front> <seat_up> <seat_camera> <seat_render> <seat_exit_position>
            seat_table(ui, state, &data, &attribute_filter, &mut clicked_attribute);

            // <light_position> <light_forward> <light_color>
            light_table(ui, state, &data, &attribute_filter, &mut clicked_attribute);

            // <door_size> <door_normal> <door_side> <door_up> <door_base_pos>
            door_table(ui, state, &data, &attribute_filter, &mut clicked_attribute);

            // <dynamic_body_position> <dynamic_rotation_axes> <dynamic_side_axis>

            // <connector_axis> <connector_up>

            // <tooltip_properties> <reward_properties>

            // <jet_engine_connections_prev> <jet_engine_connections_next>

            // <particle_direction> <particle_offset> <particle_bounds>

            // <weapon_breech_position> <weapon_breech_normal> <weapon_cart_position> <weapon_cart_velocity>

            // <force_dir> <magnet_offset> <rope_hook_offset>

            Some(AttributeDetailWindow::new(
                clicked_attribute?,
                state.hide_default(),
            ))
        } else {
            None
        }
    }
}

fn collapsing_heading<R>(
    ui: &mut Ui,
    title: impl Into<String>,
    add_body: impl FnOnce(&mut Ui) -> R,
    default_open: bool,
) -> egui::CollapsingResponse<R> {
    CollapsingHeader::new(RichText::new(title).heading())
        .default_open(default_open)
        .show(ui, add_body)
}

fn attribute_detail_button<T: Copy>(
    ui: &mut Ui,
    attr: &T,
    clicked: &mut Option<T>,
    label: Option<&'_ str>,
) {
    let res = match label {
        Some(label) => ui.add(Button::new(label).small()),
        _ => ui.add_sized([20.0, 20.0], Button::new("...").truncate()),
    };
    if res.clicked() {
        *clicked = Some(*attr);
    }
}

#[allow(clippy::too_many_arguments)]
fn attribute_list<T: GetAttributeValue<S>, S>(
    ui: &mut Ui,
    state: &mut State,
    id: Id,
    attribute_filter: &AttributeFilter,
    attributes: &[T],
    data: &S,
    clicked_attribute: &mut Option<T>,
) {
    Grid::new(id)
        .min_col_width(0.0)
        .spacing([10.0, 4.0])
        .striped(true)
        .show(ui, |ui| {
            for attr in attributes {
                let value = attr.get_value(data);
                if attribute_filter.check(&value) {
                    attribute_detail_button(ui, attr, clicked_attribute, None);
                    ui.label(attr.to_string());
                    ui_attribute_value(ui, state, &attr.property(), value.as_ref(), false, None);
                    ui.end_row();
                }
            }
        });
}

fn attribute_table<T: GetAttributeValue<S>, S>(
    ui: &mut Ui,
    state: &mut State,
    id: Id,
    attribute_filter: &AttributeFilter,
    attrs: &[T],
    items: &[S],
) -> Option<T> {
    let columns: Vec<&T> = attrs
        .iter()
        .filter(|attr| {
            attribute_filter.show_all
                || items
                    .iter()
                    .any(|item| attribute_filter.check(&attr.get_value(item)))
        })
        .collect();
    let mut clicked = None;

    Grid::new(id)
        .min_col_width(0.0)
        .spacing([20.0, 4.0])
        .striped(true)
        .show(ui, |ui| {
            for attr in &columns {
                ui.horizontal(|ui| {
                    ui.strong(attr.to_string());
                    attribute_detail_button(ui, *attr, &mut clicked, None);
                });
            }
            ui.end_row();

            for item in items {
                for attr in &columns {
                    let is_number = attr.property().is_number;
                    ui_attribute_value(
                        ui,
                        state,
                        &attr.property(),
                        attr.get_value(item).as_ref(),
                        true,
                        if is_number { Some((0.0, 28.0)) } else { None },
                    );
                }
                ui.end_row();
            }
        });

    clicked
}

fn elements_table<T: GetAttributeValue<S>, S>(
    ui: &mut Ui,
    state: &mut State,
    name: &'_ str,
    attrs: &[T],
    data: Option<&Vec<S>>,
    attribute_filter: &AttributeFilter,
    clicked_attribute: &mut Option<AttributeSpecifier>,
) {
    if !attribute_filter.show_all && data.map_or(true, |v| v.is_empty()) {
        return;
    }
    collapsing_heading(
        ui,
        name,
        |ui| {
            if let Some(clicked) = attribute_table(
                ui,
                state,
                Id::new(name),
                attribute_filter,
                attrs,
                data.map_or(&Vec::new(), |v| v),
            ) {
                *clicked_attribute = Some(clicked.into());
            }
        },
        false,
    );
}

#[allow(clippy::too_many_arguments)]
fn vec3_table(
    ui: &mut Ui,
    state: &mut State,
    title: impl Into<String>,
    id: Id,
    data: &Definition,
    attribute_filter: &AttributeFilter,
    elements: &[(DefinitionAttribute, &'_ str)],
    clicked_attribute: &mut Option<AttributeSpecifier>,
) {
    let rows: Vec<(&DefinitionAttribute, &&str, [Option<AttributeValue>; 3])> = elements
        .iter()
        .filter_map(|(attr, label)| {
            let values: Option<[Option<AttributeValue>; 3]> = attr
                .get_value(data)
                .and_then(|v| v.vec_as_attribute_values());

            let values = values.unwrap_or([None, None, None]);
            let show = values.iter().any(|value| attribute_filter.check(value));
            if !show {
                return None;
            }

            Some((attr, label, values))
        })
        .collect();

    if rows.is_empty() {
        return;
    }

    let mut clicked = None;
    collapsing_heading(
        ui,
        title,
        |ui| {
            Grid::new(id)
                .min_col_width(0.0)
                .spacing([10.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("");
                    ui.label("");
                    for label in ["x", "y", "z"] {
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.strong(label);
                        });
                    }
                    ui.end_row();

                    for (attr, label, values) in rows {
                        let property = attr.property();
                        attribute_detail_button(ui, attr, &mut clicked, None);
                        ui.label(*label);
                        for value in &values {
                            ui_attribute_value(ui, state, &property, value.as_ref(), true, None);
                        }
                        ui.end_row();
                    }
                });
        },
        false,
    );
    if let Some(clicked) = clicked {
        *clicked_attribute = Some(clicked.into());
    }
}

fn sfx_data_table(
    ui: &mut Ui,
    state: &mut State,
    id: Id,
    attribute_filter: &AttributeFilter,
    sfx_data: &SfxData,
    clicked_attribute: &mut Option<AttributeSpecifier>,
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
    );
    if let Some(c) = clicked {
        *clicked_attribute = Some(c.into());
    }

    if let Some(layers) = sfx_data.sfx_layers.last() {
        ui.add_space(4.0);
        if let Some(clicked) = attribute_table(
            ui,
            state,
            id.with("layer_table"),
            attribute_filter,
            SfxLayerAttribute::VARIANTS,
            &layers.sfx_layer,
        ) {
            *clicked_attribute = Some(clicked.into());
        }
    }
}

fn bounding_boxes_table(
    ui: &mut Ui,
    state: &mut State,
    data: &Definition,
    attribute_filter: &AttributeFilter,
    clicked_attribute: &mut Option<AttributeSpecifier>,
) {
    let mut rows = Vec::with_capacity(3);
    for (name, attr_min, attr_max) in [
        (
            "Voxel",
            DefinitionAttribute::VoxelMin,
            DefinitionAttribute::VoxelMax,
        ),
        (
            "Voxel physics",
            DefinitionAttribute::VoxelPhysicsMin,
            DefinitionAttribute::VoxelPhysicsMax,
        ),
        (
            "BB physics",
            DefinitionAttribute::BbPhysicsMin,
            DefinitionAttribute::BbPhysicsMax,
        ),
    ] {
        let value_min = attr_min.get_value(data);
        let value_max = attr_max.get_value(data);
        let show = attribute_filter.check(&value_min) || attribute_filter.check(&value_max);
        if show {
            if let (Some(values_min), Some(values_max)) = (
                value_min.and_then(|v| v.vec_as_attribute_values()),
                value_max.and_then(|v| v.vec_as_attribute_values()),
            ) {
                rows.push((name, attr_min, attr_max, values_min, values_max));
            }
        }
    }
    if rows.is_empty() {
        return;
    }

    let property = AttributeProperty {
        is_audio_file: false,
        is_number: true,
    };

    let mut clicked = None;
    collapsing_heading(
        ui,
        "Bounding Boxes",
        |ui| {
            Grid::new(Id::new("bounding_boxes_table"))
                .min_col_width(0.0)
                .spacing([10.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("");
                    ui.label("");
                    ui.label("");
                    for label in ["min x", "min y", "min z", "max x", "max y", "max z"] {
                        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                            ui.strong(label);
                        });
                    }
                    ui.end_row();

                    for (name, attr_min, attr_max, values_min, values_max) in rows {
                        attribute_detail_button(ui, &attr_min, &mut clicked, Some("min"));
                        attribute_detail_button(ui, &attr_max, &mut clicked, Some("max"));
                        ui.label(name);
                        for v in values_min {
                            ui_attribute_value(ui, state, &property, v.as_ref(), true, None);
                        }
                        for v in values_max {
                            ui_attribute_value(ui, state, &property, v.as_ref(), true, None);
                        }
                        ui.end_row();
                    }
                });
        },
        false,
    );
    if let Some(clicked) = clicked {
        *clicked_attribute = Some(clicked.into());
    }
}

fn positions_table(
    ui: &mut Ui,
    state: &mut State,
    data: &Definition,
    attribute_filter: &AttributeFilter,
    clicked_attribute: &mut Option<AttributeSpecifier>,
) {
    vec3_table(
        ui,
        state,
        "Positions",
        Id::new("positions_table"),
        data,
        attribute_filter,
        &[
            (
                DefinitionAttribute::CompartmentSamplePos,
                "Compartment sample pos",
            ),
            (
                DefinitionAttribute::ConstraintPosParent,
                "Constraint pos parent",
            ),
            (
                DefinitionAttribute::ConstraintPosChild,
                "Constraint pos child",
            ),
            (
                DefinitionAttribute::VoxelLocationChild,
                "Voxel location child",
            ),
        ],
        clicked_attribute,
    );
}

fn seat_table(
    ui: &mut Ui,
    state: &mut State,
    data: &Definition,
    attribute_filter: &AttributeFilter,
    clicked_attribute: &mut Option<AttributeSpecifier>,
) {
    vec3_table(
        ui,
        state,
        "Seat",
        Id::new("seat_table"),
        data,
        attribute_filter,
        &[
            (DefinitionAttribute::SeatOffset, "Offset"),
            (DefinitionAttribute::SeatFront, "Front"),
            (DefinitionAttribute::SeatUp, "Up"),
            (DefinitionAttribute::SeatCamera, "Camera"),
            (DefinitionAttribute::SeatRender, "Render"),
            (DefinitionAttribute::SeatExitPosition, "Exit position"),
        ],
        clicked_attribute,
    );
}

fn light_table(
    ui: &mut Ui,
    state: &mut State,
    data: &Definition,
    attribute_filter: &AttributeFilter,
    clicked_attribute: &mut Option<AttributeSpecifier>,
) {
    vec3_table(
        ui,
        state,
        "Light",
        Id::new("light_table"),
        data,
        attribute_filter,
        &[
            (DefinitionAttribute::LightPosition, "Position"),
            (DefinitionAttribute::LightForward, "Forward"),
            (DefinitionAttribute::LightColor, "Color"),
        ],
        clicked_attribute,
    );
}

fn door_table(
    ui: &mut Ui,
    state: &mut State,
    data: &Definition,
    attribute_filter: &AttributeFilter,
    clicked_attribute: &mut Option<AttributeSpecifier>,
) {
    vec3_table(
        ui,
        state,
        "Door",
        Id::new("door_table"),
        data,
        attribute_filter,
        &[
            (DefinitionAttribute::DoorSize, "Size"),
            (DefinitionAttribute::DoorNormal, "Normal"),
            (DefinitionAttribute::DoorSide, "Side"),
            (DefinitionAttribute::DoorUp, "Up"),
            (DefinitionAttribute::DoorBasePos, "BasePos"),
        ],
        clicked_attribute,
    );
}
