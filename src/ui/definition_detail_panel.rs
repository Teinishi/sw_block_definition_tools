use super::{ui_attribute_value, AttributeDetailWindow, State};
use crate::sw_block_definition::{
    AttributeProperty, AttributeSpecifier, AttributeValue, CouplingAttribute, DefinitionAttribute,
    GetAttributeValue, IsDefault, JetEngineConnectionAttribute, LogicNodeAttribute,
    SfxDataAttribute, SfxLayerAttribute, SurfaceAttribute, VoxelAttribute,
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
        let is_default = value.as_ref().is_none_or(|v| v.is_default());
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
            let mut list = AttributeList::new(
                "definition_attribute_list",
                &DefinitionAttribute::NON_ELEMENT_VARIANTS,
            );
            if list.update(&attribute_filter, &data) {
                CollapsingPanel::new("Definition Attributes")
                    .default_open(true)
                    .ui(ui, |ui| {
                        list.ui(ui, state, &mut clicked_attribute);
                    });
            }

            // <sfx_datas> のリスト
            if let Some(sfx_datas) = data.sfx_datas.last() {
                for (i, item) in sfx_datas.sfx_data.iter().enumerate() {
                    let mut attribute_list = AttributeList::new(
                        format!("sfx_attribute_list_{}", i),
                        SfxDataAttribute::VARIANTS,
                    );
                    let mut layers_table = ElementsTable::new(
                        format!("sfx_layers_table_{}", i),
                        SfxLayerAttribute::VARIANTS,
                    );

                    let mut show = attribute_list.update(&attribute_filter, item);
                    if let Some(sfx_layers) = item.sfx_layers.last() {
                        if layers_table.update(&attribute_filter, &sfx_layers.sfx_layer) {
                            show = true;
                        }
                    }

                    if show {
                        let title = match &item.sfx_name {
                            Some(name) => format!("Sfx data ({})", name),
                            None => "Sfx data".to_string(),
                        };
                        CollapsingPanel::new(&title).ui(ui, |ui| {
                            attribute_list.ui(ui, state, &mut clicked_attribute);
                            layers_table.ui(ui, state, &mut clicked_attribute);
                        });
                    }
                }
            }

            // <surfaces> のリスト
            if let Some(surfaces) = &data.surfaces.last() {
                let mut table = ElementsTable::new("surfaces_table", SurfaceAttribute::VARIANTS);
                if table.update(&attribute_filter, &surfaces.surface) {
                    CollapsingPanel::new("Surfaces").ui(ui, |ui| {
                        table.ui(ui, state, &mut clicked_attribute);
                    });
                }
            }

            // <buoyancy_surfaces> のリスト
            if let Some(buoyancy_surfaces) = &data.buoyancy_surfaces.last() {
                let mut table =
                    ElementsTable::new("buoyancy_surfaces_table", SurfaceAttribute::VARIANTS);
                if table.update(&attribute_filter, &buoyancy_surfaces.surface) {
                    CollapsingPanel::new("Buoyancy surfaces").ui(ui, |ui| {
                        table.ui(ui, state, &mut clicked_attribute);
                    });
                }
            }

            // <logic_nodes> のリスト
            if let Some(logic_nodes) = &data.logic_nodes.last() {
                let mut table =
                    ElementsTable::new("logic_nodes_table", LogicNodeAttribute::VARIANTS);
                if table.update(&attribute_filter, &logic_nodes.logic_node) {
                    CollapsingPanel::new("Logic nodes").ui(ui, |ui| {
                        table.ui(ui, state, &mut clicked_attribute);
                    });
                }
            }

            // <couplings> のリスト
            if let Some(couplings) = &data.couplings.last() {
                let mut table = ElementsTable::new("couplings_table", CouplingAttribute::VARIANTS);
                if table.update(&attribute_filter, &couplings.coupling) {
                    CollapsingPanel::new("Couplings").ui(ui, |ui| {
                        table.ui(ui, state, &mut clicked_attribute);
                    });
                }
            }

            // <voxels> のリスト
            if let Some(voxels) = &data.voxels.last() {
                let mut table = ElementsTable::new("voxels_table", VoxelAttribute::VARIANTS);
                if table.update(&attribute_filter, &voxels.voxel) {
                    CollapsingPanel::new("Voxels").ui(ui, |ui| {
                        table.ui(ui, state, &mut clicked_attribute);
                    });
                }
            }

            // <voxel_min> <voxel_max> <voxel_physics_min> <voxel_physics_max> <bb_physics_min> <bb_physics_max>
            let mut table = MultipleVecTable::new(
                "bounding_boxes_table",
                Some(["min", "max"]),
                [
                    (
                        "Voxel",
                        [DefinitionAttribute::VoxelMin, DefinitionAttribute::VoxelMax],
                    ),
                    (
                        "Voxel physics",
                        [
                            DefinitionAttribute::VoxelPhysicsMin,
                            DefinitionAttribute::VoxelPhysicsMax,
                        ],
                    ),
                    (
                        "BB physics",
                        [
                            DefinitionAttribute::BbPhysicsMin,
                            DefinitionAttribute::BbPhysicsMax,
                        ],
                    ),
                ],
            );
            if table.update(&attribute_filter, &data) {
                CollapsingPanel::new("Bouding boxes").ui(ui, |ui| {
                    table.ui(ui, state, &mut clicked_attribute);
                });
            }

            // <seat_offset> <seat_front> <seat_up> <seat_camera> <seat_render> <seat_exit_position>
            let mut table = MultipleVecTable::single(
                "seat_table",
                [
                    ("Offset", DefinitionAttribute::SeatOffset),
                    ("Front", DefinitionAttribute::SeatFront),
                    ("Up", DefinitionAttribute::SeatUp),
                    ("Camera", DefinitionAttribute::SeatCamera),
                    ("Render", DefinitionAttribute::SeatRender),
                    ("Exit position", DefinitionAttribute::SeatExitPosition),
                ],
            );
            if table.update(&attribute_filter, &data) {
                CollapsingPanel::new("Seat").ui(ui, |ui| {
                    table.ui(ui, state, &mut clicked_attribute);
                });
            }

            // <light_position> <light_forward> <light_color>
            let mut table = MultipleVecTable::single(
                "light_table",
                [
                    ("Position", DefinitionAttribute::LightPosition),
                    ("Forward", DefinitionAttribute::LightForward),
                    ("Color", DefinitionAttribute::LightColor),
                ],
            );
            if table.update(&attribute_filter, &data) {
                CollapsingPanel::new("Light").ui(ui, |ui| {
                    table.ui(ui, state, &mut clicked_attribute);
                });
            }

            // <door_size> <door_normal> <door_side> <door_up> <door_base_pos>
            let mut table = MultipleVecTable::single(
                "door_table",
                [
                    ("Size", DefinitionAttribute::DoorSize),
                    ("Normal", DefinitionAttribute::DoorNormal),
                    ("Side", DefinitionAttribute::DoorSide),
                    ("Up", DefinitionAttribute::DoorUp),
                    ("BasePos", DefinitionAttribute::DoorBasePos),
                ],
            );
            if table.update(&attribute_filter, &data) {
                CollapsingPanel::new("Door").ui(ui, |ui| {
                    table.ui(ui, state, &mut clicked_attribute);
                });
            }

            // <dynamic_body_position> <dynamic_rotation_axes> <dynamic_side_axis>
            let mut table = MultipleVecTable::single(
                "dynamic_table",
                [
                    ("Body position", DefinitionAttribute::DynamicBodyPosition),
                    ("Rotation axes", DefinitionAttribute::DynamicRotationAxes),
                    ("Side axis", DefinitionAttribute::DynamicSideAxis),
                ],
            );
            if table.update(&attribute_filter, &data) {
                CollapsingPanel::new("Dynamic").ui(ui, |ui| {
                    table.ui(ui, state, &mut clicked_attribute);
                });
            }

            // <connector_axis> <connector_up>
            let mut table = MultipleVecTable::single(
                "connector_table",
                [
                    ("Axis", DefinitionAttribute::ConnectorAxis),
                    ("Up", DefinitionAttribute::ConnectorUp),
                ],
            );
            if table.update(&attribute_filter, &data) {
                CollapsingPanel::new("Connector").ui(ui, |ui| {
                    table.ui(ui, state, &mut clicked_attribute);
                });
            }

            // <tooltip_properties> <reward_properties>

            // <jet_engine_connections_prev> <jet_engine_connections_next>
            let mut table = MultipleVecTable::new(
                "jet_engine_connections_table",
                Some(["pos", "normal"]),
                [
                    (
                        "Prev",
                        [
                            JetEngineConnectionAttribute::PrevPos,
                            JetEngineConnectionAttribute::PrevNormal,
                        ],
                    ),
                    (
                        "Next",
                        [
                            JetEngineConnectionAttribute::NextPos,
                            JetEngineConnectionAttribute::NextNormal,
                        ],
                    ),
                ],
            );
            if table.update(&attribute_filter, &data) {
                CollapsingPanel::new("Jet engine connection").ui(ui, |ui| {
                    table.ui(ui, state, &mut clicked_attribute);
                });
            }

            // <particle_direction> <particle_offset> <particle_bounds>
            let mut table = MultipleVecTable::single(
                "particle_table",
                [
                    ("Direction", DefinitionAttribute::ParticleDirection),
                    ("Offset", DefinitionAttribute::ParticleOffset),
                    ("Bounds", DefinitionAttribute::ParticleBounds),
                ],
            );
            if table.update(&attribute_filter, &data) {
                CollapsingPanel::new("Particle").ui(ui, |ui| {
                    table.ui(ui, state, &mut clicked_attribute);
                });
            }

            // <weapon_breech_position> <weapon_breech_normal> <weapon_cart_position> <weapon_cart_velocity>
            let mut table = MultipleVecTable::single(
                "weapon_table",
                [
                    ("Breech position", DefinitionAttribute::WeaponBreechPosition),
                    ("Breech normal", DefinitionAttribute::WeaponBreechNormal),
                    ("Cart position", DefinitionAttribute::WeaponCartPosition),
                    ("Cart velocity", DefinitionAttribute::WeaponCartVelocity),
                ],
            );
            if table.update(&attribute_filter, &data) {
                CollapsingPanel::new("Weapon").ui(ui, |ui| {
                    table.ui(ui, state, &mut clicked_attribute);
                });
            }

            // <compartment_sample_pos> <constraint_pos_parent> <constraint_pos_child> <voxel_location_child> <force_dir> <magnet_offset> <rope_hook_offset>
            let mut table = MultipleVecTable::single(
                "others_table",
                [
                    (
                        "Compartment sample pos",
                        DefinitionAttribute::CompartmentSamplePos,
                    ),
                    (
                        "Constraint pos parent",
                        DefinitionAttribute::ConstraintPosParent,
                    ),
                    (
                        "Constraint pos child",
                        DefinitionAttribute::ConstraintPosChild,
                    ),
                    (
                        "Voxel location child",
                        DefinitionAttribute::VoxelLocationChild,
                    ),
                    ("Force dir", DefinitionAttribute::ForceDir),
                    ("Magnet offset", DefinitionAttribute::MagnetOffset),
                    ("Rope hook offset", DefinitionAttribute::RopeHookOffset),
                ],
            );
            if table.update(&attribute_filter, &data) {
                CollapsingPanel::new("Others").ui(ui, |ui| {
                    table.ui(ui, state, &mut clicked_attribute);
                });
            }

            Some(AttributeDetailWindow::new(
                clicked_attribute?,
                state.hide_default(),
            ))
        } else {
            None
        }
    }
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

