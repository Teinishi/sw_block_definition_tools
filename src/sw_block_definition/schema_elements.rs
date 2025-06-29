use super::{
    AttributeSpecifier, AttributeType, AttributeValue, Definition, DefinitionVec3,
    GetAttributeValue, GetAttributeValueRoot, Of32,
};
use paste::paste;
use serde::{Deserialize, Serialize};

macro_rules! define_vec3 {
    ($name:ident, $type:ty) => {
        #[derive(Serialize, Deserialize, Default, Debug, Clone, Copy)]
        #[serde(default)]
        pub struct $name {
            #[serde(rename = "@x")]
            pub x: Option<$type>,
            #[serde(rename = "@y")]
            pub y: Option<$type>,
            #[serde(rename = "@z")]
            pub z: Option<$type>,
        }

        impl From<$name> for DefinitionVec3<$type> {
            fn from(value: $name) -> DefinitionVec3<$type> {
                DefinitionVec3 {
                    x: value.x,
                    y: value.y,
                    z: value.z,
                }
            }
        }

        paste! {
            #[derive(
                Serialize,
                Deserialize,
                PartialEq,
                strum::Display,
                strum::VariantArray,
                Clone,
                Copy,
            )]
            #[strum(serialize_all = "snake_case")]
            pub enum [<$name Attribute>] {
                X,
                Y,
                Z,
            }

            impl GetAttributeValue<$name> for [<$name Attribute>] {
                fn get_value(&self, d: &$name) -> Option<AttributeValue> {
                    match self {
                        Self::X => Some(d.x?.into()),
                        Self::Y => Some(d.y?.into()),
                        Self::Z => Some(d.z?.into()),
                    }
                }
            }

            impl From<[<$name Attribute>]> for AttributeSpecifier {
                fn from(value: [<$name Attribute>]) -> Self {
                    Self::$name(value)
                }
            }
        }
    };
}

macro_rules! define_vec3_int {
    ($name:ident) => {
        define_vec3!($name, i32);

        impl From<$name> for AttributeValue {
            fn from(value: $name) -> AttributeValue {
                Self::VecI32(value.into())
            }
        }

        impl From<$name> for glam::Vec3 {
            fn from(value: $name) -> glam::Vec3 {
                glam::Vec3::new(
                    value.x.unwrap_or_default() as f32,
                    value.y.unwrap_or_default() as f32,
                    value.z.unwrap_or_default() as f32,
                )
            }
        }

        paste! {
            impl GetAttributeValueRoot for [<$name Attribute>] {
                fn get_value_root(&self, d: &Definition) -> Vec<AttributeValue> {
                    if let Some(item) = d.[<$name:snake>].last() {
                        self.get_value(item).into_iter().collect()
                    } else {
                        vec![]
                    }
                }

                fn get_type(&self) -> AttributeType {
                    AttributeType::Int
                }
            }
        }
    };
}

macro_rules! define_vec3_float {
    ($name:ident) => {
        define_vec3!($name, Of32);

        impl From<$name> for AttributeValue {
            fn from(value: $name) -> AttributeValue {
                Self::VecF32(value.into())
            }
        }

        impl From<$name> for glam::Vec3 {
            fn from(value: $name) -> glam::Vec3 {
                glam::Vec3::new(
                    value.x.unwrap_or_default().into(),
                    value.y.unwrap_or_default().into(),
                    value.z.unwrap_or_default().into(),
                )
            }
        }

        paste! {
            impl GetAttributeValueRoot for [<$name Attribute>] {
                fn get_value_root(&self, d: &Definition) -> Vec<AttributeValue> {
                    if let Some(item) = d.[<$name:snake>].last() {
                        self.get_value(item).into_iter().collect()
                    } else {
                        vec![]
                    }
                }

                fn get_type(&self) -> AttributeType {
                    AttributeType::Float
                }
            }
        }
    };
}

