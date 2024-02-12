use serde::{Deserialize, Serialize};
use crate::lcabyg::nodes::LCAbygUnit;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum EdgeType {
    ProcessToInstallation(String),
    ProcessToTransport(String),
    ProductToStage(ProductToStageEdge),
    CategoryToStage(String),
    ConstructionToProduct(ConstructionToProductEdge),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct  ProductToStageEdge {
    pub id: String,
    pub excluded_scenarios: Vec<String>,
    pub enabled: bool
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct  ConstructionToProductEdge {
    pub id: String,
    pub amount: f64,
    pub unit: LCAbygUnit,
    pub lifespan: u32,
    pub demolition: bool,
    pub delayed_start: u32,
    pub enabled: bool,
    pub excluded_scenarios: Vec<String>,
}