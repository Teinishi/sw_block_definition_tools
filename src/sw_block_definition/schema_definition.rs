use super::{
    AttributeSpecifier, AttributeType, AttributeValue, BbPhysicsMax, BbPhysicsMin,
    BuoyancySurfaces, CompartmentSamplePos, ConnectorAxis, ConnectorUp, ConstraintPosChild,
    ConstraintPosParent, Couplings, DoorBasePos, DoorNormal, DoorSide, DoorSize, DoorUp,
    DynamicBodyPosition, DynamicRotationAxes, DynamicSideAxis, ForceDir, GetAttributeValue,
    GetAttributeValueRoot, JetEngineConnectionsNext, JetEngineConnectionsPrev, LightColor,
    LightForward, LightPosition, LogicNodes, MagnetOffset, Of32, ParticleBounds, ParticleDirection,
    ParticleOffset, RewardProperties, RopeHookOffset, SeatCamera, SeatExitPosition, SeatFront,
    SeatOffset, SeatRender, SeatUp, SfxDatas, Surfaces, TooltipProperties, VoxelLocationChild,
    VoxelMax, VoxelMin, VoxelPhysicsMax, VoxelPhysicsMin, Voxels, WeaponBreechNormal,
    WeaponBreechPosition, WeaponCartPosition, WeaponCartVelocity,
};
use serde::{Deserialize, Serialize};

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
    pub mass: Option<Of32>,
    #[serde(rename = "@value")]
    pub value: Option<Of32>,
    #[serde(rename = "@flags")]
    pub flags: Option<u64>,
    #[serde(rename = "@tags")]
    pub tags: Option<String>,
    #[serde(rename = "@phys_collision_dampen")]
    pub phys_collision_dampen: Option<i32>,
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
    pub audio_gain: Option<Of32>,
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
    pub constraint_range_of_motion: Option<Of32>,
    #[serde(rename = "@max_motor_force")]
    pub max_motor_force: Option<Of32>,
    #[serde(rename = "@max_motor_speed")]
    pub max_motor_speed: Option<Of32>,
    #[serde(rename = "@cable_radius")]
    pub cable_radius: Option<Of32>,
    #[serde(rename = "@cable_length")]
    pub cable_length: Option<Of32>,
    #[serde(rename = "@seat_type")]
    pub seat_type: Option<i32>,
    #[serde(rename = "@seat_pose")]
    pub seat_pose: Option<i32>,
    #[serde(rename = "@seat_health_per_sec")]
    pub seat_health_per_sec: Option<i32>,
    #[serde(rename = "@buoy_radius")]
    pub buoy_radius: Option<Of32>,
    #[serde(rename = "@buoy_factor")]
    pub buoy_factor: Option<Of32>,
    #[serde(rename = "@buoy_force")]
    pub buoy_force: Option<Of32>,
    #[serde(rename = "@force_emitter_max_force")]
    pub force_emitter_max_force: Option<Of32>,
    #[serde(rename = "@force_emitter_max_vector")]
    pub force_emitter_max_vector: Option<Of32>,
    #[serde(rename = "@force_emitter_default_pitch")]
    pub force_emitter_default_pitch: Option<Of32>,
    #[serde(rename = "@force_emitter_blade_height")]
    pub force_emitter_blade_height: Option<Of32>,
    #[serde(rename = "@force_emitter_rotation_speed")]
    pub force_emitter_rotation_speed: Option<Of32>,
    #[serde(rename = "@force_emitter_blade_physics_length")]
    pub force_emitter_blade_physics_length: Option<Of32>,
    #[serde(rename = "@force_emitter_blade_efficiency")]
    pub force_emitter_blade_efficiency: Option<Of32>,
    #[serde(rename = "@force_emitter_efficiency")]
    pub force_emitter_efficiency: Option<Of32>,
    #[serde(rename = "@engine_max_force")]
    pub engine_max_force: Option<Of32>,
    #[serde(rename = "@engine_frictionless_force")]
    pub engine_frictionless_force: Option<Of32>,
    #[serde(rename = "@trans_conn_type")]
    pub trans_conn_type: Option<i32>,
    #[serde(rename = "@trans_type")]
    pub trans_type: Option<i32>,
    #[serde(rename = "@wheel_radius")]
    pub wheel_radius: Option<Of32>,
    #[serde(rename = "@wheel_width")]
    pub wheel_width: Option<Of32>,
    #[serde(rename = "@wheel_wishbone_length")]
    pub wheel_wishbone_length: Option<Of32>,
    #[serde(rename = "@wheel_suspension_height")]
    pub wheel_suspension_height: Option<Of32>,
    #[serde(rename = "@wheel_wishbone_margin")]
    pub wheel_wishbone_margin: Option<Of32>,
    #[serde(rename = "@wheel_suspension_offset")]
    pub wheel_suspension_offset: Option<Of32>,
    #[serde(rename = "@wheel_wishbone_offset")]
    pub wheel_wishbone_offset: Option<Of32>,
    #[serde(rename = "@wheel_type")]
    pub wheel_type: Option<Of32>,
    #[serde(rename = "@button_type")]
    pub button_type: Option<i32>,
    #[serde(rename = "@light_intensity")]
    pub light_intensity: Option<Of32>,
    #[serde(rename = "@light_range")]
    pub light_range: Option<Of32>,
    #[serde(rename = "@light_ies_map")]
    pub light_ies_map: Option<String>,
    #[serde(rename = "@light_fov")]
    pub light_fov: Option<Of32>,
    #[serde(rename = "@light_type")]
    pub light_type: Option<i32>,
    #[serde(rename = "@door_lower_limit")]
    pub door_lower_limit: Option<Of32>,
    #[serde(rename = "@door_upper_limit")]
    pub door_upper_limit: Option<Of32>,
    #[serde(rename = "@door_flipped")]
    pub door_flipped: Option<bool>,
    #[serde(rename = "@custom_door_type")]
    pub custom_door_type: Option<i32>,
    #[serde(rename = "@door_side_dist")]
    pub door_side_dist: Option<i32>,
    #[serde(rename = "@door_up_dist")]
    pub door_up_dist: Option<i32>,
    #[serde(rename = "@dynamic_min_rotation")]
    pub dynamic_min_rotation: Option<Of32>,
    #[serde(rename = "@dynamic_max_rotation")]
    pub dynamic_max_rotation: Option<Of32>,
    #[serde(rename = "@logic_gate_type")]
    pub logic_gate_type: Option<i32>,
    #[serde(rename = "@logic_gate_subtype")]
    pub logic_gate_subtype: Option<i32>,
    #[serde(rename = "@indicator_type")]
    pub indicator_type: Option<i32>,
    #[serde(rename = "@connector_type")]
    pub connector_type: Option<i32>,
    #[serde(rename = "@magnet_force")]
    pub magnet_force: Option<Of32>,
    #[serde(rename = "@gyro_type")]
    pub gyro_type: Option<i32>,
    #[serde(rename = "@reward_tier")]
    pub reward_tier: Option<i32>,
    #[serde(rename = "@revision")]
    pub revision: Option<i32>,
    #[serde(rename = "@rudder_surface_area")]
    pub rudder_surface_area: Option<Of32>,
    #[serde(rename = "@pump_pressure")]
    pub pump_pressure: Option<Of32>,
    #[serde(rename = "@m_pump_pressure")]
    pub m_pump_pressure: Option<Of32>,
    #[serde(rename = "@water_component_type")]
    pub water_component_type: Option<i32>,
    #[serde(rename = "@torque_component_type")]
    pub torque_component_type: Option<i32>,
    #[serde(rename = "@jet_engine_component_type")]
    pub jet_engine_component_type: Option<i32>,
    #[serde(rename = "@particle_speed")]
    pub particle_speed: Option<Of32>,
    #[serde(rename = "@inventory_type")]
    pub inventory_type: Option<i32>,
    #[serde(rename = "@inventory_default_outfit")]
    pub inventory_default_outfit: Option<i32>,
    #[serde(rename = "@inventory_class")]
    pub inventory_class: Option<i32>,
    #[serde(rename = "@inventory_default_item")]
    pub inventory_default_item: Option<i32>,
    #[serde(rename = "@electric_type")]
    pub electric_type: Option<i32>,
    #[serde(rename = "@electric_charge_capacity")]
    pub electric_charge_capacity: Option<i32>,
    #[serde(rename = "@electric_magnitude")]
    pub electric_magnitude: Option<Of32>,
    #[serde(rename = "@composite_type")]
    pub composite_type: Option<i32>,
    #[serde(rename = "@camera_fov_min")]
    pub camera_fov_min: Option<Of32>,
    #[serde(rename = "@camera_fov_max")]
    pub camera_fov_max: Option<Of32>,
    #[serde(rename = "@monitor_border")]
    pub monitor_border: Option<Of32>,
    #[serde(rename = "@monitor_inset")]
    pub monitor_inset: Option<Of32>,
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
    pub rx_range: Option<Of32>,
    #[serde(rename = "@rx_length")]
    pub rx_length: Option<Of32>,
    #[serde(rename = "@rocket_type")]
    pub rocket_type: Option<i32>,
    #[serde(rename = "@radar_range")]
    pub radar_range: Option<Of32>,
    #[serde(rename = "@radar_speed")]
    pub radar_speed: Option<Of32>,
    #[serde(rename = "@engine_module_type")]
    pub engine_module_type: Option<i32>,
    #[serde(rename = "@steam_component_type")]
    pub steam_component_type: Option<i32>,
    #[serde(rename = "@steam_component_capacity")]
    pub steam_component_capacity: Option<Of32>,
    #[serde(rename = "@nuclear_component_type")]
    pub nuclear_component_type: Option<i32>,
    #[serde(rename = "@radar_type")]
    pub radar_type: Option<i32>,
    #[serde(rename = "@piston_len")]
    pub piston_len: Option<Of32>,
    #[serde(rename = "@piston_cam")]
    pub piston_cam: Option<Of32>,
    #[serde(rename = "@data_logger_component_type")]
    pub data_logger_component_type: Option<i32>,
    #[serde(rename = "@metadata_component_type")]
    pub metadata_component_type: Option<i32>,
    #[serde(rename = "@oil_component_type")]
    pub oil_component_type: Option<i32>,
    #[serde(rename = "@tool_type")]
    pub tool_type: Option<i32>,
    #[serde(rename = "@rudder_type")]
    pub rudder_type: Option<i32>,

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
    pub bb_physics_min: Vec<BbPhysicsMin>,
    pub bb_physics_max: Vec<BbPhysicsMax>,
    pub compartment_sample_pos: Vec<CompartmentSamplePos>,
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

