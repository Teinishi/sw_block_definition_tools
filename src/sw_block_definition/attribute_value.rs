use ambassador::{delegatable_trait, Delegate};
use std::fmt::{self, Debug, Display};
use strum::EnumIs;

pub type Of32 = ordered_float::NotNan<f32>;

#[delegatable_trait]
pub trait DisplayAttributeValue: Debug {
    fn display_string(&self) -> String {
        format!("{:?}", self)
    }
}
impl DisplayAttributeValue for bool {}
impl DisplayAttributeValue for i32 {}
impl DisplayAttributeValue for u64 {}
impl DisplayAttributeValue for Of32 {}
impl DisplayAttributeValue for String {}

#[delegatable_trait]
pub trait IsDefault {
    fn is_default(&self) -> bool;
}
impl IsDefault for bool {
    fn is_default(&self) -> bool {
        !*self
    }
}
impl IsDefault for i32 {
    fn is_default(&self) -> bool {
        *self == 0
    }
}
impl IsDefault for u64 {
    fn is_default(&self) -> bool {
        *self == 0
    }
}
impl IsDefault for Of32 {
    fn is_default(&self) -> bool {
        *self == 0.0
    }
}
impl IsDefault for String {
    fn is_default(&self) -> bool {
        self.is_empty()
    }
}

#[derive(
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
)]
#[serde(default)]
pub struct DefinitionVec3<T> {
    #[serde(rename = "@x")]
    pub x: Option<T>,
    #[serde(rename = "@y")]
    pub y: Option<T>,
    #[serde(rename = "@z")]
    pub z: Option<T>,
}
impl<T: Copy + std::convert::Into<AttributeValue>> DefinitionVec3<T> {
    pub fn x_as_attribute_value(&self) -> Option<AttributeValue> {
        self.x.map(|x| x.into())
    }
    pub fn y_as_attribute_value(&self) -> Option<AttributeValue> {
        self.y.map(|y| y.into())
    }
    pub fn z_as_attribute_value(&self) -> Option<AttributeValue> {
        self.z.map(|z| z.into())
    }
    pub fn as_attribute_values(&self) -> [Option<AttributeValue>; 3] {
        [
            self.x_as_attribute_value(),
            self.y_as_attribute_value(),
            self.z_as_attribute_value(),
        ]
    }
}
impl<T: Copy + Default + PartialEq + Display + Debug> IsDefault for DefinitionVec3<T> {
    fn is_default(&self) -> bool {
        self.x.is_none_or(|v| v == Default::default())
            && self.y.is_none_or(|v| v == Default::default())
            && self.z.is_none_or(|v| v == Default::default())
    }
}
impl<T: Copy + Default + PartialEq + Display + Debug> Display for DefinitionVec3<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        if let Some(x) = self.x {
            write!(f, "{:2}, ", x)?;
        } else {
            write!(f, " _, ")?;
        }
        if let Some(y) = self.y {
            write!(f, "{:2}, ", y)?;
        } else {
            write!(f, " _, ")?;
        }
        if let Some(z) = self.z {
            write!(f, "{:2})", z)
        } else {
            write!(f, " _)")
        }
    }
}
impl<T: Copy + Default + PartialEq + Display + Debug> DisplayAttributeValue for DefinitionVec3<T> {
    fn display_string(&self) -> String {
        format!("{}", self)
    }
}

fn one() -> i32 {
    1
}

#[derive(
    serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy,
)]
#[serde(default)]
pub struct Matrix {
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
impl Matrix {
    const IDENTITY: Self = Self {
        r00: 1,
        r01: 0,
        r02: 0,
        r10: 0,
        r11: 1,
        r12: 0,
        r20: 0,
        r21: 0,
        r22: 1,
    };
}
impl Default for Matrix {
    fn default() -> Self {
        Self::IDENTITY
    }
}
impl IsDefault for Matrix {
    fn is_default(&self) -> bool {
        self == &Self::IDENTITY
    }
}
impl Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}, {} / {}, {}, {} / {}, {}, {})",
            self.r00,
            self.r01,
            self.r02,
            self.r10,
            self.r11,
            self.r12,
            self.r20,
            self.r21,
            self.r22
        )
    }
}
impl DisplayAttributeValue for Matrix {
    fn display_string(&self) -> String {
        if self.is_default() {
            "Identity".to_string()
        } else {
            format!("{}", self)
        }
    }
}

#[derive(EnumIs, Debug)]
pub enum AttributeType {
    Bool,
    Int,
    Float,
    String,
    Flags,
    AudioFile,
    MeshFile,
    VecInt,
    VecFloat,
    Matrix,
}

impl AttributeType {
    pub fn is_number(&self) -> bool {
        matches!(self, Self::Int | Self::Flags | Self::Float)
    }

    pub fn undefined_text(&self) -> &str {
        match self {
            Self::Bool => "false",
            Self::Int | Self::Flags => "0",
            Self::Float => "0.0",
            _ => "undefined",
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Delegate)]
#[delegate(DisplayAttributeValue)]
#[delegate(IsDefault)]
pub enum AttributeValue {
    Bool(bool),
    I32(i32),
    U64(u64),
    F32(Of32),
    String(String),
    VecI32(DefinitionVec3<i32>),
    VecF32(DefinitionVec3<Of32>),
    Matrix(Matrix),
}

impl AttributeValue {
    pub fn is_number(&self) -> bool {
        matches!(self, Self::I32(_) | Self::U64(_) | Self::F32(_))
    }

    pub fn vec_as_attribute_values(&self) -> Option<[Option<AttributeValue>; 3]> {
        match self {
            Self::VecI32(v) => Some(v.as_attribute_values()),
            Self::VecF32(v) => Some(v.as_attribute_values()),
            _ => None,
        }
    }
}

impl From<bool> for AttributeValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<i32> for AttributeValue {
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}

impl From<u64> for AttributeValue {
    fn from(value: u64) -> Self {
        Self::U64(value)
    }
}

impl From<Of32> for AttributeValue {
    fn from(value: Of32) -> Self {
        Self::F32(value)
    }
}

impl From<String> for AttributeValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<DefinitionVec3<i32>> for AttributeValue {
    fn from(value: DefinitionVec3<i32>) -> Self {
        Self::VecI32(value)
    }
}

impl From<DefinitionVec3<Of32>> for AttributeValue {
    fn from(value: DefinitionVec3<Of32>) -> Self {
        Self::VecF32(value)
    }
}

impl From<Matrix> for AttributeValue {
    fn from(value: Matrix) -> Self {
        Self::Matrix(value)
    }
}
