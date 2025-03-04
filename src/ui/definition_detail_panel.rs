use super::{ui_attribute_value, AttributeDetailWindow, State};
use crate::sw_block_definition::{
    AttributeSpecifier, AttributeValue, BbPhysicsMaxAttribute, BbPhysicsMinAttribute,
    CouplingAttribute, Definition, DefinitionAttribute, GetAttributeValue, GetAttributeValueRoot,
    IsDefault, LogicNodeAttribute, SfxData, SfxDataAttribute, SfxLayerAttribute, SurfaceAttribute,
    VoxelAttribute, VoxelMaxAttribute, VoxelMinAttribute, VoxelPhysicsMaxAttribute,
    VoxelPhysicsMinAttribute,
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
                        DefinitionAttribute::VARIANTS,
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

            // <seat_offset> <seat_front> <seat_up> <seat_camera> <seat_render>
            seat_table(ui, state, &data, &attribute_filter, &mut clicked_attribute);

            // <force_dir>

            // <light_position> <light_color> <light_forward>

            // <door_size> <door_normal> <door_side> <door_up> <door_base_pos>

            // <dynamic_body_position> <dynamic_rotation_axes> <dynamic_side_axis>

            // <magnet_offset>

            // <connector_axis> <connector_up>

            // <tooltip_properties>

            // <jet_engine_connections_prev> <jet_engine_connections_next>

            // <particle_direction> <particle_offset> <particle_bounds>

            // <reward_properties>

            // <seat_exit_position>

            // <weapon_breech_position> <weapon_breech_normal> <weapon_cart_position> <weapon_cart_velocity>

            // <rope_hook_offset>

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
            let values: Option<[Option<AttributeValue>; 3]> =
                attr.get_value(data).and_then(|value| match value {
                    AttributeValue::VecI32(v) => Some([
                        v.x_as_attribute_value(),
                        v.y_as_attribute_value(),
                        v.z_as_attribute_value(),
                    ]),
                    AttributeValue::VecOf32(v) => Some([
                        v.x_as_attribute_value(),
                        v.y_as_attribute_value(),
                        v.z_as_attribute_value(),
                    ]),
                    _ => None,
                });
            values.as_ref()?;
            let values = values.unwrap();

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
    // TODO: 表示が0行だったとき完全に非表示
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

                    let show_voxel = attribute_filter
                        .check(&DefinitionAttribute::VoxelMin.get_value(data))
                        || attribute_filter.check(&DefinitionAttribute::VoxelMax.get_value(data));
                    if show_voxel {
                        attribute_detail_button(
                            ui,
                            &DefinitionAttribute::VoxelMin,
                            &mut clicked,
                            Some("min"),
                        );
                        attribute_detail_button(
                            ui,
                            &DefinitionAttribute::VoxelMax,
                            &mut clicked,
                            Some("max"),
                        );
                        ui.label("Voxel");
                        for attr in VoxelMinAttribute::VARIANTS {
                            ui_attribute_value(
                                ui,
                                state,
                                &attr.property(),
                                attr.get_value_root(data).last(),
                                true,
                                None,
                            );
                        }
                        for attr in VoxelMaxAttribute::VARIANTS {
                            ui_attribute_value(
                                ui,
                                state,
                                &attr.property(),
                                attr.get_value_root(data).last(),
                                true,
                                None,
                            );
                        }
                        ui.end_row();
                    }

                    let show_voxel_physics = attribute_filter
                        .check(&DefinitionAttribute::VoxelPhysicsMin.get_value(data))
                        || attribute_filter
                            .check(&DefinitionAttribute::VoxelPhysicsMax.get_value(data));
                    if show_voxel_physics {
                        attribute_detail_button(
                            ui,
                            &DefinitionAttribute::VoxelPhysicsMin,
                            &mut clicked,
                            Some("min"),
                        );
                        attribute_detail_button(
                            ui,
                            &DefinitionAttribute::VoxelPhysicsMax,
                            &mut clicked,
                            Some("max"),
                        );
                        ui.label("Voxel Physics");
                        for attr in VoxelPhysicsMinAttribute::VARIANTS {
                            ui_attribute_value(
                                ui,
                                state,
                                &attr.property(),
                                attr.get_value_root(data).last(),
                                true,
                                None,
                            );
                        }
                        for attr in VoxelPhysicsMaxAttribute::VARIANTS {
                            ui_attribute_value(
                                ui,
                                state,
                                &attr.property(),
                                attr.get_value_root(data).last(),
                                true,
                                None,
                            );
                        }
                        ui.end_row();
                    }

                    let show_bb_physics = attribute_filter
                        .check(&DefinitionAttribute::BbPhysicsMin.get_value(data))
                        || attribute_filter
                            .check(&DefinitionAttribute::BbPhysicsMax.get_value(data));
                    if show_bb_physics {
                        attribute_detail_button(
                            ui,
                            &DefinitionAttribute::BbPhysicsMin,
                            &mut clicked,
                            Some("min"),
                        );
                        attribute_detail_button(
                            ui,
                            &DefinitionAttribute::BbPhysicsMax,
                            &mut clicked,
                            Some("max"),
                        );
                        ui.label("BB Physics");
                        for attr in BbPhysicsMinAttribute::VARIANTS {
                            ui_attribute_value(
                                ui,
                                state,
                                &attr.property(),
                                attr.get_value_root(data).last(),
                                true,
                                None,
                            );
                        }
                        for attr in BbPhysicsMaxAttribute::VARIANTS {
                            ui_attribute_value(
                                ui,
                                state,
                                &attr.property(),
                                attr.get_value_root(data).last(),
                                true,
                                None,
                            );
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
        ],
        clicked_attribute,
    );
}
