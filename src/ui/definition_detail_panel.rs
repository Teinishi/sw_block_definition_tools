use super::{
    definitions_store::DefinitionPointer, ui_attribute_value, utils, AttributeDetailWindow, State,
};
use crate::sw_block_definition::{
    AttributeSpecifier, AttributeValue, BuoyancySurface, BuoyancySurfaceAttribute, Coupling,
    CouplingAttribute, Definition, DefinitionAttribute, GetAttributeValue, IsDefault,
    JetEngineConnectionAttribute, LogicNode, LogicNodeAttribute, RewardPropertiesAttribute,
    SfxDataAttribute, SfxLayer, SfxLayerAttribute, Surface, SurfaceAttribute,
    TooltipPropertiesAttribute, Voxel, VoxelAttribute,
};
use egui::{Align, Button, CollapsingHeader, Layout, RichText, Sides, Ui};
use egui_extras::{Column, TableBuilder};
use strum::{EnumCount, VariantArray};

struct AttributeFilter {
    show_all: bool,
    hide_default: bool,
}

impl AttributeFilter {
    fn from_state(state: &State) -> Self {
        Self {
            show_all: state.show_all,
            hide_default: state.hide_default,
        }
    }

    fn check(&self, value: &Option<AttributeValue>) -> bool {
        let is_default = value.as_ref().is_none_or(|v| v.is_default());
        (self.show_all || value.is_some()) && !(self.hide_default && is_default)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct DefinitionDetailPanel;

impl DefinitionDetailPanel {
    pub fn ui(
        &mut self,
        ui: &mut Ui,
        state: &mut State,
        definition: DefinitionPointer,
    ) -> Option<AttributeDetailWindow> {
        let attribute_filter = AttributeFilter::from_state(state);
        let mut new_window = None;

        if let Ok(mut definition) = definition.lock() {
            let filename = definition.filename();
            let path = definition.path().clone();
            let mut refresh = false;

            match definition.load_data() {
                Some(Ok(data)) => {
                    Sides::new().show(
                        ui,
                        |ui| {
                            if let Some(name) = &data.name {
                                ui.heading(name);
                                ui.add_space(10.0);
                            }
                            ui.weak(filename);

                            #[cfg(not(target_arch = "wasm32"))]
                            {
                                ui.add_space(10.0);
                                if ui.button("Open").clicked() {
                                    let _ = open::that(path);
                                }
                            }
                        },
                        |ui| {
                            if ui
                                .add_sized(egui::vec2(20.0, 20.0), Button::new("\u{1F503}"))
                                .clicked()
                            {
                                refresh = true;
                            }
                        },
                    );

                    ui.separator();

                    if let Some(clicked_attribute) =
                        ui_definition_detail(ui, state, &data, &attribute_filter)
                    {
                        new_window = Some(AttributeDetailWindow::new(
                            clicked_attribute,
                            state.hide_default,
                        ))
                    }
                }
                Some(Err(err)) => {
                    ui.collapsing("Error", |ui| {
                        ui.label(err.to_string());
                    });
                }
                None => {}
            }

            if refresh {
                definition.unload();
            }
        }
        new_window
    }
}

fn ui_definition_detail(
    ui: &mut Ui,
    state: &mut State,
    data: &Definition,
    attribute_filter: &AttributeFilter,
) -> Option<AttributeSpecifier> {
    let mut clicked_attribute = None;

    // <definition> の属性リスト
    let mut list = AttributeList::new(&[
        DefinitionAttribute::Name,
        DefinitionAttribute::Category,
        DefinitionAttribute::Type,
        DefinitionAttribute::Mass,
        DefinitionAttribute::Value,
        DefinitionAttribute::Flags,
        DefinitionAttribute::Tags,
        DefinitionAttribute::PhysCollisionDampen,
        DefinitionAttribute::AudioFilenameStart,
        DefinitionAttribute::AudioFilenameLoop,
        DefinitionAttribute::AudioFilenameEnd,
        DefinitionAttribute::AudioFilenameStartB,
        DefinitionAttribute::AudioFilenameLoopB,
        DefinitionAttribute::AudioFilenameEndB,
        DefinitionAttribute::AudioGain,
        DefinitionAttribute::MeshDataName,
        DefinitionAttribute::Mesh0Name,
        DefinitionAttribute::Mesh1Name,
        DefinitionAttribute::Mesh2Name,
        DefinitionAttribute::MeshEditorOnlyName,
        DefinitionAttribute::BlockType,
        DefinitionAttribute::ChildName,
        DefinitionAttribute::ExtenderName,
        DefinitionAttribute::MaxMotorForce,
        DefinitionAttribute::MaxMotorSpeed,
        DefinitionAttribute::CableRadius,
        DefinitionAttribute::CableLength,
        DefinitionAttribute::BuoyRadius,
        DefinitionAttribute::BuoyFactor,
        DefinitionAttribute::BuoyForce,
        DefinitionAttribute::EngineMaxForce,
        DefinitionAttribute::EngineFrictionlessForce,
        DefinitionAttribute::TransConnType,
        DefinitionAttribute::TransType,
        DefinitionAttribute::ButtonType,
        DefinitionAttribute::LogicGateType,
        DefinitionAttribute::LogicGateSubtype,
        DefinitionAttribute::IndicatorType,
        DefinitionAttribute::MagnetForce,
        DefinitionAttribute::GyroType,
        DefinitionAttribute::Revision,
        DefinitionAttribute::RudderSurfaceArea,
        DefinitionAttribute::PumpPressure,
        DefinitionAttribute::MPumpPressure,
        DefinitionAttribute::WaterComponentType,
        DefinitionAttribute::TorqueComponentType,
        DefinitionAttribute::CompositeType,
        DefinitionAttribute::CameraFovMin,
        DefinitionAttribute::CameraFovMax,
        DefinitionAttribute::MonitorBorder,
        DefinitionAttribute::MonitorInset,
        DefinitionAttribute::RxRange,
        DefinitionAttribute::RxLength,
        DefinitionAttribute::RocketType,
        DefinitionAttribute::EngineModuleType,
        DefinitionAttribute::SteamComponentType,
        DefinitionAttribute::SteamComponentCapacity,
        DefinitionAttribute::NuclearComponentType,
        DefinitionAttribute::PistonLen,
        DefinitionAttribute::PistonCam,
        DefinitionAttribute::DataLoggerComponentType,
        DefinitionAttribute::MetadataComponentType,
        DefinitionAttribute::OilComponentType,
        DefinitionAttribute::ToolType,
        DefinitionAttribute::RudderType,
        DefinitionAttribute::VoxelLocationChild,
        DefinitionAttribute::CompartmentSamplePos,
        DefinitionAttribute::ForceDir,
        DefinitionAttribute::MagnetOffset,
        DefinitionAttribute::RopeHookOffset,
    ]);
    if list.update(attribute_filter, Some(data)) {
        CollapsingPanel::new("Definition attributes")
            .default_open(true)
            .ui(ui, |ui| {
                list.ui(ui, state, &mut clicked_attribute);
            });
    }

    // sfx_datas のリスト
    if let Some(sfx_datas) = data.sfx_datas.last() {
        for (i, item) in sfx_datas.sfx_data.iter().enumerate() {
            let mut attribute_list = AttributeList::new(SfxDataAttribute::VARIANTS);
            let mut layers_table: ElementsTable<
                '_,
                SfxLayerAttribute,
                SfxLayer,
                { SfxLayerAttribute::COUNT },
            > = ElementsTable::new(SfxLayerAttribute::VARIANTS.try_into().unwrap());

            let mut show = attribute_list.update(attribute_filter, Some(item));

            if layers_table.update(
                attribute_filter,
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

    // surfaces のリスト
    let mut table: ElementsTable<'_, SurfaceAttribute, Surface, { SurfaceAttribute::COUNT }> =
        ElementsTable::new(SurfaceAttribute::VARIANTS.try_into().unwrap());
    if table.update(
        attribute_filter,
        data.surfaces.last().map(|s| s.surface.as_slice()),
    ) {
        CollapsingPanel::new(&format!("Surfaces ({})", table.len().unwrap_or(0))).ui(ui, |ui| {
            table.ui(ui, state, &mut clicked_attribute);
        });
    }

    // buoyancy_surfaces のリスト
    let mut table: ElementsTable<
        '_,
        BuoyancySurfaceAttribute,
        BuoyancySurface,
        { BuoyancySurfaceAttribute::COUNT },
    > = ElementsTable::new(BuoyancySurfaceAttribute::VARIANTS.try_into().unwrap());
    if table.update(
        attribute_filter,
        data.buoyancy_surfaces.last().map(|s| s.surface.as_slice()),
    ) {
        CollapsingPanel::new(&format!("Buoyancy surfaces ({})", table.len().unwrap_or(0))).ui(
            ui,
            |ui| {
                table.ui(ui, state, &mut clicked_attribute);
            },
        );
    }

    // logic_nodes のリスト
    let mut table: ElementsTable<'_, LogicNodeAttribute, LogicNode, { LogicNodeAttribute::COUNT }> =
        ElementsTable::new(LogicNodeAttribute::VARIANTS.try_into().unwrap());
    if table.update(
        attribute_filter,
        data.logic_nodes.last().map(|l| l.logic_node.as_slice()),
    ) {
        CollapsingPanel::new(&format!("Logic nodes ({})", table.len().unwrap_or(0))).ui(ui, |ui| {
            table.ui(ui, state, &mut clicked_attribute);
        });
    }

    // couplings のリスト
    let mut table: ElementsTable<'_, CouplingAttribute, Coupling, { CouplingAttribute::COUNT }> =
        ElementsTable::new(CouplingAttribute::VARIANTS.try_into().unwrap());
    if table.update(
        attribute_filter,
        data.couplings.last().map(|c| c.coupling.as_slice()),
    ) {
        CollapsingPanel::new(&format!("Couplings ({})", table.len().unwrap_or(0))).ui(ui, |ui| {
            table.ui(ui, state, &mut clicked_attribute);
        });
    }

    // voxels のリスト
    let mut table: ElementsTable<'_, VoxelAttribute, Voxel, { VoxelAttribute::COUNT }> =
        ElementsTable::new(VoxelAttribute::VARIANTS.try_into().unwrap());
    if table.update(
        attribute_filter,
        data.voxels.last().map(|v| v.voxel.as_slice()),
    ) {
        CollapsingPanel::new(&format!("Voxels ({})", table.len().unwrap_or(0))).ui(ui, |ui| {
            table.ui(ui, state, &mut clicked_attribute);
        });
    }

    // min, max ベクトル系
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
    if table.update(attribute_filter, Some(data)) {
        CollapsingPanel::new("Bouding boxes").ui(ui, |ui| {
            table.ui(ui, state, &mut clicked_attribute);
        });
    }

    // constraint 系
    let mut list = AttributeList::new(&[
        DefinitionAttribute::ConstraintType,
        DefinitionAttribute::ConstraintAxis,
        DefinitionAttribute::ConstraintRangeOfMotion,
        DefinitionAttribute::ConstraintPosParent,
        DefinitionAttribute::ConstraintPosChild,
    ]);
    if list.update(attribute_filter, Some(data)) {
        CollapsingPanel::new("Constraint").ui(ui, |ui| {
            list.ui(ui, state, &mut clicked_attribute);
        });
    }

    // seat系
    let mut list = AttributeList::new(&[
        DefinitionAttribute::SeatType,
        DefinitionAttribute::SeatPose,
        DefinitionAttribute::SeatHealthPerSec,
        DefinitionAttribute::SeatOffset,
        DefinitionAttribute::SeatFront,
        DefinitionAttribute::SeatUp,
        DefinitionAttribute::SeatCamera,
        DefinitionAttribute::SeatRender,
        DefinitionAttribute::SeatExitPosition,
    ]);
    if list.update(attribute_filter, Some(data)) {
        CollapsingPanel::new("Seat").ui(ui, |ui| {
            list.ui(ui, state, &mut clicked_attribute);
        });
    }

    // force_emitter 系
    let mut list = AttributeList::new(&[
        DefinitionAttribute::ForceEmitterMaxForce,
        DefinitionAttribute::ForceEmitterMaxVector,
        DefinitionAttribute::ForceEmitterDefaultPitch,
        DefinitionAttribute::ForceEmitterBladeHeight,
        DefinitionAttribute::ForceEmitterRotationSpeed,
        DefinitionAttribute::ForceEmitterBladePhysicsLength,
        DefinitionAttribute::ForceEmitterBladeEfficiency,
        DefinitionAttribute::ForceEmitterEfficiency,
    ]);
    if list.update(attribute_filter, Some(data)) {
        CollapsingPanel::new("Force emitter").ui(ui, |ui| {
            list.ui(ui, state, &mut clicked_attribute);
        });
    }

    // wheel系
    let mut list = AttributeList::new(&[
        DefinitionAttribute::WheelRadius,
        DefinitionAttribute::WheelWidth,
        DefinitionAttribute::WheelWishboneLength,
        DefinitionAttribute::WheelSuspensionHeight,
        DefinitionAttribute::WheelWishboneMargin,
        DefinitionAttribute::WheelSuspensionOffset,
        DefinitionAttribute::WheelWishboneOffset,
        DefinitionAttribute::WheelType,
    ]);
    if list.update(attribute_filter, Some(data)) {
        CollapsingPanel::new("Wheel").ui(ui, |ui| {
            list.ui(ui, state, &mut clicked_attribute);
        });
    }

    // light系
    let mut list = AttributeList::new(&[
        DefinitionAttribute::LightIntensity,
        DefinitionAttribute::LightRange,
        DefinitionAttribute::LightIesMap,
        DefinitionAttribute::LightFov,
        DefinitionAttribute::LightType,
        DefinitionAttribute::LightPosition,
        DefinitionAttribute::LightForward,
        DefinitionAttribute::LightColor,
    ]);
    if list.update(attribute_filter, Some(data)) {
        CollapsingPanel::new("Light").ui(ui, |ui| {
            list.ui(ui, state, &mut clicked_attribute);
        });
    }

    // door系
    let mut list = AttributeList::new(&[
        DefinitionAttribute::DoorLowerLimit,
        DefinitionAttribute::DoorUpperLimit,
        DefinitionAttribute::DoorFlipped,
        DefinitionAttribute::CustomDoorType,
        DefinitionAttribute::DoorSideDist,
        DefinitionAttribute::DoorUpDist,
        DefinitionAttribute::DoorSize,
        DefinitionAttribute::DoorNormal,
        DefinitionAttribute::DoorSide,
        DefinitionAttribute::DoorUp,
        DefinitionAttribute::DoorBasePos,
    ]);
    if list.update(attribute_filter, Some(data)) {
        CollapsingPanel::new("Door").ui(ui, |ui| {
            list.ui(ui, state, &mut clicked_attribute);
        });
    }

    // dynamic 系
    let mut list = AttributeList::new(&[
        DefinitionAttribute::DynamicMinRotation,
        DefinitionAttribute::DynamicMaxRotation,
        DefinitionAttribute::DynamicBodyPosition,
        DefinitionAttribute::DynamicRotationAxes,
        DefinitionAttribute::DynamicSideAxis,
    ]);
    if list.update(attribute_filter, Some(data)) {
        CollapsingPanel::new("Dynamic").ui(ui, |ui| {
            list.ui(ui, state, &mut clicked_attribute);
        });
    }

    // reward 系
    let mut list1 = AttributeList::new(&[DefinitionAttribute::RewardTier]);
    let mut list2 = AttributeList::new(RewardPropertiesAttribute::VARIANTS);
    let show1 = list1.update(attribute_filter, Some(data));
    let show2 = list2.update(attribute_filter, data.reward_properties.last());
    if show1 || show2 {
        CollapsingPanel::new("Reward").ui(ui, |ui| {
            ui.push_id("reward_1", |ui| {
                list1.ui(ui, state, &mut clicked_attribute);
            });
            ui.push_id("reward_2", |ui| {
                list2.ui(ui, state, &mut clicked_attribute);
            });
        });
    }

    // jet_engine系
    let mut list = AttributeList::new(&[DefinitionAttribute::JetEngineComponentType]);
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
    let show1 = list.update(attribute_filter, Some(data));
    let show2 = table.update(attribute_filter, Some(data));
    if show1 || show2 {
        CollapsingPanel::new("Jet engine connection").ui(ui, |ui| {
            ui.push_id("jet_engine_1", |ui| {
                list.ui(ui, state, &mut clicked_attribute);
            });
            ui.push_id("jet_engine_2", |ui| {
                table.ui(ui, state, &mut clicked_attribute);
            });
        });
    }

    // inventory系
    let mut list = AttributeList::new(&[
        DefinitionAttribute::InventoryType,
        DefinitionAttribute::InventoryDefaultOutfit,
        DefinitionAttribute::InventoryClass,
        DefinitionAttribute::InventoryDefaultItem,
    ]);
    if list.update(attribute_filter, Some(data)) {
        CollapsingPanel::new("Inventory").ui(ui, |ui| {
            list.ui(ui, state, &mut clicked_attribute);
        });
    }

    // electric系
    let mut list = AttributeList::new(&[
        DefinitionAttribute::ElectricType,
        DefinitionAttribute::ElectricChargeCapacity,
        DefinitionAttribute::ElectricMagnitude,
    ]);
    if list.update(attribute_filter, Some(data)) {
        CollapsingPanel::new("Electric").ui(ui, |ui| {
            list.ui(ui, state, &mut clicked_attribute);
        });
    }

    // radar系
    let mut list = AttributeList::new(&[
        DefinitionAttribute::RadarType,
        DefinitionAttribute::RadarRange,
        DefinitionAttribute::RadarSpeed,
    ]);
    if list.update(attribute_filter, Some(data)) {
        CollapsingPanel::new("Radar").ui(ui, |ui| {
            list.ui(ui, state, &mut clicked_attribute);
        });
    }

    // connector系
    let mut list = AttributeList::new(&[
        DefinitionAttribute::ConnectorType,
        DefinitionAttribute::ConnectorAxis,
        DefinitionAttribute::ConnectorUp,
    ]);
    if list.update(attribute_filter, Some(data)) {
        CollapsingPanel::new("Connector").ui(ui, |ui| {
            list.ui(ui, state, &mut clicked_attribute);
        });
    }

    // tooltip_properties
    let mut list = AttributeList::new(TooltipPropertiesAttribute::VARIANTS);
    if list.update(attribute_filter, data.tooltip_properties.last()) {
        CollapsingPanel::new("Tooltip properties")
            .default_open(true)
            .ui(ui, |ui| {
                list.ui(ui, state, &mut clicked_attribute);
            });
    }

    // particle系
    let mut list = AttributeList::new(&[
        DefinitionAttribute::ParticleSpeed,
        DefinitionAttribute::ParticleDirection,
        DefinitionAttribute::ParticleOffset,
        DefinitionAttribute::ParticleBounds,
    ]);
    if list.update(attribute_filter, Some(data)) {
        CollapsingPanel::new("Particle").ui(ui, |ui| {
            list.ui(ui, state, &mut clicked_attribute);
        });
    }

    // weapon系
    let mut list = AttributeList::new(&[
        DefinitionAttribute::WeaponType,
        DefinitionAttribute::WeaponClass,
        DefinitionAttribute::WeaponBeltType,
        DefinitionAttribute::WeaponAmmoCapacity,
        DefinitionAttribute::WeaponAmmoFeed,
        DefinitionAttribute::WeaponBarrelLengthVoxels,
        DefinitionAttribute::WeaponBreechPosition,
        DefinitionAttribute::WeaponBreechNormal,
        DefinitionAttribute::WeaponCartPosition,
        DefinitionAttribute::WeaponCartVelocity,
    ]);
    if list.update(attribute_filter, Some(data)) {
        CollapsingPanel::new("Weapon").ui(ui, |ui| {
            list.ui(ui, state, &mut clicked_attribute);
        });
    }

    clicked_attribute
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
                    utils::count_true(show_columns.iter()),
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

type MultipleVecTableRow<'a, const V_COUNT: usize> = (bool, [Option<AttributeValue>; V_COUNT]);
struct MultipleVecTable<'a, T, const V_COUNT: usize, const E_COUNT: usize> {
    variants: Option<[&'a str; V_COUNT]>,
    elements: [(&'a str, [T; V_COUNT]); E_COUNT],
    table_data: Option<[MultipleVecTableRow<'a, V_COUNT>; E_COUNT]>,
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
        let table_data: [(bool, [Option<AttributeValue>; V_COUNT]); E_COUNT] =
            self.elements.map(|(_, elements)| {
                let values: [Option<AttributeValue>; V_COUNT] =
                    elements.map(|element| data.and_then(|data| element.get_value(data)));

                let show_row = values.iter().any(|vec| attribute_filter.check(vec));

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
        if self.table_data.is_some() {
            let mut clicked = None;

            let header_content = |mut row: egui_extras::TableRow<'_, '_>| {
                if let Some(variants) = self.variants {
                    for _ in 0..(V_COUNT + 1) {
                        row.col(|_| {});
                    }
                    for variant in variants {
                        row.col(|ui| {
                            ui.strong(variant);
                        });
                    }
                }
            };

            let body_content = |body: egui_extras::TableBody<'_>| {
                if let Some(rows) = &self.table_data {
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
                            row.col(|ui| {
                                ui_attribute_value(
                                    ui,
                                    state,
                                    &element.get_type(),
                                    vec.as_ref(),
                                    true,
                                    None,
                                );
                            });
                        }
                    });
                }
            };

            let table_builder = TableBuilder::new(ui)
                .columns(Column::auto_with_initial_suggestion(0.0), 1 + 2 * V_COUNT)
                .cell_layout(Layout::left_to_right(Align::Center))
                .striped(true)
                .vscroll(false);

            if self.variants.is_some() {
                table_builder
                    .header(20.0, header_content)
                    .body(body_content);
            } else {
                table_builder.body(body_content);
            }

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
