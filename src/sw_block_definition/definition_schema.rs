use serde::{Deserialize, Serialize};

macro_rules! define_vec3 {
    ($name:ident, $type:ty) => {
        #[derive(Serialize, Deserialize, Default, Debug)]
        #[serde(default)]
        pub struct $name {
            #[serde(rename = "@x")]
            pub x: $type,
            #[serde(rename = "@y")]
            pub y: $type,
            #[serde(rename = "@z")]
            pub z: $type,
        }
    };
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename = "definition", default, deny_unknown_fields)]
pub struct Definition {
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "@category")]
    pub category: Option<i32>,
    #[serde(rename = "@type")]
    pub definition_type: Option<i32>,
    #[serde(rename = "@mass")]
    pub mass: Option<f32>,
    #[serde(rename = "@value")]
    pub value: Option<f32>,
    #[serde(rename = "@flags")]
    pub flags: Option<u64>,
    #[serde(rename = "@tags")]
    pub tags: Option<String>,
    #[serde(rename = "@phys_collision_dampen")]
    pub phys_collision_dampen: Option<String>,
    #[serde(rename = "@audio_filename_start")]
    pub audio_filename_start: Option<String>,
    #[serde(rename = "@audio_filename_loop")]
    pub audio_filename_loop: Option<String>,
    #[serde(rename = "@audio_filename_end")]
    pub audio_filename_end: Option<String>,
    #[serde(rename = "@audio_filename_start_b")]
    pub audio_filename_start_b: Option<String>,
    #[serde(rename = "@audio_filename_loop_b")]
    pub audio_filename_loop_b: Option<String>,
    #[serde(rename = "@audio_filename_end_b")]
    pub audio_filename_end_b: Option<String>,
    #[serde(rename = "@audio_gain")]
    pub audio_gain: Option<f32>,
    #[serde(rename = "@mesh_data_name")]
    pub mesh_data_name: Option<String>,
    #[serde(rename = "@mesh_0_name")]
    pub mesh_0_name: Option<String>,
    #[serde(rename = "@mesh_1_name")]
    pub mesh_1_name: Option<String>,
    #[serde(rename = "@mesh_2_name")]
    pub mesh_2_name: Option<String>,
    #[serde(rename = "@mesh_editor_only_name")]
    pub mesh_editor_only_name: Option<String>,
    #[serde(rename = "@block_type")]
    pub block_type: Option<i32>,
    #[serde(rename = "@child_name")]
    pub child_name: Option<String>,
    #[serde(rename = "@extender_name")]
    pub extender_name: Option<String>,
    #[serde(rename = "@constraint_type")]
    pub constraint_type: Option<i32>,
    #[serde(rename = "@constraint_axis")]
    pub constraint_axis: Option<i32>,
    #[serde(rename = "@constraint_range_of_motion")]
    pub constraint_range_of_motion: Option<f32>,
    #[serde(rename = "@max_motor_force")]
    pub max_motor_force: Option<f32>,
    #[serde(rename = "@max_motor_speed")]
    pub max_motor_speed: Option<f32>,
    #[serde(rename = "@cable_radius")]
    pub cable_radius: Option<f32>,
    #[serde(rename = "@cable_length")]
    pub cable_length: Option<f32>,
    #[serde(rename = "@seat_type")]
    pub seat_type: Option<i32>,
    #[serde(rename = "@seat_pose")]
    pub seat_pose: Option<i32>,
    #[serde(rename = "@seat_health_per_sec")]
    pub seat_health_per_sec: Option<i32>,
    #[serde(rename = "@buoy_radius")]
    pub buoy_radius: Option<f32>,
    #[serde(rename = "@buoy_factor")]
    pub buoy_factor: Option<f32>,
    #[serde(rename = "@buoy_force")]
    pub buoy_force: Option<f32>,
    #[serde(rename = "@force_emitter_max_force")]
    pub force_emitter_max_force: Option<f32>,
    #[serde(rename = "@force_emitter_max_vector")]
    pub force_emitter_max_vector: Option<f32>,
    #[serde(rename = "@force_emitter_default_pitch")]
    pub force_emitter_default_pitch: Option<f32>,
    #[serde(rename = "@force_emitter_blade_height")]
    pub force_emitter_blade_height: Option<f32>,
    #[serde(rename = "@force_emitter_rotation_speed")]
    pub force_emitter_rotation_speed: Option<f32>,
    #[serde(rename = "@force_emitter_blade_physics_length")]
    pub force_emitter_blade_physics_length: Option<f32>,
    #[serde(rename = "@force_emitter_blade_efficiency")]
    pub force_emitter_blade_efficiency: Option<f32>,
    #[serde(rename = "@force_emitter_efficiency")]
    pub force_emitter_efficiency: Option<f32>,
    #[serde(rename = "@engine_max_force")]
    pub engine_max_force: Option<f32>,
    #[serde(rename = "@engine_frictionless_force")]
    pub engine_frictionless_force: Option<f32>,
    #[serde(rename = "@trans_conn_type")]
    pub trans_conn_type: Option<i32>,
    #[serde(rename = "@trans_type")]
    pub trans_type: Option<i32>,
    #[serde(rename = "@wheel_radius")]
    pub wheel_radius: Option<f32>,
    #[serde(rename = "@wheel_width")]
    pub wheel_width: Option<f32>,
    #[serde(rename = "@wheel_wishbone_length")]
    pub wheel_wishbone_length: Option<f32>,
    #[serde(rename = "@wheel_suspension_height")]
    pub wheel_suspension_height: Option<f32>,
    #[serde(rename = "@wheel_wishbone_margin")]
    pub wheel_wishbone_margin: Option<f32>,
    #[serde(rename = "@wheel_suspension_offset")]
    pub wheel_suspension_offset: Option<f32>,
    #[serde(rename = "@wheel_wishbone_offset")]
    pub wheel_wishbone_offset: Option<f32>,
    #[serde(rename = "@wheel_type")]
    pub wheel_type: Option<f32>,
    #[serde(rename = "@button_type")]
    pub button_type: Option<i32>,
    #[serde(rename = "@light_intensity")]
    pub light_intensity: Option<f32>,
    #[serde(rename = "@light_range")]
    pub light_range: Option<f32>,
    #[serde(rename = "@light_ies_map")]
    pub light_ies_map: Option<String>,
    #[serde(rename = "@light_fov")]
    pub light_fov: Option<f32>,
    #[serde(rename = "@light_type")]
    pub light_type: Option<i32>,
    #[serde(rename = "@door_lower_limit")]
    pub door_lower_limit: Option<f32>,
    #[serde(rename = "@door_upper_limit")]
    pub door_upper_limit: Option<f32>,
    #[serde(rename = "@door_flipped")]
    pub door_flipped: Option<bool>,
    #[serde(rename = "@custom_door_type")]
    pub custom_door_type: Option<i32>,
    #[serde(rename = "@door_side_dist")]
    pub door_side_dist: Option<i32>,
    #[serde(rename = "@door_up_dist")]
    pub door_up_dist: Option<i32>,
    #[serde(rename = "@dynamic_min_rotation")]
    pub dynamic_min_rotation: Option<f32>,
    #[serde(rename = "@dynamic_max_rotation")]
    pub dynamic_max_rotation: Option<f32>,
    #[serde(rename = "@logic_gate_type")]
    pub logic_gate_type: Option<i32>,
    #[serde(rename = "@logic_gate_subtype")]
    pub logic_gate_subtype: Option<i32>,
    #[serde(rename = "@indicator_type")]
    pub indicator_type: Option<i32>,
    #[serde(rename = "@connector_type")]
    pub connector_type: Option<i32>,
    #[serde(rename = "@magnet_force")]
    pub magnet_force: Option<f32>,
    #[serde(rename = "@gyro_type")]
    pub gyro_type: Option<i32>,
    #[serde(rename = "@reward_tier")]
    pub reward_tier: Option<i32>,
    #[serde(rename = "@revision")]
    pub revision: Option<i32>,
    #[serde(rename = "@rudder_surface_area")]
    pub rudder_surface_area: Option<f32>,
    #[serde(rename = "@pump_pressure")]
    pub pump_pressure: Option<f32>,
    #[serde(rename = "@m_pump_pressure")]
    pub m_pump_pressure: Option<f32>,
    #[serde(rename = "@water_component_type")]
    pub water_component_type: Option<f32>,
    #[serde(rename = "@torque_component_type")]
    pub torque_component_type: Option<i32>,
    #[serde(rename = "@jet_engine_component_type")]
    pub jet_engine_component_type: Option<i32>,
    #[serde(rename = "@particle_speed")]
    pub particle_speed: Option<f32>,
    #[serde(rename = "@inventory_type")]
    pub inventory_type: Option<f32>,
    #[serde(rename = "@inventory_default_outfit")]
    pub inventory_default_outfit: Option<f32>,
    #[serde(rename = "@inventory_class")]
    pub inventory_class: Option<i32>,
    #[serde(rename = "@inventory_default_item")]
    pub inventory_default_item: Option<i32>,
    #[serde(rename = "@electric_type")]
    pub electric_type: Option<i32>,
    #[serde(rename = "@electric_charge_capacity")]
    pub electric_charge_capacity: Option<i32>,
    #[serde(rename = "@electric_magnitude")]
    pub electric_magnitude: Option<f32>,
    #[serde(rename = "@composite_type")]
    pub composite_type: Option<i32>,
    #[serde(rename = "@camera_fov_min")]
    pub camera_fov_min: Option<f32>,
    #[serde(rename = "@camera_fov_max")]
    pub camera_fov_max: Option<f32>,
    #[serde(rename = "@monitor_border")]
    pub monitor_border: Option<f32>,
    #[serde(rename = "@monitor_inset")]
    pub monitor_inset: Option<f32>,
    #[serde(rename = "@weapon_type")]
    pub weapon_type: Option<i32>,
    #[serde(rename = "@weapon_class")]
    pub weapon_class: Option<i32>,
    #[serde(rename = "@weapon_belt_type")]
    pub weapon_belt_type: Option<i32>,
    #[serde(rename = "@weapon_ammo_capacity")]
    pub weapon_ammo_capacity: Option<i32>,
    #[serde(rename = "@weapon_ammo_feed")]
    pub weapon_ammo_feed: Option<bool>,
    #[serde(rename = "@weapon_barrel_length_voxels")]
    pub weapon_barrel_length_voxels: Option<i32>,
    #[serde(rename = "@rx_range")]
    pub rx_range: Option<f32>,
    #[serde(rename = "@rx_length")]
    pub rx_length: Option<f32>,
    #[serde(rename = "@rocket_type")]
    pub rocket_type: Option<i32>,
    #[serde(rename = "@radar_range")]
    pub radar_range: Option<f32>,
    #[serde(rename = "@radar_speed")]
    pub radar_speed: Option<f32>,
    #[serde(rename = "@engine_module_type")]
    pub engine_module_type: Option<i32>,
    #[serde(rename = "@steam_component_type")]
    pub steam_component_type: Option<i32>,
    #[serde(rename = "@steam_component_capacity")]
    pub steam_component_capacity: Option<f32>,
    #[serde(rename = "@nuclear_component_type")]
    pub nuclear_component_type: Option<i32>,
    #[serde(rename = "@radar_type")]
    pub radar_type: Option<i32>,
    #[serde(rename = "@piston_len")]
    pub piston_len: Option<f32>,
    #[serde(rename = "@piston_cam")]
    pub piston_cam: Option<f32>,
    #[serde(rename = "@data_logger_component_type")]
    pub data_logger_component_type: Option<i32>,
    #[serde(rename = "@metadata_component_type")]
    pub metadata_component_type: Option<i32>,
    #[serde(rename = "@oil_component_type")]
    pub oil_component_type: Option<i32>,
    #[serde(rename = "@tool_type")]
    pub tool_type: Option<i32>,

    pub sfx_datas: Vec<SfxDatas>,
    pub surfaces: Vec<Surfaces>,
    pub buoyancy_surfaces: Vec<BuoyancySurfaces>,
    pub logic_nodes: Vec<LogicNodes>,
    pub couplings: Vec<Couplings>,
    pub voxels: Vec<Voxels>,
    pub voxel_min: Vec<VoxelMin>,
    pub voxel_max: Vec<VoxelMax>,
    pub voxel_physics_min: Vec<VoxelPhysicsMin>,
    pub voxel_physics_max: Vec<VoxelPhysicsMax>,
    pub compartment_sample_pos: Vec<CompartmentSamplePos>,
    pub bb_physics_min: Vec<BbPhysicsMin>,
    pub bb_physics_max: Vec<BbPhysicsMax>,
    pub constraint_pos_parent: Vec<ConstraintPosParent>,
    pub constraint_pos_child: Vec<ConstraintPosChild>,
    pub voxel_location_child: Vec<VoxelLocationChild>,
    pub seat_offset: Vec<SeatOffset>,
    pub seat_front: Vec<SeatFront>,
    pub seat_up: Vec<SeatUp>,
    pub seat_camera: Vec<SeatCamera>,
    pub seat_render: Vec<SeatRender>,
    pub force_dir: Vec<ForceDir>,
    pub light_position: Vec<LightPosition>,
    pub light_color: Vec<LightColor>,
    pub light_forward: Vec<LightForward>,
    pub door_size: Vec<DoorSize>,
    pub door_normal: Vec<DoorNormal>,
    pub door_side: Vec<DoorSide>,
    pub door_up: Vec<DoorUp>,
    pub door_base_pos: Vec<DoorBasePos>,
    pub dynamic_body_position: Vec<DynamicBodyPosition>,
    pub dynamic_rotation_axes: Vec<DynamicRotationAxes>,
    pub dynamic_side_axis: Vec<DynamicSideAxis>,
    pub magnet_offset: Vec<MagnetOffset>,
    pub connector_axis: Vec<ConnectorAxis>,
    pub connector_up: Vec<ConnectorUp>,
    pub tooltip_properties: Vec<TooltipProperties>,
    pub jet_engine_connections_prev: Vec<JetEngineConnectionsPrev>,
    pub jet_engine_connections_next: Vec<JetEngineConnectionsNext>,
    pub particle_direction: Vec<ParticleDirection>,
    pub particle_offset: Vec<ParticleOffset>,
    pub particle_bounds: Vec<ParticleBounds>,
    pub reward_properties: Vec<RewardProperties>,
    pub seat_exit_position: Vec<SeatExitPosition>,
    pub weapon_breech_position: Vec<WeaponBreechPosition>,
    pub weapon_breech_normal: Vec<WeaponBreechNormal>,
    pub weapon_cart_position: Vec<WeaponCartPosition>,
    pub weapon_cart_velocity: Vec<WeaponCartVelocity>,
    pub rope_hook_offset: Vec<RopeHookOffset>,
}

