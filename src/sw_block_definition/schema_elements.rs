use super::{Of32, Position};
use serde::{Deserialize, Serialize};

macro_rules! define_vec3 {
    ($name:ident, $type:ty) => {
        #[derive(Serialize, Deserialize, Default, Debug)]
        #[serde(default)]
        pub struct $name {
            #[serde(rename = "@x")]
            pub x: Option<$type>,
            #[serde(rename = "@y")]
            pub y: Option<$type>,
            #[serde(rename = "@z")]
            pub z: Option<$type>,
        }
    };
}

define_vec3!(Normal, i32);
define_vec3!(VoxelMin, i32);
define_vec3!(VoxelMax, i32);
define_vec3!(VoxelPhysicsMin, i32);
define_vec3!(VoxelPhysicsMax, i32);
define_vec3!(BbPhysicsMin, Of32);
define_vec3!(BbPhysicsMax, Of32);
define_vec3!(CompartmentSamplePos, i32);
define_vec3!(ConstraintPosParent, Of32);
define_vec3!(ConstraintPosChild, Of32);
define_vec3!(VoxelLocationChild, i32);
define_vec3!(SeatOffset, Of32);
define_vec3!(SeatFront, i32);
define_vec3!(SeatUp, i32);
define_vec3!(SeatCamera, Of32);
define_vec3!(SeatRender, Of32);
define_vec3!(ForceDir, Of32);
define_vec3!(LightPosition, i32);
define_vec3!(LightColor, Of32);
define_vec3!(LightForward, Of32);
define_vec3!(DoorSize, Of32);
define_vec3!(DoorNormal, Of32);
define_vec3!(DoorSide, Of32);
define_vec3!(DoorUp, Of32);
define_vec3!(DoorBasePos, Of32);
define_vec3!(DynamicBodyPosition, i32);
define_vec3!(DynamicRotationAxes, Of32);
define_vec3!(DynamicSideAxis, Of32);
define_vec3!(MagnetOffset, Of32);
define_vec3!(ConnectorAxis, i32);
define_vec3!(ConnectorUp, i32);
define_vec3!(ParticleDirection, i32);
define_vec3!(ParticleOffset, Of32);
define_vec3!(ParticleBounds, Of32);
define_vec3!(SeatExitPosition, i32);
define_vec3!(WeaponBreechPosition, Of32);
define_vec3!(WeaponBreechNormal, Of32);
define_vec3!(WeaponCartPosition, Of32);
define_vec3!(WeaponCartVelocity, Of32);
define_vec3!(RopeHookOffset, Of32);

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