struct CollapsingPanel<'a> {
    title: &'a str,
    default_open: bool,
}
impl<'a> CollapsingPanel<'a> {
    fn new(title: &'a str) -> Self {
        Self {
            title,
            default_open: false,
        }
    }

    fn default_open(mut self, value: bool) -> Self {
        self.default_open = value;
        self
    }

    fn ui<R>(&self, ui: &mut Ui, add_body: impl FnOnce(&mut Ui) -> R) {
        CollapsingHeader::new(RichText::new(self.title).heading())
            .default_open(self.default_open)
            .show(ui, add_body);
    }
}

struct AttributeList<'a, T> {
    id: Id,
    attributes: &'a [T],
    filtered_data: Option<Vec<(&'a T, Option<AttributeValue>)>>,
}
impl<'a, T> AttributeList<'a, T> {
    fn new(id: impl std::hash::Hash, attributes: &'a [T]) -> Self {
        Self {
            id: Id::new(id),
            attributes,
            filtered_data: None,
        }
    }

    fn update<S>(&mut self, attribute_filter: &AttributeFilter, data: &S) -> bool
    where
        T: GetAttributeValue<S>,
    {
        let filtered_data: Vec<(&'a T, Option<AttributeValue>)> = self
            .attributes
            .iter()
            .filter_map(|attr| {
                let value = attr.get_value(data);
                if attribute_filter.check(&value) {
                    Some((attr, value))
                } else {
                    None
                }
            })
            .collect();
        if filtered_data.is_empty() {
            self.filtered_data = None;
            false
        } else {
            self.filtered_data = Some(filtered_data);
            true
        }
    }

    fn ui<S>(
        &self,
        ui: &mut Ui,
        state: &mut State,
        clicked_attribute: &mut Option<AttributeSpecifier>,
    ) where
        T: GetAttributeValue<S> + Into<AttributeSpecifier>,
    {
        if let Some(data) = &self.filtered_data {
            let mut clicked = None;

            Grid::new(self.id)
                .min_col_width(0.0)
                .spacing([10.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    for (attr, value) in data {
                        attribute_detail_button(ui, attr, &mut clicked, None);
                        ui.label(attr.to_string());
                        ui_attribute_value(
                            ui,
                            state,
                            &attr.property(),
                            value.as_ref(),
                            false,
                            None,
                        );
                        ui.end_row();
                    }
                });

            if let Some(clicked) = clicked {
                *clicked_attribute = Some((*clicked).into());
            }
        }
    }
}

struct ElementsTable<'a, T, S>
where
    T: GetAttributeValue<S>,
{
    id: Id,
    attributes: &'a [T],
    columns_elements: Option<(Vec<&'a T>, &'a [S])>,
}
impl<'a, T: GetAttributeValue<S>, S> ElementsTable<'a, T, S> {
    fn new(id: impl std::hash::Hash, attributes: &'a [T]) -> Self {
        Self {
            id: Id::new(id),
            attributes,
            columns_elements: None,
        }
    }

