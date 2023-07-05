use std::collections::HashMap;

use epdx::epd::{EPD, ImpactCategory, Unit};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct LCAxProject {
    id: String,
    name: String,
    description: String,
    comment: Option<String>,
    location: String,
    format_version: String,
    lcia_method: Option<String>,
    meta_data: Option<HashMap<String, String>>,
    classification_system: Option<String>,
    life_cycle_stages: Vec<LifeCycleStage>,
    impact_categories: Vec<ImpactCategory>,
    emission_parts: HashMap<String, Assembly>,
    results: Option<Results>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
enum LifeCycleStage {
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
    results: Option<Results>,
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
pub enum Results {
    EN15804(ResultsEN15804),
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct ResultsEN15804 {
    adpe: Option<ImpactCategory>,
    adpf: Option<ImpactCategory>,
    ap: Option<ImpactCategory>,
    cru: Option<ImpactCategory>,
    eee: Option<ImpactCategory>,
    eet: Option<ImpactCategory>,
    ep: Option<ImpactCategory>,
    fw: Option<ImpactCategory>,
    gwp: Option<ImpactCategory>,
    hwd: Option<ImpactCategory>,
    mer: Option<ImpactCategory>,
    mrf: Option<ImpactCategory>,
    nhwd: Option<ImpactCategory>,
    nrsf: Option<ImpactCategory>,
    odp: Option<ImpactCategory>,
    penre: Option<ImpactCategory>,
    penrm: Option<ImpactCategory>,
    penrt: Option<ImpactCategory>,
    pere: Option<ImpactCategory>,
    perm: Option<ImpactCategory>,
    pert: Option<ImpactCategory>,
    pocp: Option<ImpactCategory>,
    rsf: Option<ImpactCategory>,
    rwd: Option<ImpactCategory>,
    sm: Option<ImpactCategory>,
}