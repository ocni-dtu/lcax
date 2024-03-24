use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum EdgeType {
    ProcessToInstallation(String),
    ProcessToTransport(String),
    ProductToStage(ProductToStageEdge),
    CategoryToStage(String),
    ConstructionToProduct(ConstructionToProductEdge),
    ParentTo(String),
    FuelUsage(String),
    InstallationOperation(String),
    TransportTypeUsage(TransportTypeUsageEdge),
    ElementToConstruction(ElementToConstructionEdge),
    RootToModel(String),
    RootToConstructionProcess(String),
    BuildingToRoot(String),
    BuildingToOperation(String),
    BuildingToDGNBOperation(String),
    ElectricitySource(String),
    HeatingSource(String),
    CategoryToElement(CategoryToElementEdge),
    CategoryToConstruction(CategoryToConstructionEdge),
    MainBuilding(String),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProductToStageEdge {
    pub id: String,
    pub excluded_scenarios: Vec<String>,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ConstructionToProductEdge {
    pub id: String,
    pub amount: f64,
    pub unit: String,
    pub lifespan: u32,
    pub demolition: bool,
    pub delayed_start: u32,
    pub enabled: bool,
    pub excluded_scenarios: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TransportTypeUsageEdge {
    pub id: String,
    pub distance: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ElementToConstructionEdge {
    pub id: String,
    pub amount: f64,
    pub enabled: bool,
    pub special_conditions: bool,
    pub excluded_scenarios: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CategoryToElementEdge {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CategoryToConstructionEdge {
    pub id: String,
    pub layers: Vec<u32>,
}
