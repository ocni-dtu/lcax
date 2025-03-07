use std::collections::HashMap;

#[cfg(feature = "pybindings")]
use pyo3::pyclass;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cfg(feature = "jsbindings")]
use tsify::Tsify;

#[derive(Deserialize, Serialize, JsonSchema, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
#[cfg_attr(feature = "pybindings", pyclass(eq))]
pub enum AnyValue {
    Null(),
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<AnyValue>),
    Object(HashMap<String, AnyValue>),
}

impl AnyValue {
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            AnyValue::Number(Number::Float(n)) => Some(*n),
            AnyValue::Number(Number::Int(n)) => Some(*n as f64),
            _ => None,
        }
    }
}

#[derive(Deserialize, Serialize, JsonSchema, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
#[cfg_attr(feature = "pybindings", pyclass(eq))]
pub enum Number {
    Int(i64),
    Float(f64),
}

impl From<String> for AnyValue {
    fn from(value: String) -> AnyValue {
        AnyValue::String(value)
    }
}

impl From<Option<String>> for AnyValue {
    fn from(value: Option<String>) -> AnyValue {
        match value {
            None => AnyValue::Null(),
            Some(_value) => AnyValue::String(_value.to_string()),
        }
    }
}
