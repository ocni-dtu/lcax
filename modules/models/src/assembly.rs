use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[cfg(feature = "jsbindings")]
use tsify::Tsify;

use crate::life_cycle_base::Results;
use crate::product::Product;
use crate::shared::{MetaData, ReferenceSource, Unit};

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub struct Assembly {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub comment: Option<String>,
    pub quantity: f64,
    pub unit: Unit,
    pub classification: Option<Vec<Classification>>,
    pub products: HashMap<String, ReferenceSource<Product>>,
    pub results: Option<Results>,
    pub meta_data: Option<MetaData>,
}

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub struct Classification {
    pub system: String,
    pub code: String,
    pub name: String,
}