    fn update(&mut self, attribute_filter: &AttributeFilter, data: &'a [S]) -> bool {
        let columns: Vec<&T> = self
            .attributes
            .iter()
            .filter(|attr| {
                attribute_filter.show_all
                    || data
                        .iter()
                        .any(|item| attribute_filter.check(&attr.get_value(item)))
            })
            .collect();
        if columns.is_empty() && data.is_empty() {
            self.columns_elements = None;
            false
        } else {
            self.columns_elements = Some((columns, data));
            true
        }
    }

    fn ui(
        &self,
        ui: &mut Ui,
        state: &mut State,
        clicked_attribute: &mut Option<AttributeSpecifier>,
    ) {
        if let Some((columns, elements)) = &self.columns_elements {
            let mut clicked = None;

            Grid::new(self.id)
                .min_col_width(0.0)
                .spacing([20.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    for attr in columns {
                        ui.horizontal(|ui| {
                            ui.strong(attr.to_string());
                            attribute_detail_button(ui, *attr, &mut clicked, None);
                        });
                    }
                    ui.end_row();

                    for item in elements.iter() {
                        for attr in columns {
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

            if let Some(clicked) = clicked {
                *clicked_attribute = Some(clicked.into());
            }
        }
    }
}

type MultipleVecTableRow<'a, const V_COUNT: usize> = (bool, [[Option<AttributeValue>; 3]; V_COUNT]);
struct MultipleVecTable<'a, T, const V_COUNT: usize, const E_COUNT: usize> {
    id: Id,
    variants: Option<[&'a str; V_COUNT]>,
    elements: [(&'a str, [T; V_COUNT]); E_COUNT],
    table_data: Option<[MultipleVecTableRow<'a, V_COUNT>; E_COUNT]>,
}
impl<'a, T, const E_COUNT: usize> MultipleVecTable<'a, T, 1, E_COUNT> {
    fn single(id: impl std::hash::Hash, elements: [(&'a str, T); E_COUNT]) -> Self {
        Self {
            id: Id::new(id),
            variants: None,
            elements: elements.map(|(label, element)| (label, [element])),
            table_data: None,
        }
    }
}
impl<'a, T, const V_COUNT: usize, const E_COUNT: usize> MultipleVecTable<'a, T, V_COUNT, E_COUNT> {
    fn new(
        id: impl std::hash::Hash,
        variants: Option<[&'a str; V_COUNT]>,
        elements: [(&'a str, [T; V_COUNT]); E_COUNT],
    ) -> Self {
        Self {
            id: Id::new(id),
            variants,
            elements,
            table_data: None,
        }
    }

    fn update<S>(&mut self, attribute_filter: &AttributeFilter, data: &S) -> bool
    where
        T: GetAttributeValue<S>,
    {
        let table_data: [(bool, [[Option<AttributeValue>; 3]; V_COUNT]); E_COUNT] =
            self.elements.map(|(_, elements)| {
                let values: [[Option<AttributeValue>; 3]; V_COUNT] = elements.map(|element| {
                    element
                        .get_value(data)
                        .and_then(|v| v.vec_as_attribute_values())
                        .unwrap_or([None, None, None])
                });

                let show_row = values
                    .iter()
                    .any(|vec| vec.iter().any(|v| attribute_filter.check(v)));

                (show_row, values)
            });

        let show_table = table_data.iter().any(|(show_row, _)| *show_row);

        if show_table {
            self.table_data = Some(table_data);
            true
        } else {
            self.table_data = None;
            false
        }
    }

    fn ui<S>(
        &self,
        ui: &mut Ui,
        state: &mut State,
        clicked_attribute: &mut Option<AttributeSpecifier>,
    ) where
        T: GetAttributeValue<S> + Copy,
    {
        if let Some(rows) = &self.table_data {
            let property = AttributeProperty {
                is_audio_file: false,
                is_number: true,
            };

            let mut clicked = None;

            Grid::new(self.id)
                .min_col_width(0.0)
                .spacing([10.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    for _ in 0..(V_COUNT + 1) {
                        ui.label("");
                    }
                    if let Some(variants) = self.variants {
                        for variant in variants {
                            for axis in ["x", "y", "z"] {
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    ui.strong(format!("{} {}", variant, axis));
                                });
                            }
                        }
                    } else {
                        for _ in 0..V_COUNT {
                            for axis in ["x", "y", "z"] {
                                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                    ui.strong(axis);
                                });
                            }
                        }
                    }
                    ui.end_row();

                    for ((label, elements), (show_row, values)) in
                        self.elements.iter().zip(rows.iter())
                    {
                        if !show_row {
                            continue;
                        }
                        if let Some(variants) = self.variants {
                            ui.label(*label);
                            for (variant, element) in variants.iter().zip(elements.iter()) {
                                attribute_detail_button(ui, &element, &mut clicked, Some(variant));
                            }
                        } else {
                            for element in elements {
                                attribute_detail_button(ui, &element, &mut clicked, None);
                            }
                            ui.label(*label);
                        }
                        for vec in values {
                            for v in vec {
                                ui_attribute_value(ui, state, &property, v.as_ref(), true, None);
                            }
                        }
                        ui.end_row();
                    }
                });

            if let Some(clicked) = clicked {
                *clicked_attribute = Some((*clicked).into());
            }
        }
    }
}
