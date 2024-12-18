use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[cfg(feature = "jsbindings")]
use tsify::Tsify;

#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
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

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub struct Conversion {
    pub value: f64,
    pub to: Unit,
    pub meta_data: Option<MetaData>,
}

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub struct Source {
    pub name: String,
    pub url: Option<String>,
}

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub enum ReferenceSource<T> {
    Actual(T),
    Reference(Reference),
}

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub struct Reference {
    pub uri: String,
    pub format: Option<String>,
    pub version: Option<String>,
    pub overrides: Option<HashMap<String, String>>,
}

pub type MetaData = HashMap<String, Value>;
