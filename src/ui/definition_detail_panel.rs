use super::{ui_attribute_value, AttributeDetailWindow, State};
use crate::sw_block_definition::{
    AttributeSpecifier, AttributeType, AttributeValue, Coupling, CouplingAttribute,
    DefinitionAttribute, GetAttributeValue, IsDefault, JetEngineConnectionAttribute, LogicNode,
    LogicNodeAttribute, RewardPropertiesAttribute, SfxDataAttribute, SfxLayer, SfxLayerAttribute,
    Surface, SurfaceAttribute, TooltipPropertiesAttribute, Voxel, VoxelAttribute,
};
use egui::{Align, Button, CollapsingHeader, Layout, RichText, Ui};
use egui_extras::{Column, TableBuilder};
use strum::{EnumCount, VariantArray};

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
            let mut list = AttributeList::new(&DefinitionAttribute::NON_ELEMENT_VARIANTS);
            if list.update(&attribute_filter, Some(&data)) {
                CollapsingPanel::new("Definition Attributes")
                    .default_open(true)
                    .ui(ui, |ui| {
                        list.ui(ui, state, &mut clicked_attribute);
                    });
            }

            // <sfx_datas> のリスト
            if let Some(sfx_datas) = data.sfx_datas.last() {
                for (i, item) in sfx_datas.sfx_data.iter().enumerate() {
                    let mut attribute_list = AttributeList::new(SfxDataAttribute::VARIANTS);
                    let mut layers_table: ElementsTable<
                        '_,
                        SfxLayerAttribute,
                        SfxLayer,
                        { SfxLayerAttribute::COUNT },
                    > = ElementsTable::new(SfxLayerAttribute::VARIANTS.try_into().unwrap());

                    let mut show = attribute_list.update(&attribute_filter, Some(item));

                    if layers_table.update(
                        &attribute_filter,
                        item.sfx_layers.last().map(|s| s.sfx_layer.as_slice()),
                    ) {
                        show = true;
                    }

                    if show {
                        let title = match &item.sfx_name {
                            Some(name) => format!("Sfx data ({})", name),
                            None => "Sfx data".to_string(),
                        };
                        CollapsingPanel::new(&title).ui(ui, |ui| {
                            ui.push_id(2 * i, |ui| {
                                attribute_list.ui(ui, state, &mut clicked_attribute);
                            });
                            ui.push_id(2 * i + 1, |ui| {
                                layers_table.ui(ui, state, &mut clicked_attribute);
                            });
                        });
                    }
                }
            }

            // <surfaces> のリスト
            let mut table: ElementsTable<
                '_,
                SurfaceAttribute,
                Surface,
                { SurfaceAttribute::COUNT },
            > = ElementsTable::new(SurfaceAttribute::VARIANTS.try_into().unwrap());
            if table.update(
                &attribute_filter,
                data.surfaces.last().map(|s| s.surface.as_slice()),
            ) {
                CollapsingPanel::new(&format!("Surfaces ({})", table.len().unwrap_or(0))).ui(
                    ui,
                    |ui| {
                        table.ui(ui, state, &mut clicked_attribute);
                    },
                );
            }

            // <buoyancy_surfaces> のリスト
            let mut table: ElementsTable<
                '_,
                SurfaceAttribute,
                Surface,
                { SurfaceAttribute::COUNT },
            > = ElementsTable::new(SurfaceAttribute::VARIANTS.try_into().unwrap());
            if table.update(
                &attribute_filter,
                data.buoyancy_surfaces.last().map(|s| s.surface.as_slice()),
            ) {
                CollapsingPanel::new(&format!("Buoyancy surfaces ({})", table.len().unwrap_or(0)))
                    .ui(ui, |ui| {
                        table.ui(ui, state, &mut clicked_attribute);
                    });
            }

            // <logic_nodes> のリスト
            let mut table: ElementsTable<
                '_,
                LogicNodeAttribute,
                LogicNode,
                { LogicNodeAttribute::COUNT },
            > = ElementsTable::new(LogicNodeAttribute::VARIANTS.try_into().unwrap());
            if table.update(
                &attribute_filter,
                data.logic_nodes.last().map(|l| l.logic_node.as_slice()),
            ) {
                CollapsingPanel::new(&format!("Logic nodes ({})", table.len().unwrap_or(0))).ui(
                    ui,
                    |ui| {
                        table.ui(ui, state, &mut clicked_attribute);
                    },
                );
            }

            // <couplings> のリスト
            let mut table: ElementsTable<
                '_,
                CouplingAttribute,
                Coupling,
                { CouplingAttribute::COUNT },
            > = ElementsTable::new(CouplingAttribute::VARIANTS.try_into().unwrap());
            if table.update(
                &attribute_filter,
                data.couplings.last().map(|c| c.coupling.as_slice()),
            ) {
                CollapsingPanel::new(&format!("Couplings ({})", table.len().unwrap_or(0))).ui(
                    ui,
                    |ui| {
                        table.ui(ui, state, &mut clicked_attribute);
                    },
                );
            }

            // <voxels> のリスト
            let mut table: ElementsTable<'_, VoxelAttribute, Voxel, { VoxelAttribute::COUNT }> =
                ElementsTable::new(VoxelAttribute::VARIANTS.try_into().unwrap());
            if table.update(
                &attribute_filter,
                data.voxels.last().map(|v| v.voxel.as_slice()),
            ) {
                CollapsingPanel::new(&format!("Voxels ({})", table.len().unwrap_or(0))).ui(
                    ui,
                    |ui| {
                        table.ui(ui, state, &mut clicked_attribute);
                    },
                );
            }

            // <voxel_min> <voxel_max> <voxel_physics_min> <voxel_physics_max> <bb_physics_min> <bb_physics_max>
            let mut table = MultipleVecTable::new(
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
            if table.update(&attribute_filter, Some(&data)) {
                CollapsingPanel::new("Bouding boxes").ui(ui, |ui| {
                    table.ui(ui, state, &mut clicked_attribute);
                });
            }

            // <seat_offset> <seat_front> <seat_up> <seat_camera> <seat_render> <seat_exit_position>
            let mut table = MultipleVecTable::single([
                ("Offset", DefinitionAttribute::SeatOffset),
                ("Front", DefinitionAttribute::SeatFront),
                ("Up", DefinitionAttribute::SeatUp),
                ("Camera", DefinitionAttribute::SeatCamera),
                ("Render", DefinitionAttribute::SeatRender),
                ("Exit position", DefinitionAttribute::SeatExitPosition),
            ]);
            if table.update(&attribute_filter, Some(&data)) {
                CollapsingPanel::new("Seat").ui(ui, |ui| {
                    table.ui(ui, state, &mut clicked_attribute);
                });
            }

            // <light_position> <light_forward> <light_color>
            let mut table = MultipleVecTable::single([
                ("Position", DefinitionAttribute::LightPosition),
                ("Forward", DefinitionAttribute::LightForward),
                ("Color", DefinitionAttribute::LightColor),
            ]);
            if table.update(&attribute_filter, Some(&data)) {
                CollapsingPanel::new("Light").ui(ui, |ui| {
                    table.ui(ui, state, &mut clicked_attribute);
                });
            }

            // <door_size> <door_normal> <door_side> <door_up> <door_base_pos>
            let mut table = MultipleVecTable::single([
                ("Size", DefinitionAttribute::DoorSize),
                ("Normal", DefinitionAttribute::DoorNormal),
                ("Side", DefinitionAttribute::DoorSide),
                ("Up", DefinitionAttribute::DoorUp),
                ("BasePos", DefinitionAttribute::DoorBasePos),
            ]);
            if table.update(&attribute_filter, Some(&data)) {
                CollapsingPanel::new("Door").ui(ui, |ui| {
                    table.ui(ui, state, &mut clicked_attribute);
                });
            }

            // <dynamic_body_position> <dynamic_rotation_axes> <dynamic_side_axis>
            let mut table = MultipleVecTable::single([
                ("Body position", DefinitionAttribute::DynamicBodyPosition),
                ("Rotation axes", DefinitionAttribute::DynamicRotationAxes),
                ("Side axis", DefinitionAttribute::DynamicSideAxis),
            ]);
            if table.update(&attribute_filter, Some(&data)) {
                CollapsingPanel::new("Dynamic").ui(ui, |ui| {
                    table.ui(ui, state, &mut clicked_attribute);
                });
            }

            // <connector_axis> <connector_up>
            let mut table = MultipleVecTable::single([
                ("Axis", DefinitionAttribute::ConnectorAxis),
                ("Up", DefinitionAttribute::ConnectorUp),
            ]);
            if table.update(&attribute_filter, Some(&data)) {
                CollapsingPanel::new("Connector").ui(ui, |ui| {
                    table.ui(ui, state, &mut clicked_attribute);
                });
            }

            // <tooltip_properties>
            let mut list = AttributeList::new(TooltipPropertiesAttribute::VARIANTS);
            if list.update(&attribute_filter, data.tooltip_properties.last()) {
                CollapsingPanel::new("Tooltip properties")
                    .default_open(true)
                    .ui(ui, |ui| {
                        list.ui(ui, state, &mut clicked_attribute);
                    });
            }

            // <reward_properties>
            let mut list = AttributeList::new(RewardPropertiesAttribute::VARIANTS);
            if list.update(&attribute_filter, data.reward_properties.last()) {
                CollapsingPanel::new("Reward properties")
                    .default_open(true)
                    .ui(ui, |ui| {
                        list.ui(ui, state, &mut clicked_attribute);
                    });
            }

            // <jet_engine_connections_prev> <jet_engine_connections_next>
            let mut table = MultipleVecTable::new(
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
            if table.update(&attribute_filter, Some(&data)) {
                CollapsingPanel::new("Jet engine connection").ui(ui, |ui| {
                    table.ui(ui, state, &mut clicked_attribute);
                });
            }

            // <particle_direction> <particle_offset> <particle_bounds>
            let mut table = MultipleVecTable::single([
                ("Direction", DefinitionAttribute::ParticleDirection),
                ("Offset", DefinitionAttribute::ParticleOffset),
                ("Bounds", DefinitionAttribute::ParticleBounds),
            ]);
            if table.update(&attribute_filter, Some(&data)) {
                CollapsingPanel::new("Particle").ui(ui, |ui| {
                    table.ui(ui, state, &mut clicked_attribute);
                });
            }

            // <weapon_breech_position> <weapon_breech_normal> <weapon_cart_position> <weapon_cart_velocity>
            let mut table = MultipleVecTable::single([
                ("Breech position", DefinitionAttribute::WeaponBreechPosition),
                ("Breech normal", DefinitionAttribute::WeaponBreechNormal),
                ("Cart position", DefinitionAttribute::WeaponCartPosition),
                ("Cart velocity", DefinitionAttribute::WeaponCartVelocity),
            ]);
            if table.update(&attribute_filter, Some(&data)) {
                CollapsingPanel::new("Weapon").ui(ui, |ui| {
                    table.ui(ui, state, &mut clicked_attribute);
                });
            }

            // <compartment_sample_pos> <constraint_pos_parent> <constraint_pos_child> <voxel_location_child> <force_dir> <magnet_offset> <rope_hook_offset>
            let mut table = MultipleVecTable::single([
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
            ]);
            if table.update(&attribute_filter, Some(&data)) {
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
        ui.add_space(8.0);
    }
}

struct AttributeList<'a, T> {
    attributes: &'a [T],
    list_data: Option<Vec<(&'a T, Option<AttributeValue>)>>,
}
impl<'a, T> AttributeList<'a, T> {
    fn new(attributes: &'a [T]) -> Self {
        Self {
            attributes,
            list_data: None,
        }
    }

    fn update<S>(&mut self, attribute_filter: &AttributeFilter, data: Option<&S>) -> bool
    where
        T: GetAttributeValue<S>,
    {
        let filtered_data: Vec<(&'a T, Option<AttributeValue>)> = self
            .attributes
            .iter()
            .filter_map(|attr| {
                let value = data.and_then(|d| attr.get_value(d));
                if attribute_filter.check(&value) {
                    Some((attr, value))
                } else {
                    None
                }
            })
            .collect();
        if filtered_data.is_empty() {
            self.list_data = None;
            false
        } else {
            self.list_data = Some(filtered_data);
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
        if let Some(data) = &self.list_data {
            let mut clicked = None;

            TableBuilder::new(ui)
                .column(Column::auto_with_initial_suggestion(0.0))
                .columns(Column::remainder(), 2)
                .cell_layout(Layout::left_to_right(Align::Center))
                .striped(true)
                .vscroll(false)
                .body(|body| {
                    body.rows(18.0, data.len(), |mut row| {
                        let (attr, value) = &data[row.index()];
                        row.col(|ui| {
                            attribute_detail_button(ui, attr, &mut clicked, None);
                        });
                        row.col(|ui| {
                            ui.label(attr.to_string());
                        });
                        row.col(|ui| {
                            ui_attribute_value(
                                ui,
                                state,
                                &attr.get_type(),
                                value.as_ref(),
                                false,
                                None,
                            );
                        });
                    });
                });

            if let Some(clicked) = clicked {
                *clicked_attribute = Some((*clicked).into());
            }
        }
    }
}

struct ElementsTable<'a, T, S, const COUNT: usize>
where
    T: GetAttributeValue<S>,
{
    attributes: [T; COUNT],
    table_data: Option<([bool; COUNT], Vec<&'a S>)>,
}
impl<'a, T: GetAttributeValue<S>, S, const COUNT: usize> ElementsTable<'a, T, S, COUNT> {
    fn new(attributes: [T; COUNT]) -> Self {
        Self {
            attributes,
            table_data: None,
        }
    }

    fn update(&mut self, attribute_filter: &AttributeFilter, data: Option<&'a [S]>) -> bool {
        let is_empty = data.is_none_or(|data| data.is_empty());

        let show_columns = self.attributes.map(|attr| {
            attribute_filter.show_all
                || data.is_some_and(|data| {
                    data.iter()
                        .any(|item| attribute_filter.check(&attr.get_value(item)))
                })
        });
        let show_any = show_columns.iter().any(|s| *s);

        if is_empty && !show_any {
            self.table_data = None;
            false
        } else {
            let data = data
                .unwrap_or(&[])
                .iter()
                .filter(|item| {
                    self.attributes
                        .iter()
                        .zip(show_columns.iter())
                        .any(|(attr, show_attr)| {
                            *show_attr && attribute_filter.check(&attr.get_value(item))
                        })
                })
                .collect();
            self.table_data = Some((show_columns, data));
            true
        }
    }

    fn len(&self) -> Option<usize> {
        Some(self.table_data.clone()?.1.len())
    }

    fn ui(
        &self,
        ui: &mut Ui,
        state: &mut State,
        clicked_attribute: &mut Option<AttributeSpecifier>,
    ) {
        if let Some((show_columns, elements)) = &self.table_data {
            let mut clicked = None;

            TableBuilder::new(ui)
                .columns(
                    Column::auto_with_initial_suggestion(0.0),
                    count_true(show_columns.iter()),
                )
                .striped(true)
                .vscroll(false)
                .header(20.0, |mut row| {
                    for (attr, show_column) in self.attributes.iter().zip(show_columns.iter()) {
                        if !*show_column {
                            continue;
                        }
                        row.col(|ui| {
                            let reverse = attr.get_type().is_number();
                            tabel_cell_aligned(
                                ui,
                                if reverse { Align::RIGHT } else { Align::LEFT },
                                |ui| {
                                    if reverse {
                                        attribute_detail_button(ui, attr, &mut clicked, None);
                                        ui.strong(attr.to_string());
                                    } else {
                                        ui.strong(attr.to_string());
                                        attribute_detail_button(ui, attr, &mut clicked, None);
                                    }
                                },
                            );
                        });
                    }
                })
                .body(|body| {
                    body.rows(20.0, elements.len(), |mut row| {
                        let item = elements[row.index()];
                        for (attr, show_column) in self.attributes.iter().zip(show_columns.iter()) {
                            if !*show_column {
                                continue;
                            }
                            let attr_type = attr.get_type();
                            row.col(|ui| {
                                ui_attribute_value(
                                    ui,
                                    state,
                                    &attr_type,
                                    attr.get_value(item).as_ref(),
                                    true,
                                    if attr_type.is_number() {
                                        Some((0.0, 28.0))
                                    } else {
                                        None
                                    },
                                );
                            });
                        }
                    });
                });

            if let Some(clicked) = clicked {
                *clicked_attribute = Some(clicked.into());
            }
        }
    }
}

type MultipleVecTableRow<'a, const V_COUNT: usize> = (bool, [[Option<AttributeValue>; 3]; V_COUNT]);
struct MultipleVecTable<'a, T, const V_COUNT: usize, const E_COUNT: usize> {
    variants: Option<[&'a str; V_COUNT]>,
    elements: [(&'a str, [T; V_COUNT]); E_COUNT],
    table_data: Option<[MultipleVecTableRow<'a, V_COUNT>; E_COUNT]>,
}
impl<'a, T, const E_COUNT: usize> MultipleVecTable<'a, T, 1, E_COUNT> {
    fn single(elements: [(&'a str, T); E_COUNT]) -> Self {
        Self {
            variants: None,
            elements: elements.map(|(label, element)| (label, [element])),
            table_data: None,
        }
    }
}
impl<'a, T, const V_COUNT: usize, const E_COUNT: usize> MultipleVecTable<'a, T, V_COUNT, E_COUNT> {
    fn new(
        variants: Option<[&'a str; V_COUNT]>,
        elements: [(&'a str, [T; V_COUNT]); E_COUNT],
    ) -> Self {
        Self {
            variants,
            elements,
            table_data: None,
        }
    }

    fn update<S>(&mut self, attribute_filter: &AttributeFilter, data: Option<&S>) -> bool
    where
        T: GetAttributeValue<S>,
    {
        let table_data: [(bool, [[Option<AttributeValue>; 3]; V_COUNT]); E_COUNT] =
            self.elements.map(|(_, elements)| {
                let values: [[Option<AttributeValue>; 3]; V_COUNT] = elements.map(|element| {
                    data.and_then(|data| element.get_value(data))
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
            let mut clicked = None;

            TableBuilder::new(ui)
                .columns(Column::auto_with_initial_suggestion(0.0), 1 + 4 * V_COUNT)
                .cell_layout(Layout::left_to_right(Align::Center))
                .striped(true)
                .vscroll(false)
                .header(20.0, |mut row| {
                    for _ in 0..(V_COUNT + 1) {
                        row.col(|_| {});
                    }
                    if let Some(variants) = self.variants {
                        for variant in variants {
                            for axis in ["x", "y", "z"] {
                                row.col(|ui| {
                                    tabel_cell_aligned(ui, Align::RIGHT, |ui| {
                                        ui.strong(format!("{} {}", variant, axis));
                                    });
                                });
                            }
                        }
                    } else {
                        for _ in 0..V_COUNT {
                            for axis in ["x", "y", "z"] {
                                row.col(|ui| {
                                    tabel_cell_aligned(ui, Align::RIGHT, |ui| {
                                        ui.strong(axis);
                                    });
                                });
                            }
                        }
                    }
                })
                .body(|body| {
                    let index_map: Vec<usize> = rows
                        .iter()
                        .enumerate()
                        .filter_map(|(i, (s, _))| if *s { Some(i) } else { None })
                        .collect();

                    body.rows(20.0, index_map.len(), |mut row| {
                        let i = index_map[row.index()];
                        let values = &rows[i].1;
                        let (label, elements) = &self.elements[i];

                        if let Some(variants) = self.variants {
                            row.col(|ui| {
                                ui.label(*label);
                            });
                            for (variant, element) in variants.iter().zip(elements.iter()) {
                                row.col(|ui| {
                                    attribute_detail_button(
                                        ui,
                                        &element,
                                        &mut clicked,
                                        Some(variant),
                                    );
                                });
                            }
                        } else {
                            for element in elements.iter() {
                                row.col(|ui| {
                                    attribute_detail_button(ui, &element, &mut clicked, None);
                                });
                            }
                            row.col(|ui| {
                                ui.label(*label);
                            });
                        }

                        for (vec, element) in values.iter().zip(elements.iter()) {
                            let attr_type = match element.get_type() {
                                AttributeType::VecInt => AttributeType::Int,
                                AttributeType::VecFloat => AttributeType::Float,
                                _ => AttributeType::String,
                            };
                            for v in vec {
                                row.col(|ui| {
                                    ui_attribute_value(
                                        ui,
                                        state,
                                        &attr_type,
                                        v.as_ref(),
                                        true,
                                        None,
                                    );
                                });
                            }
                        }
                    });
                });

            if let Some(clicked) = clicked {
                *clicked_attribute = Some((*clicked).into());
            }
        }
    }
}

fn tabel_cell_aligned(ui: &mut Ui, halign: Align, add_contents: impl FnOnce(&mut Ui)) {
    ui.with_layout(Layout::top_down(halign), |ui| {
        ui.horizontal(add_contents);
    });
}

fn count_true<'a, I>(iter: I) -> usize
where
    I: Iterator<Item = &'a bool>,
{
    iter.map(|s| *s as usize).sum()
}
