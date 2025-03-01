pub type Of32 = ordered_float::NotNan<f32>;

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
