use field_access::FieldAccess;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Node {
    ConstructionProcess(ConstructionProcess),
    Product(Product),
    Construction(Construction),
    ElementModel(ElementModel),
    ConstructionInstallation(ConstructionInstallation),
    FuelConsumption(FuelConsumption),
    DGNBOperationReference(DGNBOperationReference),
    Element(Element),
    EmbodiedRoot(EmbodiedRoot),
    Building(Building),
    Stage(epdx::lcabyg::Stage),
    Operation(Operation),
    ProductTransportRoot(ProductTransportRoot),
    Project(Project),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Element {
    pub id: String,
    pub name: Languages,
    pub source: String,
    pub comment: Languages,
    pub enabled: bool,
    pub excluded_scenarios: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Construction {
    pub id: String,
    pub name: Languages,
    pub unit: String,
    pub source: String,
    pub comment: Languages,
    pub locked: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct DGNBOperationReference {
    pub id: String,
    pub heat_supplement: f64,
    pub electricity_supplement: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Building {
    pub id: String,
    pub scenario_name: String,
    pub locked: String,
    pub description: Languages,
    pub building_type: String,
    pub heated_floor_area: f64,
    pub gross_area: f64,
    pub integrated_garage: f64,
    pub external_area: f64,
    pub gross_area_above_ground: f64,
    pub person_count: f64,
    pub storeys_above_ground: u16,
    pub storeys_below_ground: u16,
    pub storey_height: f64,
    pub initial_year: u32,
    pub calculation_timespan: u16,
    pub calculation_mode: String,
    pub outside_area: f64,
    pub plot_area: f64,
    pub energy_class: String,
}

#[derive(Serialize, Deserialize, FieldAccess)]
#[serde(rename_all = "UPPERCASE")]
pub struct StageIndicators {
    pub ser: f64,
    pub ep: f64,
    pub odp: f64,
    pub pocp: f64,
    pub per: f64,
    pub adpe: f64,
    pub ap: f64,
    pub gwp: f64,
    pub adpf: f64,
    pub penr: f64,
    pub senr: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ProductTransportRoot {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Project {
    pub id: String,
    pub name: Languages,
    pub owner: String,
    pub address: String,
    pub lca_advisor: String,
    pub building_regulation_version: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Operation {
    pub id: String,
    pub electricity_usage: f64,
    pub heat_usage: f64,
    pub electricity_production: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EmbodiedRoot {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FuelConsumption {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ConstructionInstallation {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ElementModel {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ConstructionProcess {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Product {
    pub id: String,
    pub name: Languages,
    pub source: String,
    pub comment: Languages,
    pub uncertainty_factor: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Languages {
    pub english: Option<String>,
    pub german: Option<String>,
    pub norwegian: Option<String>,
    pub danish: Option<String>,
}