#[derive(
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    strum::Display,
    strum::VariantArray,
    Clone,
    Copy,
)]
#[strum(serialize_all = "snake_case")]
pub enum DefinitionAttribute {
    Name,
    Category,
    Type,
    Mass,
    Value,
    Flags,
    Tags,
    PhysCollisionDampen,
    AudioFilenameStart,
    AudioFilenameLoop,
    AudioFilenameEnd,
    AudioFilenameStartB,
    AudioFilenameLoopB,
    AudioFilenameEndB,
    AudioGain,
    MeshDataName,
    Mesh0Name,
    Mesh1Name,
    Mesh2Name,
    MeshEditorOnlyName,
    BlockType,
    ChildName,
    ExtenderName,
    ConstraintType,
    ConstraintAxis,
    ConstraintRangeOfMotion,
    MaxMotorForce,
    MaxMotorSpeed,
    CableRadius,
    CableLength,
    SeatType,
    SeatPose,
    SeatHealthPerSec,
    BuoyRadius,
    BuoyFactor,
    BuoyForce,
    ForceEmitterMaxForce,
    ForceEmitterMaxVector,
    ForceEmitterDefaultPitch,
    ForceEmitterBladeHeight,
    ForceEmitterRotationSpeed,
    ForceEmitterBladePhysicsLength,
    ForceEmitterBladeEfficiency,
    ForceEmitterEfficiency,
    EngineMaxForce,
    EngineFrictionlessForce,
    TransConnType,
    TransType,
    WheelRadius,
    WheelWidth,
    WheelWishboneLength,
    WheelSuspensionHeight,
    WheelWishboneMargin,
    WheelSuspensionOffset,
    WheelWishboneOffset,
    WheelType,
    ButtonType,
    LightIntensity,
    LightRange,
    LightIesMap,
    LightFov,
    LightType,
    DoorLowerLimit,
    DoorUpperLimit,
    DoorFlipped,
    CustomDoorType,
    DoorSideDist,
    DoorUpDist,
    DynamicMinRotation,
    DynamicMaxRotation,
    LogicGateType,
    LogicGateSubtype,
    IndicatorType,
    ConnectorType,
    MagnetForce,
    GyroType,
    RewardTier,
    Revision,
    RudderSurfaceArea,
    PumpPressure,
    MPumpPressure,
    WaterComponentType,
    TorqueComponentType,
    JetEngineComponentType,
    ParticleSpeed,
    InventoryType,
    InventoryDefaultOutfit,
    InventoryClass,
    InventoryDefaultItem,
    ElectricType,
    ElectricChargeCapacity,
    ElectricMagnitude,
    CompositeType,
    CameraFovMin,
    CameraFovMax,
    MonitorBorder,
    MonitorInset,
    WeaponType,
    WeaponClass,
    WeaponBeltType,
    WeaponAmmoCapacity,
    WeaponAmmoFeed,
    WeaponBarrelLengthVoxels,
    RxRange,
    RxLength,
    RocketType,
    RadarRange,
    RadarSpeed,
    EngineModuleType,
    SteamComponentType,
    SteamComponentCapacity,
    NuclearComponentType,
    RadarType,
    PistonLen,
    PistonCam,
    DataLoggerComponentType,
    MetadataComponentType,
    OilComponentType,
    ToolType,
    RudderType,
    VoxelMin,
    VoxelMax,
    VoxelPhysicsMin,
    VoxelPhysicsMax,
    BbPhysicsMin,
    BbPhysicsMax,
    CompartmentSamplePos,
    ConstraintPosParent,
    ConstraintPosChild,
    VoxelLocationChild,
    SeatOffset,
    SeatFront,
    SeatUp,
    SeatCamera,
    SeatRender,
    ForceDir,
    LightPosition,
    LightColor,
    LightForward,
    DoorSize,
    DoorNormal,
    DoorSide,
    DoorUp,
    DoorBasePos,
    DynamicBodyPosition,
    DynamicRotationAxes,
    DynamicSideAxis,
    MagnetOffset,
    ConnectorAxis,
    ConnectorUp,
    //TooltipProperties,
    //JetEngineConnectionsPrev,
    //JetEngineConnectionsNext,
    ParticleDirection,
    ParticleOffset,
    ParticleBounds,
    //RewardProperties,
    SeatExitPosition,
    WeaponBreechPosition,
    WeaponBreechNormal,
    WeaponCartPosition,
    WeaponCartVelocity,
    RopeHookOffset,
}

