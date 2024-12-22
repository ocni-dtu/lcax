use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cfg(feature = "jsbindings")]
use tsify::Tsify;

use crate::epd::EPD;
use crate::generic_impact_data::GenericData;
use crate::life_cycle_base::{LifeCycleStage, Results};
use crate::shared::{MetaData, Reference, ReferenceSource, Unit};

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub reference_service_life: u32,
    pub impact_data: ReferenceSource<ImpactDataSource>,
    pub quantity: f64,
    pub unit: Unit,
    pub transport: Option<Vec<Transport>>,
    pub results: Option<Results>,
    pub meta_data: Option<MetaData>,
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
#[serde(untagged)]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub enum ImpactDataSource {
    #[serde(rename = "EPD")]
    EPD(EPD),
    GenericData(GenericData),
}

impl Default for ReferenceSource<ImpactDataSource> {
    fn default() -> ReferenceSource<ImpactDataSource> {
        ReferenceSource::Reference(Reference {
            uri: "".to_string(),
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
