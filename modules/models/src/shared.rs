use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cfg(feature = "jsbindings")]
use tsify::Tsify;

#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone)]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub enum Unit {
    M,
    M2,
    M3,
    KG,
    TONES,
    PCS,
    L,
    M2R1,
    KM,
    #[allow(non_camel_case_types)]
    TONES_KM,
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
            "m2r1" => Unit::M2R1,
            "tones*km" => Unit::TONES_KM,
            _ => Unit::UNKNOWN,
        }
    }
}

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub struct Conversion {
    pub value: f64,
    pub to: Unit,
    pub meta_data: String,
}

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub struct Source {
    pub name: String,
    pub url: Option<String>,
}
