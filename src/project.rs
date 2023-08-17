use std::collections::HashMap;

use epdx::epd::{EPD, Unit};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct LCAxProject {
    pub id: String,
    pub name: String,
    pub description: String,
    pub comment: Option<String>,
    pub location: String,
    pub format_version: String,
    pub lcia_method: Option<String>,
    pub classification_system: Option<String>,
    pub life_span: Option<u8>,
    pub life_cycle_stages: Vec<LifeCycleStage>,
    pub impact_categories: Vec<ImpactCategoryKey>,
    pub emission_parts: HashMap<String, Assembly>,
    pub results: Option<HashMap<ImpactCategoryKey, HashMap<LifeCycleStage, f64>>>,
    pub meta_data: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Serialize, JsonSchema, Hash, Eq, PartialEq)]
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
pub struct Assembly {
    id: String,
    name: String,
    description: String,
    comment: Option<String>,
    quantity: f64,
    unit: Unit,
    category: Option<String>,
    classification: Option<Vec<Classification>>,
    parts: HashMap<String, EPDPart>,
    results: Option<HashMap<ImpactCategoryKey, HashMap<LifeCycleStage, f64>>>,
    meta_data: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct EPDPart {
    id: String,
    name: String,
    reference_service_life: f64,
    epd_source: EPDSource,
    part_quantity: f64,
    part_unit: Unit,
    transport: Option<Transport>,
    meta_data: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Transport {
    id: String,
    name: String,
    distance: f64,
    distance_unit: DistanceUnit,
    transport_type: TransportType,
}

#[derive(Deserialize, Serialize, JsonSchema)]
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

#[derive(Deserialize, Serialize, JsonSchema)]
pub enum DistanceUnit {
    M,
    KM
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub enum TransportType {
    Truck,
    Train,
    Ship,
    Plane,
}