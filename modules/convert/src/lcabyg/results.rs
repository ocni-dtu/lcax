use std::collections::HashMap;

use field_access::FieldAccess;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LCAbygResults {
    pub model: Vec<Model>,
    pub results: HashMap<String, ModelResult>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case", untagged)]
pub enum Model {
    InstanceModel(InstanceModel),
    Other(OtherModel),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct InstanceModel {
    pub id: String,
    pub node_type: InstanceModelType,
    pub parent: String,
    pub model_id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OtherModel {
    id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum InstanceModelType {
    ElementInstance,
    OperationUtilityInstance,
    ElementModelInstance,
    ElementSuperCategoryInstance,
    ElementCategoryInstance,
    ConstructionInstance,
    ProductInstance,
    StageInstance,
}

#[derive(Serialize, Deserialize, FieldAccess)]
#[serde(rename_all = "PascalCase")]
pub struct ModelResult {
    #[serde(alias = "a1to3")]
    pub a1a3: Option<YearResult>,
    pub d: Option<YearResult>,
    pub c3: Option<YearResult>,
    pub c4: Option<YearResult>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct YearResult {
    #[serde(alias = "9999")]
    pub end_of_time: ImpactResult,
}

#[derive(Serialize, Deserialize, FieldAccess)]
#[serde(rename_all = "UPPERCASE")]
pub struct ImpactResult {
    pub ap: f64,
    pub adpf: f64,
    pub adpe: f64,
    pub odp: f64,
    pub gwp: f64,
    pub penr: f64,
    pub per: f64,
    pub ep: f64,
    pub pocp: f64,
}
