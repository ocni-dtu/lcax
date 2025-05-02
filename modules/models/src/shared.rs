use lcax_core::value::AnyValue;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[cfg(feature = "jsbindings")]
use tsify_next::{declare, Tsify};

#[cfg(feature = "pybindings")]
use pyo3::prelude::*;

#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
#[cfg_attr(feature = "pybindings", pyclass(eq, eq_int))]
pub enum Unit {
    M,
    M2,
    M3,
    KG,
    TONES,
    PCS,
    KWH,
    L,
    M2R1,
    KM,
    #[allow(non_camel_case_types)]
    TONES_KM,
    KGM3,
    UNKNOWN,
}

impl From<&String> for Unit {
    fn from(unit: &String) -> Self {
        match unit.to_ascii_lowercase().as_str() {
            "m" => Unit::M,
            "m2" | "m^2" | "qm" => Unit::M2,
            "m3" | "m^3" => Unit::M3,
            "km" => Unit::KM,
            "kg" => Unit::KG,
            "tones" | "tonnes" => Unit::TONES,
            "pcs" | "stk" | "pcs." => Unit::PCS,
            "l" => Unit::L,
            "kwh" => Unit::KWH,
            "m2r1" => Unit::M2R1,
            "tones*km" => Unit::TONES_KM,
            "kgm3" => Unit::KGM3,
            _ => Unit::UNKNOWN,
        }
    }
}
impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for Unit {
    fn default() -> Unit {
        Unit::UNKNOWN
    }
}

#[derive(Deserialize, Serialize, JsonSchema, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
#[cfg_attr(feature = "pybindings", pyclass(get_all, set_all))]
pub struct Conversion {
    pub value: f64,
    pub to: Unit,
    pub meta_data: Option<MetaData>,
}

#[cfg_attr(feature = "pybindings", pymethods)]
impl Conversion {
    #[cfg(feature = "pybindings")]
    #[new]
    #[pyo3(signature = (to, value, meta_data=None))]
    pub fn new_py(to: Unit, value: f64, meta_data: Option<MetaData>) -> Self {
        Self {
            to,
            value,
            meta_data,
        }
    }

    #[cfg(feature = "pybindings")]
    fn __repr__(&self) -> String {
        format!("Conversion: {}", self.to)
    }

    #[cfg(feature = "pybindings")]
    fn __str__(&self) -> String {
        format!("Conversion: {}", self.to)
    }
}

#[derive(Deserialize, Serialize, JsonSchema, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
#[cfg_attr(feature = "pybindings", pyclass(get_all, set_all))]
pub struct Source {
    pub name: String,
    pub url: Option<String>,
}

#[cfg_attr(feature = "pybindings", pymethods)]
impl Source {
    #[cfg(feature = "pybindings")]
    #[new]
    #[pyo3(signature = (name, url=None))]
    pub fn new_py(name: String, url: Option<String>) -> Self {
        Self { name, url }
    }
    #[cfg(feature = "pybindings")]
    fn __repr__(&self) -> String {
        format!("Source: {}", self.name)
    }

    #[cfg(feature = "pybindings")]
    fn __str__(&self) -> String {
        format!("Source: {}", self.name)
    }
}

#[derive(Deserialize, Serialize, JsonSchema, Clone, PartialEq)]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
#[cfg_attr(feature = "pybindings", pyclass(get_all, set_all))]
pub struct Reference {
    pub uri: String,
    pub format: Option<String>,
    pub version: Option<String>,
    pub overrides: Option<HashMap<String, Option<AnyValue>>>,
}

#[cfg_attr(feature = "jsbindings", declare)]
pub type MetaData = HashMap<String, Option<AnyValue>>;