impl GetAttributeValueRoot for DefinitionAttribute {
    fn get_value_root(&self, d: &Definition) -> Vec<AttributeValue> {
        self.get_value(d).into_iter().collect()
    }

    fn get_type(&self) -> AttributeType {
        match self {
            Self::Name | Self::Tags | Self::ChildName | Self::ExtenderName | Self::LightIesMap => {
                AttributeType::String
            }
            Self::Category
            | Self::Type
            | Self::PhysCollisionDampen
            | Self::BlockType
            | Self::ConstraintType
            | Self::ConstraintAxis
            | Self::SeatType
            | Self::SeatPose
            | Self::SeatHealthPerSec
            | Self::TransConnType
            | Self::TransType
            | Self::ButtonType
            | Self::LightType
            | Self::CustomDoorType
            | Self::DoorSideDist
            | Self::DoorUpDist
            | Self::LogicGateType
            | Self::LogicGateSubtype
            | Self::IndicatorType
            | Self::ConnectorType
            | Self::GyroType
            | Self::RewardTier
            | Self::Revision
            | Self::WaterComponentType
            | Self::TorqueComponentType
            | Self::JetEngineComponentType
            | Self::InventoryType
            | Self::InventoryDefaultOutfit
            | Self::InventoryClass
            | Self::InventoryDefaultItem
            | Self::ElectricType
            | Self::ElectricChargeCapacity
            | Self::CompositeType
            | Self::WeaponType
            | Self::WeaponClass
            | Self::WeaponBeltType
            | Self::WeaponAmmoCapacity
            | Self::WeaponBarrelLengthVoxels
            | Self::RocketType
            | Self::EngineModuleType
            | Self::SteamComponentType
            | Self::NuclearComponentType
            | Self::RadarType
            | Self::DataLoggerComponentType
            | Self::MetadataComponentType
            | Self::OilComponentType
            | Self::ToolType
            | Self::RudderType => AttributeType::Int,
            Self::Mass
            | Self::Value
            | Self::AudioGain
            | Self::ConstraintRangeOfMotion
            | Self::MaxMotorForce
            | Self::MaxMotorSpeed
            | Self::CableRadius
            | Self::CableLength
            | Self::BuoyRadius
            | Self::BuoyFactor
            | Self::BuoyForce
            | Self::ForceEmitterMaxForce
            | Self::ForceEmitterMaxVector
            | Self::ForceEmitterDefaultPitch
            | Self::ForceEmitterBladeHeight
            | Self::ForceEmitterRotationSpeed
            | Self::ForceEmitterBladePhysicsLength
            | Self::ForceEmitterBladeEfficiency
            | Self::ForceEmitterEfficiency
            | Self::EngineMaxForce
            | Self::EngineFrictionlessForce
            | Self::WheelRadius
            | Self::WheelWidth
            | Self::WheelWishboneLength
            | Self::WheelSuspensionHeight
            | Self::WheelWishboneMargin
            | Self::WheelSuspensionOffset
            | Self::WheelWishboneOffset
            | Self::WheelType
            | Self::LightIntensity
            | Self::LightRange
            | Self::LightFov
            | Self::DoorLowerLimit
            | Self::DoorUpperLimit
            | Self::DynamicMinRotation
            | Self::DynamicMaxRotation
            | Self::MagnetForce
            | Self::RudderSurfaceArea
            | Self::PumpPressure
            | Self::MPumpPressure
            | Self::ParticleSpeed
            | Self::ElectricMagnitude
            | Self::CameraFovMin
            | Self::CameraFovMax
            | Self::MonitorBorder
            | Self::MonitorInset
            | Self::RxRange
            | Self::RxLength
            | Self::RadarRange
            | Self::RadarSpeed
            | Self::SteamComponentCapacity
            | Self::PistonLen
            | Self::PistonCam => AttributeType::Float,
            Self::Flags => AttributeType::Flags,
            Self::AudioFilenameStart
            | Self::AudioFilenameLoop
            | Self::AudioFilenameEnd
            | Self::AudioFilenameStartB
            | Self::AudioFilenameLoopB
            | Self::AudioFilenameEndB => AttributeType::AudioFile,
            Self::MeshDataName
            | Self::Mesh0Name
            | Self::Mesh1Name
            | Self::Mesh2Name
            | Self::MeshEditorOnlyName => AttributeType::MeshFile,
            Self::DoorFlipped | Self::WeaponAmmoFeed => AttributeType::Bool,
            Self::VoxelMin
            | Self::VoxelMax
            | Self::VoxelPhysicsMin
            | Self::VoxelPhysicsMax
            | Self::CompartmentSamplePos
            | Self::VoxelLocationChild
            | Self::SeatFront
            | Self::SeatUp
            | Self::SeatExitPosition
            | Self::ForceDir
            | Self::LightPosition
            | Self::LightForward
            | Self::DoorNormal
            | Self::DoorSide
            | Self::DoorUp
            | Self::DoorBasePos
            | Self::DynamicBodyPosition
            | Self::DynamicRotationAxes
            | Self::DynamicSideAxis
            | Self::ConnectorAxis
            | Self::ConnectorUp
            | Self::ParticleDirection
            | Self::WeaponBreechPosition
            | Self::WeaponBreechNormal => AttributeType::VecInt,
            Self::BbPhysicsMin
            | Self::BbPhysicsMax
            | Self::ConstraintPosParent
            | Self::ConstraintPosChild
            | Self::SeatOffset
            | Self::SeatCamera
            | Self::SeatRender
            | Self::LightColor
            | Self::DoorSize
            | Self::MagnetOffset
            | Self::RopeHookOffset
            | Self::ParticleOffset
            | Self::ParticleBounds
            | Self::WeaponCartPosition
            | Self::WeaponCartVelocity => AttributeType::VecFloat,
        }
    }

