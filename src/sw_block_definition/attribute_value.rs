pub type Of32 = ordered_float::NotNan<f32>;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum AttributeValue {
    Bool(bool),
    I32(i32),
    U64(u64),
    Of32(Of32),
    String(String),
}

impl AttributeValue {
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

    pub fn is_number(&self) -> bool {
        matches!(self, Self::I32(_) | Self::U64(_) | Self::Of32(_))
    }
}

impl From<bool> for AttributeValue {
    fn from(value: bool) -> Self {
        AttributeValue::Bool(value)
    }
}

impl From<i32> for AttributeValue {
    fn from(value: i32) -> Self {
        AttributeValue::I32(value)
    }
}

impl From<u64> for AttributeValue {
    fn from(value: u64) -> Self {
        AttributeValue::U64(value)
    }
}

impl From<Of32> for AttributeValue {
    fn from(value: Of32) -> Self {
        AttributeValue::Of32(value)
    }
}

impl From<String> for AttributeValue {
    fn from(value: String) -> Self {
        AttributeValue::String(value)
    }
}
