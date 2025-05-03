use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(feature = "pybindings")]
use pyo3::prelude::*;

#[cfg(feature = "jsbindings")]
use tsify_next::Tsify;

#[derive(Deserialize, Serialize, JsonSchema, Clone, PartialEq)]
#[serde(rename_all = "camelCase", untagged)]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
#[cfg_attr(feature = "pybindings", derive(FromPyObject, IntoPyObject))]
pub enum AnyValue {
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
#[serde(rename_all = "camelCase", untagged)]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
#[cfg_attr(feature = "pybindings", derive(FromPyObject, IntoPyObject))]
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
            None => AnyValue::String("".to_string()),
            Some(_value) => AnyValue::String(_value.to_string()),
        }
    }
}