    /*fn property(&self) -> AttributeProperty {
        let is_audio_file = matches!(
            self,
            Self::AudioFilenameStart
                | Self::AudioFilenameLoop
                | Self::AudioFilenameEnd
                | Self::AudioFilenameStartB
                | Self::AudioFilenameLoopB
                | Self::AudioFilenameEndB
        );
        let is_not_number = is_audio_file
            || matches!(
                self,
                Self::Name
                    | Self::Tags
                    | Self::MeshDataName
                    | Self::Mesh0Name
                    | Self::Mesh1Name
                    | Self::Mesh2Name
                    | Self::MeshEditorOnlyName
                    | Self::ChildName
                    | Self::ExtenderName
                    | Self::LightIesMap
                    | Self::DoorFlipped
                    | Self::WeaponAmmoFeed
                    | Self::VoxelMin
                    | Self::VoxelMax
                    | Self::VoxelPhysicsMin
                    | Self::VoxelPhysicsMax
                    | Self::BbPhysicsMin
                    | Self::BbPhysicsMax
                    | Self::CompartmentSamplePos
                    | Self::ConstraintPosParent
                    | Self::ConstraintPosChild
                    | Self::VoxelLocationChild
                    | Self::SeatOffset
                    | Self::SeatFront
                    | Self::SeatUp
                    | Self::SeatCamera
                    | Self::SeatRender
                    | Self::ForceDir
                    | Self::LightPosition
                    | Self::LightColor
                    | Self::LightForward
                    | Self::DoorSize
                    | Self::DoorNormal
                    | Self::DoorSide
                    | Self::DoorUp
                    | Self::DoorBasePos
                    | Self::DynamicBodyPosition
                    | Self::DynamicRotationAxes
                    | Self::DynamicSideAxis
                    | Self::MagnetOffset
                    | Self::ConnectorAxis
                    | Self::ConnectorUp
                    //| Self::TooltipProperties
                    //| Self::JetEngineConnectionsPrev
                    //| Self::JetEngineConnectionsNext
                    | Self::ParticleDirection
                    | Self::ParticleOffset
                    | Self::ParticleBounds
                    //| Self::RewardProperties
                    | Self::SeatExitPosition
                    | Self::WeaponBreechPosition
                    | Self::WeaponBreechNormal
                    | Self::WeaponCartPosition
                    | Self::WeaponCartVelocity
                    | Self::RopeHookOffset
            );
        AttributeProperty {
            is_audio_file,
            is_number: !is_not_number,
        }
    }*/
}

