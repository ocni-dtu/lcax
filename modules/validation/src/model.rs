use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Level {
    Project,
    Assembly,
    Product,
    ImpactData
}

#[derive(Deserialize, Serialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ValidationSchema {
    pub level: Level,
    pub field: String,
    pub rule: ValidationRules
}


#[derive(Deserialize, Serialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Range {
    pub min: f64,
    pub max: f64,
}

// #[derive(Deserialize, Serialize, JsonSchema, Debug)]
// #[serde(rename_all = "camelCase")]
// pub enum ValidationRules {
//     Range(Range),
//     Includes(String),
//     Required,
//     Equal(String),
// }

#[derive(Deserialize, Serialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct  ValidationRules {
    pub range: Option<Range>,
    pub includes: Option<String>,
    pub required: Option<bool>,
    pub equal: Option<String>,
}