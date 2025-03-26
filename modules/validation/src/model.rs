use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cfg(feature = "jsbindings")]
use tsify_next::Tsify;

#[cfg(feature = "pybindings")]
use pyo3::prelude::*;

#[derive(Deserialize, Serialize, JsonSchema, Debug, Clone)]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
#[cfg_attr(feature = "pybindings", pyclass)]
#[serde(rename_all = "camelCase")]
pub enum Level {
    Project,
    Assembly,
    Product,
    ImpactData,
}

#[derive(Deserialize, Serialize, JsonSchema, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "jsbindings",
    derive(Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
#[cfg_attr(feature = "pybindings", pyclass(get_all, set_all))]
pub struct ValidationSchema {
    pub level: Level,
    pub field: String,
    pub rule: ValidationRules,
}

#[cfg_attr(feature = "pybindings", pymethods)]
impl ValidationSchema {
    #[cfg(feature = "pybindings")]
    #[new]
    pub fn new_py(level: Level, field: String, rule: ValidationRules) -> Self {
        Self { level, field, rule }
    }
}

#[derive(Deserialize, Serialize, JsonSchema, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "jsbindings",
    derive(Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
#[cfg_attr(feature = "pybindings", pyclass(get_all, set_all))]
pub struct ValidationRules {
    pub range: Option<[f64; 2]>,
    pub includes: Option<String>,
    pub required: Option<bool>,
    pub equal: Option<String>,
    pub greater: Option<f64>,
    pub less: Option<f64>,
    pub one_of: Option<Vec<String>>,
}

#[cfg_attr(feature = "pybindings", pymethods)]
impl ValidationRules {
    #[cfg(feature = "pybindings")]
    #[new]
    #[pyo3(signature=(range=None, includes=None, required=None, equal=None, greater=None, less=None, one_of=None))]
    pub fn new_py(
        range: Option<[f64; 2]>,
        includes: Option<String>,
        required: Option<bool>,
        equal: Option<String>,
        greater: Option<f64>,
        less: Option<f64>,
        one_of: Option<Vec<String>>,
    ) -> Self {
        Self {
            range,
            includes,
            required,
            equal,
            greater,
            less,
            one_of,
        }
    }
}