//define_vec3!(Normal, NormalAttribute, i32);
define_vec3_int!(VoxelMin);
define_vec3_int!(VoxelMax);
define_vec3_int!(VoxelPhysicsMin);
define_vec3_int!(VoxelPhysicsMax);
define_vec3_float!(BbPhysicsMin);
define_vec3_float!(BbPhysicsMax);
define_vec3_int!(CompartmentSamplePos);
define_vec3_float!(ConstraintPosParent);
define_vec3_float!(ConstraintPosChild);
define_vec3_int!(VoxelLocationChild);
define_vec3_float!(SeatOffset);
define_vec3_float!(SeatFront);
define_vec3_float!(SeatUp);
define_vec3_float!(SeatCamera);
define_vec3_float!(SeatRender);
define_vec3_float!(ForceDir);
define_vec3_int!(LightPosition);
define_vec3_float!(LightColor);
define_vec3_float!(LightForward);
define_vec3_float!(DoorSize);
define_vec3_float!(DoorNormal);
define_vec3_float!(DoorSide);
define_vec3_float!(DoorUp);
define_vec3_float!(DoorBasePos);
define_vec3_int!(DynamicBodyPosition);
define_vec3_float!(DynamicRotationAxes);
define_vec3_float!(DynamicSideAxis);
define_vec3_float!(MagnetOffset);
define_vec3_int!(ConnectorAxis);
define_vec3_int!(ConnectorUp);
define_vec3_int!(ParticleDirection);
define_vec3_float!(ParticleOffset);
define_vec3_float!(ParticleBounds);
define_vec3_int!(SeatExitPosition);
define_vec3_float!(WeaponBreechPosition);
define_vec3_float!(WeaponBreechNormal);
define_vec3_float!(WeaponCartPosition);
define_vec3_float!(WeaponCartVelocity);
define_vec3_float!(RopeHookOffset);

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
    pub pos: Vec<DefinitionVec3<i32>>,
    pub normal: Vec<DefinitionVec3<i32>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct TooltipProperties {
    #[serde(rename = "@description")]
    pub description: Option<String>,
    #[serde(rename = "@short_description")]
    pub short_description: Option<String>,
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
pub enum TooltipPropertiesAttribute {
    Description,
    ShortDescription,
}
impl GetAttributeValueRoot for TooltipPropertiesAttribute {
    fn get_value_root(&self, d: &Definition) -> Vec<AttributeValue> {
        if let Some(tooltip_properties) = d.tooltip_properties.last() {
            self.get_value(tooltip_properties).into_iter().collect()
        } else {
            vec![]
        }
    }

    fn get_type(&self) -> AttributeType {
        AttributeType::String
    }
}
impl GetAttributeValue<TooltipProperties> for TooltipPropertiesAttribute {
    fn get_value(&self, d: &TooltipProperties) -> Option<AttributeValue> {
        match self {
            Self::Description => Some(d.description.clone()?.into()),
            Self::ShortDescription => Some(d.short_description.clone()?.into()),
        }
    }
}
impl From<TooltipPropertiesAttribute> for AttributeSpecifier {
    fn from(value: TooltipPropertiesAttribute) -> Self {
        Self::TooltipProperties(value)
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct RewardProperties {
    #[serde(rename = "@tier")]
    pub tier: Option<i32>,
    #[serde(rename = "@number_rewarded")]
    pub number_rewarded: Option<i32>,
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
pub enum RewardPropertiesAttribute {
    Tier,
    NumberRewarded,
}
impl GetAttributeValueRoot for RewardPropertiesAttribute {
    fn get_value_root(&self, d: &Definition) -> Vec<AttributeValue> {
        if let Some(reward_properties) = d.reward_properties.last() {
            self.get_value(reward_properties).into_iter().collect()
        } else {
            vec![]
        }
    }

    fn get_type(&self) -> AttributeType {
        AttributeType::Int
    }
}
impl GetAttributeValue<RewardProperties> for RewardPropertiesAttribute {
    fn get_value(&self, d: &RewardProperties) -> Option<AttributeValue> {
        match self {
            Self::Tier => Some(d.tier?.into()),
            Self::NumberRewarded => Some(d.number_rewarded?.into()),
        }
    }
}
impl From<RewardPropertiesAttribute> for AttributeSpecifier {
    fn from(value: RewardPropertiesAttribute) -> Self {
        Self::RewardProperties(value)
    }
}
