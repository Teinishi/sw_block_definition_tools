use super::definition_schema::Definition;

type Of32 = ordered_float::NotNan<f32>;

#[derive(
    serde::Serialize, serde::Deserialize, PartialEq, strum::Display, strum::VariantArray, Clone,
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
}

impl DefinitionAttribute {
    pub fn get_value(&self, d: &Definition) -> Option<DefinitionAttributeValue> {
        match self {
            Self::Name => Some(d.name.clone()?.into()),
            Self::Category => Some(d.category?.into()),
            Self::Type => Some(d.definition_type?.into()),
            Self::Mass => Some(d.mass?.into()),
            Self::Value => Some(d.value?.into()),
            Self::Flags => Some(d.flags?.into()),
            Self::Tags => Some(d.tags.clone()?.into()),
            Self::PhysCollisionDampen => Some(d.phys_collision_dampen.clone()?.into()),
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
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum DefinitionAttributeValue {
    Bool(bool),
    I32(i32),
    U64(u64),
    Of32(Of32),
    String(String),
}

impl DefinitionAttributeValue {
    pub fn debug_str(&self) -> String {
        match self {
            Self::Bool(value) => format!("{:?}", value),
            Self::I32(value) => format!("{:?}", value),
            Self::U64(value) => format!("{:?}", value),
            Self::Of32(value) => format!("{:?}", value),
            Self::String(value) => format!("{:?}", value),
        }
    }

    pub fn is_default(&self) -> bool {
        match self {
            Self::Bool(value) => !value,
            Self::I32(value) => *value == 0,
            Self::U64(value) => *value == 0,
            Self::Of32(value) => *value == 0.0,
            Self::String(value) => value.is_empty(),
        }
    }
}

impl From<bool> for DefinitionAttributeValue {
    fn from(value: bool) -> Self {
        DefinitionAttributeValue::Bool(value)
    }
}

impl From<i32> for DefinitionAttributeValue {
    fn from(value: i32) -> Self {
        DefinitionAttributeValue::I32(value)
    }
}

impl From<u64> for DefinitionAttributeValue {
    fn from(value: u64) -> Self {
        DefinitionAttributeValue::U64(value)
    }
}

impl From<Of32> for DefinitionAttributeValue {
    fn from(value: Of32) -> Self {
        DefinitionAttributeValue::Of32(value)
    }
}

impl From<String> for DefinitionAttributeValue {
    fn from(value: String) -> Self {
        DefinitionAttributeValue::String(value)
    }
}