impl Definition {
    pub fn rope_hook_offset_last(&self) -> Option<&RopeHookOffset> {
        self.rope_hook_offset.last()
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct SfxDatas {
    #[serde(default)]
    pub sfx_data: Vec<SfxData>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct SfxData {
    #[serde(rename = "@sfx_name")]
    pub sfx_name: Option<String>,
    #[serde(rename = "@sfx_range_inner")]
    pub sfx_range_inner: Option<f32>,
    #[serde(rename = "@sfx_range_outer")]
    pub sfx_range_outer: Option<f32>,
    #[serde(rename = "@sfx_priority")]
    pub sfx_priority: Option<f32>,
    #[serde(rename = "@sfx_is_underwater_affected")]
    pub sfx_is_underwater_affected: Option<bool>,

    pub sfx_layers: Vec<SfxLayers>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct SfxLayers {
    #[serde(default)]
    pub sfx_layer: Vec<SfxLayer>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct SfxLayer {
    #[serde(rename = "@sfx_filename_start")]
    pub sfx_filename_start: Option<String>,
    #[serde(rename = "@sfx_filename_loop")]
    pub sfx_filename_loop: Option<String>,
    #[serde(rename = "@sfx_filename_end")]
    pub sfx_filename_end: Option<String>,
    #[serde(rename = "@sfx_gain")]
    pub sfx_gain: Option<f32>,
    #[serde(rename = "@sfx_loop_start_time")]
    pub sfx_loop_start_time: Option<f32>,
    #[serde(rename = "@sfx_loop_blend_duration")]
    pub sfx_loop_blend_duration: Option<f32>,
    #[serde(rename = "@sfx_volume_fade_speed")]
    pub sfx_volume_fade_speed: Option<f32>,
    #[serde(rename = "@sfx_pitch_fade_speed")]
    pub sfx_pitch_fade_speed: Option<f32>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct Surfaces {
    #[serde(default)]
    pub surface: Vec<Surface>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct BuoyancySurfaces {
    #[serde(default)]
    pub surface: Vec<Surface>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct Surface {
    #[serde(rename = "@orientation")]
    pub orientation: Option<i32>,
    #[serde(rename = "@rotation")]
    pub rotation: Option<i32>,
    #[serde(rename = "@shape")]
    pub shape: Option<i32>,
    #[serde(rename = "@trans_type")]
    pub trans_type: Option<i32>,
    #[serde(rename = "@flags")]
    pub flags: Option<u64>,
    #[serde(rename = "@is_reverse_normals")]
    pub is_reverse_normals: Option<bool>,
    #[serde(rename = "@is_two_sided")]
    pub is_two_sided: Option<bool>,

    pub position: Vec<Position>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct LogicNodes {
    #[serde(default)]
    pub logic_node: Vec<LogicNode>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct LogicNode {
    #[serde(rename = "@orientation")]
    pub orientation: Option<i32>,
    #[serde(rename = "@label")]
    pub label: Option<String>,
    #[serde(rename = "@mode")]
    pub mode: Option<i32>,
    #[serde(rename = "@type")]
    pub node_type: Option<i32>,
    #[serde(rename = "@description")]
    pub description: Option<String>,
    #[serde(rename = "@flags")]
    pub flags: Option<u64>,

    pub position: Vec<Position>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct Couplings {
    #[serde(default)]
    pub coupling: Vec<Coupling>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct Coupling {
    #[serde(rename = "@orientation")]
    pub orientation: Option<i32>,
    #[serde(rename = "@alignment")]
    pub alignment: Option<i32>,
    #[serde(rename = "@coupling_type")]
    pub coupling_type: Option<String>,
    #[serde(rename = "@coupling_name")]
    pub coupling_name: Option<String>,
    #[serde(rename = "@coupling_gender")]
    pub coupling_gender: Option<i32>,
    #[serde(rename = "@alignment_required")]
    pub alignment_required: Option<bool>,
    #[serde(rename = "@allow_bipolar_alignment")]
    pub allow_bipolar_alignment: Option<bool>,

    pub position: Vec<Position>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct Voxels {
    #[serde(default)]
    pub voxel: Vec<Voxel>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct Voxel {
    #[serde(rename = "@flags")]
    pub flags: Option<i32>,
    #[serde(rename = "@physics_shape")]
    pub physics_shape: Option<i32>,
    #[serde(rename = "@buoy_pipes")]
    pub buoy_pipes: Option<i32>,

    pub position: Vec<Position>,
    pub physics_shape_rotation: Vec<PhysicsShapeRotation>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct PhysicsShapeRotation {
    #[serde(rename = "@00", default = "one")]
    pub r00: i32,
    #[serde(rename = "@01")]
    pub r01: i32,
    #[serde(rename = "@02")]
    pub r02: i32,
    #[serde(rename = "@10")]
    pub r10: i32,
    #[serde(rename = "@11", default = "one")]
    pub r11: i32,
    #[serde(rename = "@12")]
    pub r12: i32,
    #[serde(rename = "@20")]
    pub r20: i32,
    #[serde(rename = "@21")]
    pub r21: i32,
    #[serde(rename = "@22", default = "one")]
    pub r22: i32,
}

impl Default for PhysicsShapeRotation {
    fn default() -> Self {
        Self {
            r00: 1,
            r01: 0,
            r02: 0,
            r10: 0,
            r11: 1,
            r12: 0,
            r20: 0,
            r21: 0,
            r22: 1,
        }
    }
}

define_vec3!(Position, i32);
define_vec3!(Normal, i32);
define_vec3!(VoxelMin, i32);
define_vec3!(VoxelMax, i32);
define_vec3!(VoxelPhysicsMin, i32);
define_vec3!(VoxelPhysicsMax, i32);
define_vec3!(BbPhysicsMin, f32);
define_vec3!(BbPhysicsMax, f32);
define_vec3!(CompartmentSamplePos, i32);
define_vec3!(ConstraintPosParent, f32);
define_vec3!(ConstraintPosChild, f32);
define_vec3!(VoxelLocationChild, i32);
define_vec3!(SeatOffset, f32);
define_vec3!(SeatFront, i32);
define_vec3!(SeatUp, i32);
define_vec3!(SeatCamera, f32);
define_vec3!(SeatRender, f32);
define_vec3!(ForceDir, f32);
define_vec3!(LightPosition, i32);
define_vec3!(LightColor, f32);
define_vec3!(LightForward, f32);
define_vec3!(DoorSize, f32);
define_vec3!(DoorNormal, f32);
define_vec3!(DoorSide, f32);
define_vec3!(DoorUp, f32);
define_vec3!(DoorBasePos, f32);
define_vec3!(DynamicBodyPosition, i32);
define_vec3!(DynamicRotationAxes, f32);
define_vec3!(DynamicSideAxis, f32);
define_vec3!(MagnetOffset, f32);
define_vec3!(ConnectorAxis, i32);
define_vec3!(ConnectorUp, i32);
define_vec3!(ParticleDirection, i32);
define_vec3!(ParticleOffset, f32);
define_vec3!(ParticleBounds, f32);
define_vec3!(SeatExitPosition, i32);
define_vec3!(WeaponBreechPosition, f32);
define_vec3!(WeaponBreechNormal, f32);
define_vec3!(WeaponCartPosition, f32);
define_vec3!(WeaponCartVelocity, f32);
define_vec3!(RopeHookOffset, f32);

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct TooltipProperties {
    #[serde(rename = "@description")]
    pub description: Option<String>,
    #[serde(rename = "@short_description")]
    pub short_description: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct JetEngineConnectionsPrev {
    #[serde(default)]
    pub j: Vec<JetEngineConnection>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct JetEngineConnectionsNext {
    #[serde(default)]
    pub j: Vec<JetEngineConnection>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct JetEngineConnection {
    pub pos: Vec<Position>,
    pub normal: Vec<Normal>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct RewardProperties {
    #[serde(rename = "@tier")]
    pub tier: Option<i32>,
    #[serde(rename = "@number_rewarded")]
    pub number_rewarded: Option<i32>,
}

fn one() -> i32 {
    1
}
