use super::State;
use std::fmt::Debug;

#[derive(Default)]
pub struct DefinitionDetailPanel {}

impl DefinitionDetailPanel {
    pub fn ui(&mut self, ui: &mut egui::Ui, state: &mut State) {
        let definition = state.selected_definition();
        if definition.is_none() {
            return;
        }
        let data = definition.unwrap().data();
        if let Err(err) = data {
            ui.collapsing("Error", |ui| {
                ui.label(err.to_string());
            });
            return;
        }
        let data = data.unwrap();

        if let Some(name) = &data.name {
            ui.heading(name);
        }

        attribute_table_body(
            ui,
            "definition_detail_table",
            state.show_all_attributes(),
            state.hide_default_attributes(),
            [
                ("name", fmt_default(&data.name)),
                ("category", fmt_default(&data.category)),
                ("type", fmt_default(&data.definition_type)),
                ("mass", fmt_default(&data.mass)),
                ("value", fmt_default(&data.value)),
                ("flags", fmt_default(&data.flags)),
                ("tags", fmt_default(&data.tags)),
                (
                    "phys_collision_dampen",
                    fmt_default(&data.phys_collision_dampen),
                ),
                (
                    "audio_filename_start",
                    fmt_default(&data.audio_filename_start),
                ),
                (
                    "audio_filename_loop",
                    fmt_default(&data.audio_filename_loop),
                ),
                ("audio_filename_end", fmt_default(&data.audio_filename_end)),
                (
                    "audio_filename_start_b",
                    fmt_default(&data.audio_filename_start_b),
                ),
                (
                    "audio_filename_loop_b",
                    fmt_default(&data.audio_filename_loop_b),
                ),
                (
                    "audio_filename_end_b",
                    fmt_default(&data.audio_filename_end_b),
                ),
                ("audio_gain", fmt_default(&data.audio_gain)),
                ("mesh_data_name", fmt_default(&data.mesh_data_name)),
                ("mesh_0_name", fmt_default(&data.mesh_0_name)),
                ("mesh_1_name", fmt_default(&data.mesh_1_name)),
                ("mesh_2_name", fmt_default(&data.mesh_2_name)),
                (
                    "mesh_editor_only_name",
                    fmt_default(&data.mesh_editor_only_name),
                ),
                ("block_type", fmt_default(&data.block_type)),
                ("child_name", fmt_default(&data.child_name)),
                ("extender_name", fmt_default(&data.extender_name)),
                ("constraint_type", fmt_default(&data.constraint_type)),
                ("constraint_axis", fmt_default(&data.constraint_axis)),
                (
                    "constraint_range_of_motion",
                    fmt_default(&data.constraint_range_of_motion),
                ),
                ("max_motor_force", fmt_default(&data.max_motor_force)),
                ("max_motor_speed", fmt_default(&data.max_motor_speed)),
                ("cable_radius", fmt_default(&data.cable_radius)),
                ("cable_length", fmt_default(&data.cable_length)),
                ("seat_type", fmt_default(&data.seat_type)),
                ("seat_pose", fmt_default(&data.seat_pose)),
                (
                    "seat_health_per_sec",
                    fmt_default(&data.seat_health_per_sec),
                ),
                ("buoy_radius", fmt_default(&data.buoy_radius)),
                ("buoy_factor", fmt_default(&data.buoy_factor)),
                ("buoy_force", fmt_default(&data.buoy_force)),
                (
                    "force_emitter_max_force",
                    fmt_default(&data.force_emitter_max_force),
                ),
                (
                    "force_emitter_max_vector",
                    fmt_default(&data.force_emitter_max_vector),
                ),
                (
                    "force_emitter_default_pitch",
                    fmt_default(&data.force_emitter_default_pitch),
                ),
                (
                    "force_emitter_blade_height",
                    fmt_default(&data.force_emitter_blade_height),
                ),
                (
                    "force_emitter_rotation_speed",
                    fmt_default(&data.force_emitter_rotation_speed),
                ),
                (
                    "force_emitter_blade_physics_length",
                    fmt_default(&data.force_emitter_blade_physics_length),
                ),
                (
                    "force_emitter_blade_efficiency",
                    fmt_default(&data.force_emitter_blade_efficiency),
                ),
                (
                    "force_emitter_efficiency",
                    fmt_default(&data.force_emitter_efficiency),
                ),
                ("engine_max_force", fmt_default(&data.engine_max_force)),
                (
                    "engine_frictionless_force",
                    fmt_default(&data.engine_frictionless_force),
                ),
                ("trans_conn_type", fmt_default(&data.trans_conn_type)),
                ("trans_type", fmt_default(&data.trans_type)),
                ("wheel_radius", fmt_default(&data.wheel_radius)),
                ("wheel_width", fmt_default(&data.wheel_width)),
                (
                    "wheel_wishbone_length",
                    fmt_default(&data.wheel_wishbone_length),
                ),
                (
                    "wheel_suspension_height",
                    fmt_default(&data.wheel_suspension_height),
                ),
                (
                    "wheel_wishbone_margin",
                    fmt_default(&data.wheel_wishbone_margin),
                ),
                (
                    "wheel_suspension_offset",
                    fmt_default(&data.wheel_suspension_offset),
                ),
                (
                    "wheel_wishbone_offset",
                    fmt_default(&data.wheel_wishbone_offset),
                ),
                ("wheel_type", fmt_default(&data.wheel_type)),
                ("button_type", fmt_default(&data.button_type)),
                ("light_intensity", fmt_default(&data.light_intensity)),
                ("light_range", fmt_default(&data.light_range)),
                ("light_ies_map", fmt_default(&data.light_ies_map)),
                ("light_fov", fmt_default(&data.light_fov)),
                ("light_type", fmt_default(&data.light_type)),
                ("door_lower_limit", fmt_default(&data.door_lower_limit)),
                ("door_upper_limit", fmt_default(&data.door_upper_limit)),
                ("door_flipped", fmt_default(&data.door_flipped)),
                ("custom_door_type", fmt_default(&data.custom_door_type)),
                ("door_side_dist", fmt_default(&data.door_side_dist)),
                ("door_up_dist", fmt_default(&data.door_up_dist)),
                (
                    "dynamic_min_rotation",
                    fmt_default(&data.dynamic_min_rotation),
                ),
                (
                    "dynamic_max_rotation",
                    fmt_default(&data.dynamic_max_rotation),
                ),
                ("logic_gate_type", fmt_default(&data.logic_gate_type)),
                ("logic_gate_subtype", fmt_default(&data.logic_gate_subtype)),
                ("indicator_type", fmt_default(&data.indicator_type)),
                ("connector_type", fmt_default(&data.connector_type)),
                ("magnet_force", fmt_default(&data.magnet_force)),
                ("gyro_type", fmt_default(&data.gyro_type)),
                ("reward_tier", fmt_default(&data.reward_tier)),
                ("revision", fmt_default(&data.revision)),
                (
                    "rudder_surface_area",
                    fmt_default(&data.rudder_surface_area),
                ),
                ("pump_pressure", fmt_default(&data.pump_pressure)),
                ("m_pump_pressure", fmt_default(&data.m_pump_pressure)),
                (
                    "water_component_type",
                    fmt_default(&data.water_component_type),
                ),
                (
                    "torque_component_type",
                    fmt_default(&data.torque_component_type),
                ),
                (
                    "jet_engine_component_type",
                    fmt_default(&data.jet_engine_component_type),
                ),
                ("particle_speed", fmt_default(&data.particle_speed)),
                ("inventory_type", fmt_default(&data.inventory_type)),
                (
                    "inventory_default_outfit",
                    fmt_default(&data.inventory_default_outfit),
                ),
                ("inventory_class", fmt_default(&data.inventory_class)),
                (
                    "inventory_default_item",
                    fmt_default(&data.inventory_default_item),
                ),
                ("electric_type", fmt_default(&data.electric_type)),
                (
                    "electric_charge_capacity",
                    fmt_default(&data.electric_charge_capacity),
                ),
                ("electric_magnitude", fmt_default(&data.electric_magnitude)),
                ("composite_type", fmt_default(&data.composite_type)),
                ("camera_fov_min", fmt_default(&data.camera_fov_min)),
                ("camera_fov_max", fmt_default(&data.camera_fov_max)),
                ("monitor_border", fmt_default(&data.monitor_border)),
                ("monitor_inset", fmt_default(&data.monitor_inset)),
                ("weapon_type", fmt_default(&data.weapon_type)),
                ("weapon_class", fmt_default(&data.weapon_class)),
                ("weapon_belt_type", fmt_default(&data.weapon_belt_type)),
                (
                    "weapon_ammo_capacity",
                    fmt_default(&data.weapon_ammo_capacity),
                ),
                ("weapon_ammo_feed", fmt_default(&data.weapon_ammo_feed)),
                (
                    "weapon_barrel_length_voxels",
                    fmt_default(&data.weapon_barrel_length_voxels),
                ),
                ("rx_range", fmt_default(&data.rx_range)),
                ("rx_length", fmt_default(&data.rx_length)),
                ("rocket_type", fmt_default(&data.rocket_type)),
                ("radar_range", fmt_default(&data.radar_range)),
                ("radar_speed", fmt_default(&data.radar_speed)),
                ("engine_module_type", fmt_default(&data.engine_module_type)),
                (
                    "steam_component_type",
                    fmt_default(&data.steam_component_type),
                ),
                (
                    "steam_component_capacity",
                    fmt_default(&data.steam_component_capacity),
                ),
                (
                    "nuclear_component_type",
                    fmt_default(&data.nuclear_component_type),
                ),
                ("radar_type", fmt_default(&data.radar_type)),
                ("piston_len", fmt_default(&data.piston_len)),
                ("piston_cam", fmt_default(&data.piston_cam)),
                (
                    "data_logger_component_type",
                    fmt_default(&data.data_logger_component_type),
                ),
                (
                    "metadata_component_type",
                    fmt_default(&data.metadata_component_type),
                ),
                ("oil_component_type", fmt_default(&data.oil_component_type)),
                ("tool_type", fmt_default(&data.tool_type)),
            ],
        );
    }
}

fn fmt_default<T: Debug + Default + PartialEq>(value: &Option<T>) -> (Option<String>, bool) {
    if let Some(val) = value {
        (Some(format!("{:?}", val)), *val == T::default())
    } else {
        (None, false)
    }
}

fn attribute_table_body<'a>(
    ui: &mut egui::Ui,
    id: &str,
    show_all: bool,
    hide_default: bool,
    items: impl IntoIterator<Item = (&'a str, (Option<String>, bool))>,
) {
    egui::Grid::new(id)
        .num_columns(2)
        .spacing([10.0, 4.0])
        .striped(true)
        .show(ui, |ui| {
            for (label, (value, is_default)) in items {
                if (show_all || value.is_some()) && !(hide_default && is_default) {
                    ui.label(label);
                    if let Some(val) = value {
                        ui.label(val);
                    } else {
                        ui.weak("Not defined");
                    }
                    ui.end_row();
                }
            }
        });
}
