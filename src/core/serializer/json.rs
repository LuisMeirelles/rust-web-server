use std::{collections::HashMap, fmt::Display};

#[allow(dead_code)]
pub enum JsonValue {
    String(String),
    Float(f64),
    Int(i64),
    Boolean(bool),
}

impl From<i64> for JsonValue {
    fn from(value: i64) -> Self {
        JsonValue::Int(value.into())
    }
}

impl From<String> for JsonValue {
    fn from(value: String) -> Self {
        JsonValue::String(value)
    }
}

impl From<f64> for JsonValue {
    fn from(value: f64) -> Self {
        JsonValue::Float(value)
    }
}

impl From<bool> for JsonValue {
    fn from(value: bool) -> Self {
        JsonValue::Boolean(value)
    }
}

impl Display for JsonValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::String(v) => format!("\"{}\"", v),
                Self::Float(v) => format!("{}", v),
                Self::Int(v) => format!("{}", v),
                Self::Boolean(v) => format!("{}", v),
            }
        )
    }
}

pub struct Json {
    value: HashMap<String, JsonValue>,
}

impl Json {
    pub fn new() -> Self {
        Self {
            value: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: impl Into<JsonValue>) {
        self.value.insert(key, value.into());
    }

    pub fn serialize(&self) -> String {
        format!(
            "{{{}}}",
            self.value
                .iter()
                .map(|(key, value)| format!("\"{}\":{}", key, value))
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
