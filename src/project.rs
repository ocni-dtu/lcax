use std::collections::HashMap;

use epdx::epd::{EPD, Unit};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct LCAxProject {
    pub id: String,
    pub name: String,
    pub description: String,
    pub comment: Option<String>,
    pub location: Location,
    pub owner: String,
    pub format_version: String,
    pub lcia_method: Option<String>,
    pub classification_system: Option<String>,
    pub reference_study_period: Option<u8>,
    pub life_cycle_stages: Vec<LifeCycleStage>,
    pub impact_categories: Vec<ImpactCategoryKey>,
    pub assemblies: HashMap<String, Assembly>,
    pub results: Option<HashMap<ImpactCategoryKey, HashMap<LifeCycleStage, f64>>>,
    pub project_info: Option<ProjectInfo>,
    pub project_phase: ProjectPhase,
    pub software_info: SoftwareInfo,
    pub meta_data: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SoftwareInfo {
    pub goal_and_scope_definition: String,
    pub lca_software: String,
    pub calculation_type: String
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum ProjectPhase {
    DESIGN,
    ONGOING,
    BUILT,
    OTHER
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub country: String,
    pub city: String,
    pub address: String
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum ProjectInfo {
    BuildingInfo(BuildingInfo),
    InfrastructureInfo(HashMap<String, String>),
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BuildingInfo {
    pub building_type: BuildingType,
    pub building_typology: BuildingTypology,
    pub certifications: String,
    pub building_mass: String,
    pub gross_floor_area: f64,
    pub gross_floor_area_definition: String,
    pub heated_floor_area: f64,
    pub heated_floor_area_definition: String,
    pub floors_above_ground: u16,
    pub floors_below_ground: u16,
    pub frame_type: String,
    pub building_completion_year: u64,
    pub energy_demand_heating: f64,
    pub energy_supply_heating: f64,
    pub energy_demand_electricity: f64,
    pub energy_supply_electricity: f64,
    pub exported_electricity: f64,
    pub energy_class: String,
    pub building_model_scope: BuildingModelScope
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub struct BuildingModelScope {
    pub facilitating_works: bool,
    pub substructure: bool,
    pub superstructure_frame: bool,
    pub superstructure_envelope: bool,
    pub superstructure_internal_elements: bool,
    pub finishes: bool,
    pub building_services: bool,
    pub external_works: bool,
    pub ff_e: bool
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum BuildingType {
    RENOVATION,
    NEW
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum BuildingTypology {
    OFFICE,
    RESIDENTIAL,
    PUBLIC,
    COMMERCIAL,
    INDUSTRIAL,
    INFRASTRUCTURE,
    AGRICULTURAL,
    OTHER
}

#[derive(Deserialize, Serialize, JsonSchema, Hash, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LifeCycleStage {
    A1A3,
    A4,
    A5,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    C1,
    C2,
    C3,
    C4,
    D,
}

#[derive(Deserialize, Serialize, JsonSchema, Hash, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ImpactCategoryKey {
    GWP,
    ODP,
    AP,
    EP,
    POCP,
    ADPE,
    ADPF,
    PENRE,
    PERE,
    PERM,
    PERT,
    PENRT,
    PENRM,
    SM,
    RSF,
    NRSF,
    FW,
    HWD,
    NHWD,
    RWD,
    CRU,
    MRF,
    MER,
    EEE,
    EET
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Assembly {
    id: String,
    name: String,
    description: String,
    comment: Option<String>,
    quantity: f64,
    unit: Unit,
    category: Option<String>,
    classification: Option<Vec<Classification>>,
    products: HashMap<String, EPDProduct>,
    results: Option<HashMap<ImpactCategoryKey, HashMap<LifeCycleStage, f64>>>,
    meta_data: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct EPDProduct {
    id: String,
    name: String,
    description: String,
    reference_service_life: f64,
    epd_source: EPDSource,
    quantity: f64,
    unit: Unit,
    transport: Option<Transport>,
    results: Option<HashMap<ImpactCategoryKey, HashMap<LifeCycleStage, f64>>>,
    meta_data: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Transport {
    id: String,
    name: String,
    distance: f64,
    distance_unit: Unit,
    transport_epd: EPDSource,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum EPDSource {
    EPD(EPD),
    ExternalEPD(ExternalEPD),
    InternalEPD(InternalEPD),
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct ExternalEPD {
    url: String,
    format: String,
    version: Option<String>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct InternalEPD {
    path: String,
}


#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Classification {
    system: String,
    code: String,
    name: String,
}