impl GetAttributeValue<Definition> for DefinitionAttribute {
    fn get_value(&self, d: &Definition) -> Option<AttributeValue> {
        match self {
            Self::Name => Some(d.name.clone()?.into()),
            Self::Category => Some(d.category?.into()),
            Self::Type => Some(d.definition_type?.into()),
            Self::Mass => Some(d.mass?.into()),
            Self::Value => Some(d.value?.into()),
            Self::Flags => Some(d.flags?.into()),
            Self::Tags => Some(d.tags.clone()?.into()),
            Self::PhysCollisionDampen => Some(d.phys_collision_dampen?.into()),
            Self::AudioFilenameStart => Some(d.audio_filename_start.clone()?.into()),
            Self::AudioFilenameLoop => Some(d.audio_filename_loop.clone()?.into()),
            Self::AudioFilenameEnd => Some(d.audio_filename_end.clone()?.into()),
            Self::AudioFilenameStartB => Some(d.audio_filename_start_b.clone()?.into()),
            Self::AudioFilenameLoopB => Some(d.audio_filename_loop_b.clone()?.into()),
            Self::AudioFilenameEndB => Some(d.audio_filename_end_b.clone()?.into()),
            Self::AudioGain => Some(d.audio_gain?.into()),
            Self::MeshDataName => Some(d.mesh_data_name.clone()?.into()),
            Self::Mesh0Name => Some(d.mesh_0_name.clone()?.into()),
            Self::Mesh1Name => Some(d.mesh_1_name.clone()?.into()),
            Self::Mesh2Name => Some(d.mesh_2_name.clone()?.into()),
            Self::MeshEditorOnlyName => Some(d.mesh_editor_only_name.clone()?.into()),
            Self::BlockType => Some(d.block_type?.into()),
            Self::ChildName => Some(d.child_name.clone()?.into()),
            Self::ExtenderName => Some(d.extender_name.clone()?.into()),
            Self::ConstraintType => Some(d.constraint_type?.into()),
            Self::ConstraintAxis => Some(d.constraint_axis?.into()),
            Self::ConstraintRangeOfMotion => Some(d.constraint_range_of_motion?.into()),
            Self::MaxMotorForce => Some(d.max_motor_force?.into()),
            Self::MaxMotorSpeed => Some(d.max_motor_speed?.into()),
            Self::CableRadius => Some(d.cable_radius?.into()),
            Self::CableLength => Some(d.cable_length?.into()),
            Self::SeatType => Some(d.seat_type?.into()),
            Self::SeatPose => Some(d.seat_pose?.into()),
            Self::SeatHealthPerSec => Some(d.seat_health_per_sec?.into()),
            Self::BuoyRadius => Some(d.buoy_radius?.into()),
            Self::BuoyFactor => Some(d.buoy_factor?.into()),
            Self::BuoyForce => Some(d.buoy_force?.into()),
            Self::ForceEmitterMaxForce => Some(d.force_emitter_max_force?.into()),
            Self::ForceEmitterMaxVector => Some(d.force_emitter_max_vector?.into()),
            Self::ForceEmitterDefaultPitch => Some(d.force_emitter_default_pitch?.into()),
            Self::ForceEmitterBladeHeight => Some(d.force_emitter_blade_height?.into()),
            Self::ForceEmitterRotationSpeed => Some(d.force_emitter_rotation_speed?.into()),
            Self::ForceEmitterBladePhysicsLength => {
                Some(d.force_emitter_blade_physics_length?.into())
            }
            Self::ForceEmitterBladeEfficiency => Some(d.force_emitter_blade_efficiency?.into()),
            Self::ForceEmitterEfficiency => Some(d.force_emitter_efficiency?.into()),
            Self::EngineMaxForce => Some(d.engine_max_force?.into()),
            Self::EngineFrictionlessForce => Some(d.engine_frictionless_force?.into()),
            Self::TransConnType => Some(d.trans_conn_type?.into()),
            Self::TransType => Some(d.trans_type?.into()),
            Self::WheelRadius => Some(d.wheel_radius?.into()),
            Self::WheelWidth => Some(d.wheel_width?.into()),
            Self::WheelWishboneLength => Some(d.wheel_wishbone_length?.into()),
            Self::WheelSuspensionHeight => Some(d.wheel_suspension_height?.into()),
            Self::WheelWishboneMargin => Some(d.wheel_wishbone_margin?.into()),
            Self::WheelSuspensionOffset => Some(d.wheel_suspension_offset?.into()),
            Self::WheelWishboneOffset => Some(d.wheel_wishbone_offset?.into()),
            Self::WheelType => Some(d.wheel_type?.into()),
            Self::ButtonType => Some(d.button_type?.into()),
            Self::LightIntensity => Some(d.light_intensity?.into()),
            Self::LightRange => Some(d.light_range?.into()),
            Self::LightIesMap => Some(d.light_ies_map.clone()?.into()),
            Self::LightFov => Some(d.light_fov?.into()),
            Self::LightType => Some(d.light_type?.into()),
            Self::DoorLowerLimit => Some(d.door_lower_limit?.into()),
            Self::DoorUpperLimit => Some(d.door_upper_limit?.into()),
            Self::DoorFlipped => Some(d.door_flipped?.into()),
            Self::CustomDoorType => Some(d.custom_door_type?.into()),
            Self::DoorSideDist => Some(d.door_side_dist?.into()),
            Self::DoorUpDist => Some(d.door_up_dist?.into()),
            Self::DynamicMinRotation => Some(d.dynamic_min_rotation?.into()),
            Self::DynamicMaxRotation => Some(d.dynamic_max_rotation?.into()),
            Self::LogicGateType => Some(d.logic_gate_type?.into()),
            Self::LogicGateSubtype => Some(d.logic_gate_subtype?.into()),
            Self::IndicatorType => Some(d.indicator_type?.into()),
            Self::ConnectorType => Some(d.connector_type?.into()),
            Self::MagnetForce => Some(d.magnet_force?.into()),
            Self::GyroType => Some(d.gyro_type?.into()),
            Self::RewardTier => Some(d.reward_tier?.into()),
            Self::Revision => Some(d.revision?.into()),
            Self::RudderSurfaceArea => Some(d.rudder_surface_area?.into()),
            Self::PumpPressure => Some(d.pump_pressure?.into()),
            Self::MPumpPressure => Some(d.m_pump_pressure?.into()),
            Self::WaterComponentType => Some(d.water_component_type?.into()),
            Self::TorqueComponentType => Some(d.torque_component_type?.into()),
            Self::JetEngineComponentType => Some(d.jet_engine_component_type?.into()),
            Self::ParticleSpeed => Some(d.particle_speed?.into()),
            Self::InventoryType => Some(d.inventory_type?.into()),
            Self::InventoryDefaultOutfit => Some(d.inventory_default_outfit?.into()),
            Self::InventoryClass => Some(d.inventory_class?.into()),
            Self::InventoryDefaultItem => Some(d.inventory_default_item?.into()),
            Self::ElectricType => Some(d.electric_type?.into()),
            Self::ElectricChargeCapacity => Some(d.electric_charge_capacity?.into()),
            Self::ElectricMagnitude => Some(d.electric_magnitude?.into()),
            Self::CompositeType => Some(d.composite_type?.into()),
            Self::CameraFovMin => Some(d.camera_fov_min?.into()),
            Self::CameraFovMax => Some(d.camera_fov_max?.into()),
            Self::MonitorBorder => Some(d.monitor_border?.into()),
            Self::MonitorInset => Some(d.monitor_inset?.into()),
            Self::WeaponType => Some(d.weapon_type?.into()),
            Self::WeaponClass => Some(d.weapon_class?.into()),
            Self::WeaponBeltType => Some(d.weapon_belt_type?.into()),
            Self::WeaponAmmoCapacity => Some(d.weapon_ammo_capacity?.into()),
            Self::WeaponAmmoFeed => Some(d.weapon_ammo_feed?.into()),
            Self::WeaponBarrelLengthVoxels => Some(d.weapon_barrel_length_voxels?.into()),
            Self::RxRange => Some(d.rx_range?.into()),
            Self::RxLength => Some(d.rx_length?.into()),
            Self::RocketType => Some(d.rocket_type?.into()),
            Self::RadarRange => Some(d.radar_range?.into()),
            Self::RadarSpeed => Some(d.radar_speed?.into()),
            Self::EngineModuleType => Some(d.engine_module_type?.into()),
            Self::SteamComponentType => Some(d.steam_component_type?.into()),
            Self::SteamComponentCapacity => Some(d.steam_component_capacity?.into()),
            Self::NuclearComponentType => Some(d.nuclear_component_type?.into()),
            Self::RadarType => Some(d.radar_type?.into()),
            Self::PistonLen => Some(d.piston_len?.into()),
            Self::PistonCam => Some(d.piston_cam?.into()),
            Self::DataLoggerComponentType => Some(d.data_logger_component_type?.into()),
            Self::MetadataComponentType => Some(d.metadata_component_type?.into()),
            Self::OilComponentType => Some(d.oil_component_type?.into()),
            Self::ToolType => Some(d.tool_type?.into()),
            Self::RudderType => Some(d.tool_type?.into()),
            Self::VoxelMin => Some((*d.voxel_min.last()?).into()),
            Self::VoxelMax => Some((*d.voxel_max.last()?).into()),
            Self::VoxelPhysicsMin => Some((*d.voxel_physics_min.last()?).into()),
            Self::VoxelPhysicsMax => Some((*d.voxel_physics_max.last()?).into()),
            Self::BbPhysicsMin => Some((*d.bb_physics_min.last()?).into()),
            Self::BbPhysicsMax => Some((*d.bb_physics_max.last()?).into()),
            Self::CompartmentSamplePos => Some((*d.compartment_sample_pos.last()?).into()),
            Self::ConstraintPosParent => Some((*d.constraint_pos_parent.last()?).into()),
            Self::ConstraintPosChild => Some((*d.constraint_pos_child.last()?).into()),
            Self::VoxelLocationChild => Some((*d.voxel_location_child.last()?).into()),
            Self::SeatOffset => Some((*d.seat_offset.last()?).into()),
            Self::SeatFront => Some((*d.seat_front.last()?).into()),
            Self::SeatUp => Some((*d.seat_up.last()?).into()),
            Self::SeatCamera => Some((*d.seat_camera.last()?).into()),
            Self::SeatRender => Some((*d.seat_render.last()?).into()),
            Self::ForceDir => Some((*d.force_dir.last()?).into()),
            Self::LightPosition => Some((*d.light_position.last()?).into()),
            Self::LightColor => Some((*d.light_color.last()?).into()),
            Self::LightForward => Some((*d.light_forward.last()?).into()),
            Self::DoorSize => Some((*d.door_size.last()?).into()),
            Self::DoorNormal => Some((*d.door_normal.last()?).into()),
            Self::DoorSide => Some((*d.door_side.last()?).into()),
            Self::DoorUp => Some((*d.door_up.last()?).into()),
            Self::DoorBasePos => Some((*d.door_base_pos.last()?).into()),
            Self::DynamicBodyPosition => Some((*d.dynamic_body_position.last()?).into()),
            Self::DynamicRotationAxes => Some((*d.dynamic_rotation_axes.last()?).into()),
            Self::DynamicSideAxis => Some((*d.dynamic_side_axis.last()?).into()),
            Self::MagnetOffset => Some((*d.magnet_offset.last()?).into()),
            Self::ConnectorAxis => Some((*d.connector_axis.last()?).into()),
            Self::ConnectorUp => Some((*d.connector_up.last()?).into()),
            //Self::TooltipProperties => Some((*d.tooltip_properties.last()?).into()),
            //Self::JetEngineConnectionsPrev => Some((*d.jet_engine_connections_prev.last()?).into()),
            //Self::JetEngineConnectionsNext => Some((*d.jet_engine_connections_next.last()?).into()),
            Self::ParticleDirection => Some((*d.particle_direction.last()?).into()),
            Self::ParticleOffset => Some((*d.particle_offset.last()?).into()),
            Self::ParticleBounds => Some((*d.particle_bounds.last()?).into()),
            //Self::RewardProperties => Some((*d.reward_properties.last()?).into()),
            Self::SeatExitPosition => Some((*d.seat_exit_position.last()?).into()),
            Self::WeaponBreechPosition => Some((*d.weapon_breech_position.last()?).into()),
            Self::WeaponBreechNormal => Some((*d.weapon_breech_normal.last()?).into()),
            Self::WeaponCartPosition => Some((*d.weapon_cart_position.last()?).into()),
            Self::WeaponCartVelocity => Some((*d.weapon_cart_velocity.last()?).into()),
            Self::RopeHookOffset => Some((*d.rope_hook_offset.last()?).into()),
        }
    }
}

