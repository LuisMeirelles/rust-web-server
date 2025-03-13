use std::{collections::HashMap, fmt::Display};

#[allow(dead_code)]
pub enum JsonValue {
    String(String),
    Float(f64),
    Int(i64),
    Boolean(bool),
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

pub struct JsonSerializer;

impl JsonSerializer {
    pub fn serialize(data: HashMap<String, JsonValue>) -> String {
        format!(
            "{{{}}}",
            data.iter()
                .map(|(key, value)| format!("\"{}\":{}", key, value))
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
