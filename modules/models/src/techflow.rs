use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[cfg(feature = "jsbindings")]
use tsify::Tsify;

use lcax_core::country::Country;
use lcax_core::utils::get_version;

use crate::life_cycle_base::{ImpactCategory, ImpactCategoryKey};
use crate::shared::{Conversion, Source, Unit};

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
#[cfg_attr(
    feature = "jsbindings",
    derive(Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
pub struct TechFlow {
    pub id: String,
    pub name: String,
    pub declared_unit: Unit,
    pub format_version: String,
    pub source: Option<Source>,
    pub comment: Option<String>,
    pub location: Country,
    pub conversions: Option<Vec<Conversion>>,
    pub impacts: HashMap<ImpactCategoryKey, ImpactCategory>,
    pub meta_data: Option<HashMap<String, String>>,
}

impl TechFlow {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: "".to_string(),
            declared_unit: Unit::UNKNOWN,
            format_version: get_version(),
            source: None,
            comment: None,
            location: Country::UNKNOWN,
            conversions: None,
            impacts: HashMap::new(),
            meta_data: None,
        }
    }
}