impl From<DefinitionAttribute> for AttributeSpecifier {
    fn from(value: DefinitionAttribute) -> Self {
        Self::Definition(value)
    }
}

#[derive(
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    strum::Display,
    strum::VariantArray,
    Clone,
    Copy,
)]
#[strum(serialize_all = "snake_case")]
pub enum JetEngineConnectionAttribute {
    PrevPos,
    PrevNormal,
    NextPos,
    NextNormal,
}

impl GetAttributeValueRoot for JetEngineConnectionAttribute {
    fn get_value_root(&self, d: &Definition) -> Vec<AttributeValue> {
        self.get_value(d).into_iter().collect()
    }

    fn get_type(&self) -> AttributeType {
        AttributeType::VecInt
    }
}

impl GetAttributeValue<Definition> for JetEngineConnectionAttribute {
    fn get_value(&self, d: &Definition) -> Option<AttributeValue> {
        match self {
            Self::PrevPos => {
                Some((*d.jet_engine_connections_prev.last()?.j.last()?.pos.last()?).into())
            }
            Self::PrevNormal => Some(
                (*d.jet_engine_connections_prev
                    .last()?
                    .j
                    .last()?
                    .normal
                    .last()?)
                .into(),
            ),
            Self::NextPos => {
                Some((*d.jet_engine_connections_next.last()?.j.last()?.pos.last()?).into())
            }
            Self::NextNormal => Some(
                (*d.jet_engine_connections_next
                    .last()?
                    .j
                    .last()?
                    .normal
                    .last()?)
                .into(),
            ),
        }
    }
}

impl From<JetEngineConnectionAttribute> for AttributeSpecifier {
    fn from(value: JetEngineConnectionAttribute) -> Self {
        Self::JetEngineConnection(value)
    }
}
