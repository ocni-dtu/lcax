use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(feature = "jsbindings")]
use tsify::Tsify;

use crate::epd::EPD;
use crate::life_cycle_base::Results;
use crate::shared::Unit;
use crate::techflow::TechFlow;

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: String,
    pub reference_service_life: u32,
    pub impact_data: ImpactDataSource,
    pub quantity: f64,
    pub unit: Unit,
    pub transport: Option<Transport>,
    pub results: Results,
    pub meta_data: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub struct Transport {
    pub id: String,
    pub name: String,
    pub distance: f64,
    pub distance_unit: Unit,
    pub transport_epd: ImpactDataSource,
}

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub enum ImpactDataSource {
    EPD(EPD),
    TechFlow(TechFlow),
    ExternalImpactData(ExternalImpactData),
    InternalImpactData(InternalImpactData),
}

impl Default for ImpactDataSource {
    fn default() -> ImpactDataSource {
        ImpactDataSource::ExternalImpactData(ExternalImpactData {
            url: "".to_string(),
            format: "".to_string(),
            version: None,
        })
    }
}

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub struct ExternalImpactData {
    pub url: String,
    pub format: String,
    pub version: Option<String>,
}

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub struct InternalImpactData {
    pub path: String,
}
