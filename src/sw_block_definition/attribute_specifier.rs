use super::{AttributeType, AttributeValue, Definition};
use ambassador::{delegatable_trait, delegatable_trait_remote, Delegate};
use std::fmt::Display;

#[delegatable_trait_remote]
trait Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>;
}

#[delegatable_trait]
pub trait GetAttributeValueRoot: Clone + Copy + Display {
    fn get_value_root(&self, d: &Definition) -> Vec<AttributeValue>;
    fn get_type(&self) -> AttributeType;
}

pub trait GetAttributeValue<T>: GetAttributeValueRoot + Into<AttributeSpecifier> {
    fn get_value(&self, d: &T) -> Option<AttributeValue>;
}

#[derive(serde::Serialize, serde::Deserialize, Delegate, Clone, Copy)]
#[delegate(Display)]
#[delegate(GetAttributeValueRoot)]
pub enum AttributeSpecifier {
    Definition(crate::sw_block_definition::DefinitionAttribute),
    SfxData(crate::sw_block_definition::SfxDataAttribute),
    SfxLayer(crate::sw_block_definition::SfxLayerAttribute),
    Surface(crate::sw_block_definition::SurfaceAttribute),
    BuoyancySurface(crate::sw_block_definition::BuoyancySurfaceAttribute),
    LogicNode(crate::sw_block_definition::LogicNodeAttribute),
    Coupling(crate::sw_block_definition::CouplingAttribute),
    Voxel(crate::sw_block_definition::VoxelAttribute),
    VoxelMin(crate::sw_block_definition::VoxelMinAttribute),
    VoxelMax(crate::sw_block_definition::VoxelMaxAttribute),
    VoxelPhysicsMin(crate::sw_block_definition::VoxelPhysicsMinAttribute),
    VoxelPhysicsMax(crate::sw_block_definition::VoxelPhysicsMaxAttribute),
    BbPhysicsMin(crate::sw_block_definition::BbPhysicsMinAttribute),
    BbPhysicsMax(crate::sw_block_definition::BbPhysicsMaxAttribute),
    CompartmentSamplePos(crate::sw_block_definition::CompartmentSamplePosAttribute),
    ConstraintPosParent(crate::sw_block_definition::ConstraintPosParentAttribute),
    ConstraintPosChild(crate::sw_block_definition::ConstraintPosChildAttribute),
    VoxelLocationChild(crate::sw_block_definition::VoxelLocationChildAttribute),
    SeatOffset(crate::sw_block_definition::SeatOffsetAttribute),
    SeatFront(crate::sw_block_definition::SeatFrontAttribute),
    SeatUp(crate::sw_block_definition::SeatUpAttribute),
    SeatCamera(crate::sw_block_definition::SeatCameraAttribute),
    SeatRender(crate::sw_block_definition::SeatRenderAttribute),
    ForceDir(crate::sw_block_definition::ForceDirAttribute),
    LightPosition(crate::sw_block_definition::LightPositionAttribute),
    LightColor(crate::sw_block_definition::LightColorAttribute),
    LightForward(crate::sw_block_definition::LightForwardAttribute),
    DoorSize(crate::sw_block_definition::DoorSizeAttribute),
    DoorNormal(crate::sw_block_definition::DoorNormalAttribute),
    DoorSide(crate::sw_block_definition::DoorSideAttribute),
    DoorUp(crate::sw_block_definition::DoorUpAttribute),
    DoorBasePos(crate::sw_block_definition::DoorBasePosAttribute),
    DynamicBodyPosition(crate::sw_block_definition::DynamicBodyPositionAttribute),
    DynamicRotationAxes(crate::sw_block_definition::DynamicRotationAxesAttribute),
    DynamicSideAxis(crate::sw_block_definition::DynamicSideAxisAttribute),
    MagnetOffset(crate::sw_block_definition::MagnetOffsetAttribute),
    ConnectorAxis(crate::sw_block_definition::ConnectorAxisAttribute),
    ConnectorUp(crate::sw_block_definition::ConnectorUpAttribute),
    ParticleDirection(crate::sw_block_definition::ParticleDirectionAttribute),
    ParticleOffset(crate::sw_block_definition::ParticleOffsetAttribute),
    ParticleBounds(crate::sw_block_definition::ParticleBoundsAttribute),
    SeatExitPosition(crate::sw_block_definition::SeatExitPositionAttribute),
    WeaponBreechPosition(crate::sw_block_definition::WeaponBreechPositionAttribute),
    WeaponBreechNormal(crate::sw_block_definition::WeaponBreechNormalAttribute),
    WeaponCartPosition(crate::sw_block_definition::WeaponCartPositionAttribute),
    WeaponCartVelocity(crate::sw_block_definition::WeaponCartVelocityAttribute),
    RopeHookOffset(crate::sw_block_definition::RopeHookOffsetAttribute),
    JetEngineConnection(crate::sw_block_definition::JetEngineConnectionAttribute),
    TooltipProperties(crate::sw_block_definition::TooltipPropertiesAttribute),
    RewardProperties(crate::sw_block_definition::RewardPropertiesAttribute),
}
