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
        if data.is_err() {
            ui.collapsing("Error", |ui| {
                ui.label(data.unwrap_err().to_string());
            });
            return;
        }
        let data = data.unwrap();

        if let Some(name) = &data.name {
            ui.heading(name);
        }

        egui::Grid::new("definition_detail_table")
            .num_columns(2)
            .spacing([10.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                add_attr_row(ui, "name", data.name.clone());
                add_attr_row(ui, "category", data.category);
                add_attr_row(ui, "type", data.definition_type);
                add_attr_row(ui, "mass", data.mass);
                add_attr_row(ui, "value", data.value);
                add_attr_row(ui, "flags", data.flags);
                add_attr_row(ui, "tags", data.tags.clone());

                add_attr_row_if_some(
                    ui,
                    "phys_collision_dampen",
                    data.phys_collision_dampen.clone(),
                );
                add_attr_row_if_some(
                    ui,
                    "audio_filename_start",
                    data.audio_filename_start.clone(),
                );
                add_attr_row_if_some(ui, "audio_filename_loop", data.audio_filename_loop.clone());
                add_attr_row_if_some(ui, "audio_filename_end", data.audio_filename_end.clone());
                add_attr_row_if_some(
                    ui,
                    "audio_filename_start_b",
                    data.audio_filename_start_b.clone(),
                );
                add_attr_row_if_some(
                    ui,
                    "audio_filename_loop_b",
                    data.audio_filename_loop_b.clone(),
                );
                add_attr_row_if_some(
                    ui,
                    "audio_filename_end_b",
                    data.audio_filename_end_b.clone(),
                );
                add_attr_row_if_some(ui, "audio_gain", data.audio_gain.clone());
                add_attr_row_if_some(ui, "mesh_data_name", data.mesh_data_name.clone());
                add_attr_row_if_some(ui, "mesh_0_name", data.mesh_0_name.clone());
                add_attr_row_if_some(ui, "mesh_1_name", data.mesh_1_name.clone());
                add_attr_row_if_some(ui, "mesh_2_name", data.mesh_2_name.clone());
                add_attr_row_if_some(
                    ui,
                    "mesh_editor_only_name",
                    data.mesh_editor_only_name.clone(),
                );
                add_attr_row_if_some(ui, "block_type", data.block_type.clone());
                add_attr_row_if_some(ui, "child_name", data.child_name.clone());
                add_attr_row_if_some(ui, "extender_name", data.extender_name.clone());
                add_attr_row_if_some(ui, "constraint_type", data.constraint_type);
                add_attr_row_if_some(ui, "constraint_axis", data.constraint_axis);
                add_attr_row_if_some(
                    ui,
                    "constraint_range_of_motion",
                    data.constraint_range_of_motion,
                );
                add_attr_row_if_some(ui, "max_motor_force", data.max_motor_force);
                add_attr_row_if_some(ui, "max_motor_speed", data.max_motor_speed);
                add_attr_row_if_some(ui, "cable_radius", data.cable_radius);
                add_attr_row_if_some(ui, "cable_length", data.cable_length);
                add_attr_row_if_some(ui, "seat_type", data.seat_type);
                add_attr_row_if_some(ui, "seat_pose", data.seat_pose);
                add_attr_row_if_some(ui, "seat_health_per_sec", data.seat_health_per_sec);
                add_attr_row_if_some(ui, "buoy_radius", data.buoy_radius);
                add_attr_row_if_some(ui, "buoy_factor", data.buoy_factor);
                add_attr_row_if_some(ui, "buoy_force", data.buoy_force);
                add_attr_row_if_some(ui, "force_emitter_max_force", data.force_emitter_max_force);
                add_attr_row_if_some(
                    ui,
                    "force_emitter_max_vector",
                    data.force_emitter_max_vector,
                );
                add_attr_row_if_some(
                    ui,
                    "force_emitter_default_pitch",
                    data.force_emitter_default_pitch,
                );
                add_attr_row_if_some(
                    ui,
                    "force_emitter_blade_height",
                    data.force_emitter_blade_height,
                );
                add_attr_row_if_some(
                    ui,
                    "force_emitter_rotation_speed",
                    data.force_emitter_rotation_speed,
                );
                add_attr_row_if_some(
                    ui,
                    "force_emitter_blade_physics_length",
                    data.force_emitter_blade_physics_length,
                );
                add_attr_row_if_some(
                    ui,
                    "force_emitter_blade_efficiency",
                    data.force_emitter_blade_efficiency,
                );
                add_attr_row_if_some(
                    ui,
                    "force_emitter_efficiency",
                    data.force_emitter_efficiency,
                );
                add_attr_row_if_some(ui, "engine_max_force", data.engine_max_force);
                add_attr_row_if_some(
                    ui,
                    "engine_frictionless_force",
                    data.engine_frictionless_force,
                );
                add_attr_row_if_some(ui, "trans_conn_type", data.trans_conn_type);
                add_attr_row_if_some(ui, "trans_type", data.trans_type);
                add_attr_row_if_some(ui, "wheel_radius", data.wheel_radius);
                add_attr_row_if_some(ui, "wheel_width", data.wheel_width);
                add_attr_row_if_some(ui, "wheel_wishbone_length", data.wheel_wishbone_length);
                add_attr_row_if_some(ui, "wheel_suspension_height", data.wheel_suspension_height);
                add_attr_row_if_some(ui, "wheel_wishbone_margin", data.wheel_wishbone_margin);
                add_attr_row_if_some(ui, "wheel_suspension_offset", data.wheel_suspension_offset);
                add_attr_row_if_some(ui, "wheel_wishbone_offset", data.wheel_wishbone_offset);
                add_attr_row_if_some(ui, "wheel_type", data.wheel_type);
                add_attr_row_if_some(ui, "button_type", data.button_type);
                add_attr_row_if_some(ui, "light_intensity", data.light_intensity);
                add_attr_row_if_some(ui, "light_range", data.light_range);
                add_attr_row_if_some(ui, "light_ies_map", data.light_ies_map.clone());
                add_attr_row_if_some(ui, "light_fov", data.light_fov);
                add_attr_row_if_some(ui, "light_type", data.light_type);
                add_attr_row_if_some(ui, "door_lower_limit", data.door_lower_limit);
                add_attr_row_if_some(ui, "door_upper_limit", data.door_upper_limit);
                add_attr_row_if_some(ui, "door_flipped", data.door_flipped);
                add_attr_row_if_some(ui, "custom_door_type", data.custom_door_type);
                add_attr_row_if_some(ui, "door_side_dist", data.door_side_dist);
                add_attr_row_if_some(ui, "door_up_dist", data.door_up_dist);
                add_attr_row_if_some(ui, "dynamic_min_rotation", data.dynamic_min_rotation);
                add_attr_row_if_some(ui, "dynamic_max_rotation", data.dynamic_max_rotation);
                add_attr_row_if_some(ui, "logic_gate_type", data.logic_gate_type);
                add_attr_row_if_some(ui, "logic_gate_subtype", data.logic_gate_subtype);
                add_attr_row_if_some(ui, "indicator_type", data.indicator_type);
                add_attr_row_if_some(ui, "connector_type", data.connector_type);
                add_attr_row_if_some(ui, "magnet_force", data.magnet_force);
                add_attr_row_if_some(ui, "gyro_type", data.gyro_type);
                add_attr_row_if_some(ui, "reward_tier", data.reward_tier);
                add_attr_row_if_some(ui, "revision", data.revision);
                add_attr_row_if_some(ui, "rudder_surface_area", data.rudder_surface_area);
                add_attr_row_if_some(ui, "pump_pressure", data.pump_pressure);
                add_attr_row_if_some(ui, "m_pump_pressure", data.m_pump_pressure);
                add_attr_row_if_some(ui, "water_component_type", data.water_component_type);
                add_attr_row_if_some(ui, "torque_component_type", data.torque_component_type);
                add_attr_row_if_some(
                    ui,
                    "jet_engine_component_type",
                    data.jet_engine_component_type,
                );
                add_attr_row_if_some(ui, "particle_speed", data.particle_speed);
                add_attr_row_if_some(ui, "inventory_type", data.inventory_type);
                add_attr_row_if_some(
                    ui,
                    "inventory_default_outfit",
                    data.inventory_default_outfit,
                );
                add_attr_row_if_some(ui, "inventory_class", data.inventory_class);
                add_attr_row_if_some(ui, "inventory_default_item", data.inventory_default_item);
                add_attr_row_if_some(ui, "electric_type", data.electric_type);
                add_attr_row_if_some(
                    ui,
                    "electric_charge_capacity",
                    data.electric_charge_capacity,
                );
                add_attr_row_if_some(ui, "electric_magnitude", data.electric_magnitude);
                add_attr_row_if_some(ui, "composite_type", data.composite_type);
                add_attr_row_if_some(ui, "camera_fov_min", data.camera_fov_min);
                add_attr_row_if_some(ui, "camera_fov_max", data.camera_fov_max);
                add_attr_row_if_some(ui, "monitor_border", data.monitor_border);
                add_attr_row_if_some(ui, "monitor_inset", data.monitor_inset);
                add_attr_row_if_some(ui, "weapon_type", data.weapon_type);
                add_attr_row_if_some(ui, "weapon_class", data.weapon_class);
                add_attr_row_if_some(ui, "weapon_belt_type", data.weapon_belt_type);
                add_attr_row_if_some(ui, "weapon_ammo_capacity", data.weapon_ammo_capacity);
                add_attr_row_if_some(ui, "weapon_ammo_feed", data.weapon_ammo_feed);
                add_attr_row_if_some(
                    ui,
                    "weapon_barrel_length_voxels",
                    data.weapon_barrel_length_voxels,
                );
                add_attr_row_if_some(ui, "rx_range", data.rx_range);
                add_attr_row_if_some(ui, "rx_length", data.rx_length);
                add_attr_row_if_some(ui, "rocket_type", data.rocket_type);
                add_attr_row_if_some(ui, "radar_range", data.radar_range);
                add_attr_row_if_some(ui, "radar_speed", data.radar_speed);
                add_attr_row_if_some(ui, "engine_module_type", data.engine_module_type);
                add_attr_row_if_some(ui, "steam_component_type", data.steam_component_type);
                add_attr_row_if_some(
                    ui,
                    "steam_component_capacity",
                    data.steam_component_capacity,
                );
                add_attr_row_if_some(ui, "nuclear_component_type", data.nuclear_component_type);
                add_attr_row_if_some(ui, "radar_type", data.radar_type);
                add_attr_row_if_some(ui, "piston_len", data.piston_len);
                add_attr_row_if_some(ui, "piston_cam", data.piston_cam);
                add_attr_row_if_some(
                    ui,
                    "data_logger_component_type",
                    data.data_logger_component_type,
                );
                add_attr_row_if_some(ui, "metadata_component_type", data.metadata_component_type);
                add_attr_row_if_some(ui, "oil_component_type", data.oil_component_type);
                add_attr_row_if_some(ui, "tool_type", data.tool_type);
            });
    }
}

fn add_attr_row(ui: &mut egui::Ui, label: &str, value: Option<impl Debug>) {
    ui.label(label);
    if let Some(val) = value {
        ui.label(format!("{:?}", val));
    } else {
        ui.weak("Not defined");
    }
    ui.end_row();
}

fn add_attr_row_if_some(ui: &mut egui::Ui, label: &str, value: Option<impl Debug>) {
    if value.is_some() {
        add_attr_row(ui, label, value);
    }
}
