use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(feature = "jsbindings")]
use tsify::Tsify;

use crate::epd::EPD;
use crate::life_cycle_base::{ImpactCategoryKey, LifeCycleStage, Results};
use crate::shared::{Reference, ReferenceType, Unit};
use crate::techflow::TechFlow;

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub enum ProductSource {
    Product(Product),
    Reference(Reference),
}

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub reference_service_life: u32,
    pub impact_data: ImpactDataSource,
    pub quantity: f64,
    pub unit: Unit,
    pub transport: Option<Vec<Transport>>,
    pub results: Results,
    pub meta_data: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub struct Transport {
    pub id: String,
    pub name: String,
    pub life_cycle_stages: Vec<LifeCycleStage>,
    pub distance: f64,
    pub distance_unit: Unit,
    pub impact_data: ImpactDataSource,
}

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub enum ImpactDataSource {
    #[serde(rename = "EPD")]
    EPD(EPD),
    TechFlow(TechFlow),
    Reference(Reference),
}

impl Default for ImpactDataSource {
    fn default() -> ImpactDataSource {
        ImpactDataSource::Reference(Reference {
            _type: ReferenceType::EXTERNAL,
            path: "".to_string(),
            format: None,
            version: None,
            overrides: None,
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
