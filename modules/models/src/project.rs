use std::collections::HashMap;

use lcax_core::country::Country;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[cfg(feature = "jsbindings")]
use tsify::Tsify;

use crate::assembly::Assembly;
use crate::life_cycle_base::{ImpactCategoryKey, LifeCycleStage, Results};
use crate::shared::{MetaData, ReferenceSource, Unit};

#[derive(Deserialize, Serialize, JsonSchema, Default, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "jsbindings",
    derive(Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub comment: Option<String>,
    pub location: Location,
    pub owner: Option<String>,
    pub format_version: String,
    pub lcia_method: Option<String>,
    pub classification_system: Option<String>,
    pub reference_study_period: Option<u8>,
    pub life_cycle_stages: Vec<LifeCycleStage>,
    pub impact_categories: Vec<ImpactCategoryKey>,
    pub assemblies: HashMap<String, ReferenceSource<Assembly>>,
    pub results: Option<Results>,
    pub project_info: Option<ProjectInfo>,
    pub project_phase: ProjectPhase,
    pub software_info: SoftwareInfo,
    pub meta_data: Option<MetaData>,
}

impl Project {
    pub fn new() -> Self {
        Project {
            id: uuid::Uuid::new_v4().to_string(),
            name: "".to_string(),
            description: None,
            comment: None,
            location: Location {
                country: Country::UNKNOWN,
                city: None,
                address: None,
            },
            owner: None,
            format_version: "0.1.0".to_string(),
            lcia_method: None,
            classification_system: None,
            reference_study_period: None,
            life_cycle_stages: vec![],
            impact_categories: vec![],
            assemblies: HashMap::new(),
            results: None,
            project_info: None,
            project_phase: ProjectPhase::STRATEGIC_DESIGN,
            software_info: SoftwareInfo {
                goal_and_scope_definition: None,
                lca_software: "lcax".to_string(),
                calculation_type: None,
            },
            meta_data: None,
        }
    }
}

#[derive(Deserialize, Serialize, JsonSchema, Default, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub struct SoftwareInfo {
    pub goal_and_scope_definition: Option<String>,
    pub lca_software: String,
    pub calculation_type: Option<String>,
}

#[derive(Deserialize, Serialize, JsonSchema, Default, Clone)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub enum ProjectPhase {
    #[allow(non_camel_case_types)]
    STRATEGIC_DESIGN,
    #[allow(non_camel_case_types)]
    CONCEPT_DESIGN,
    #[allow(non_camel_case_types)]
    TECHNICAL_DESIGN,
    CONSTRUCTION,
    #[allow(non_camel_case_types)]
    POST_COMPLETION,
    #[allow(non_camel_case_types)]
    IN_USE,
    #[default]
    OTHER,
}

#[derive(Deserialize, Serialize, JsonSchema, Default, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub struct Location {
    pub country: Country,
    pub city: Option<String>,
    pub address: Option<String>,
}

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub enum ProjectInfo {
    BuildingInfo(BuildingInfo),
    InfrastructureInfo(HashMap<String, String>),
}

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub struct BuildingInfo {
    pub building_type: BuildingType,
    pub building_typology: Vec<BuildingTypology>,
    pub certifications: Option<Vec<String>>,
    pub building_mass: Option<ValueUnit>,
    pub building_height: Option<ValueUnit>,
    pub gross_floor_area: Option<AreaType>,
    pub heated_floor_area: Option<AreaType>,
    pub building_footprint: Option<ValueUnit>,
    pub floors_above_ground: u16,
    pub floors_below_ground: Option<u16>,
    pub roof_type: RoofType,
    pub frame_type: Option<String>,
    pub building_completion_year: Option<u64>,
    pub building_permit_year: Option<u64>,
    pub energy_demand_heating: Option<f64>,
    pub energy_supply_heating: Option<f64>,
    pub energy_demand_electricity: Option<f64>,
    pub energy_supply_electricity: Option<f64>,
    pub exported_electricity: Option<f64>,
    pub general_energy_class: GeneralEnergyClass,
    pub local_energy_class: Option<String>,
    pub building_users: Option<u64>,
    pub building_model_scope: Option<Vec<BuildingModelScope>>,
}

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub struct AreaType {
    pub value: f64,
    pub unit: Unit,
    pub definition: String,
}

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub struct ValueUnit {
    value: f64,
    unit: Unit,
}

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub enum RoofType {
    FLAT,
    PITCHED,
    SADDLE,
    PYRAMID,
    UNKNOWN,
    OTHER,
}

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub enum GeneralEnergyClass {
    EXISTING,
    STANDARD,
    ADVANCED,
    UNKNOWN,
}

impl From<&String> for GeneralEnergyClass {
    fn from(class: &String) -> Self {
        match class.to_ascii_lowercase().as_str() {
            "lowenergy" => GeneralEnergyClass::ADVANCED,
            _ => GeneralEnergyClass::UNKNOWN,
        }
    }
}

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub enum BuildingModelScope {
    #[allow(non_camel_case_types)]
    FACILITATING_WORKS,
    SUBSTRUCTURE,
    #[allow(non_camel_case_types)]
    SUPERSTRUCTURE_FRAME,
    #[allow(non_camel_case_types)]
    SUPERSTRUCTURE_ENVELOPE,
    #[allow(non_camel_case_types)]
    SUPERSTRUCTURE_INTERNAL_ELEMENTS,
    FINISHES,
    #[allow(non_camel_case_types)]
    BUILDING_SERVICES,
    #[allow(non_camel_case_types)]
    EXTERNAL_WORKS,
    #[allow(non_camel_case_types)]
    FF_E,
}

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub enum BuildingType {
    #[allow(non_camel_case_types)]
    NEW_CONSTRUCTION_WORKS,
    DEMOLITION,
    #[allow(non_camel_case_types)]
    DECONSTRUCTION_AND_NEW_CONSTRUCTION_WORKS,
    #[allow(non_camel_case_types)]
    RETROFIT_WORKS,
    #[allow(non_camel_case_types)]
    EXTENSION_WORKS,
    #[allow(non_camel_case_types)]
    RETROFIT_AND_EXTENSION_WORKS,
    #[allow(non_camel_case_types)]
    FIT_OUT_WORKS,
    OPERATIONS,
    UNKNOWN,
    OTHER,
}

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub enum BuildingTypology {
    OFFICE,
    RESIDENTIAL,
    PUBLIC,
    COMMERCIAL,
    INDUSTRIAL,
    INFRASTRUCTURE,
    AGRICULTURAL,
    EDUCATIONAL,
    HEALTH,
    UNKNOWN,
    OTHER,
}

impl From<&String> for BuildingTypology {
    fn from(unit: &String) -> Self {
        match unit.to_ascii_lowercase().as_str() {
            "office" => BuildingTypology::OFFICE,
            "residential" => BuildingTypology::RESIDENTIAL,
            "public" => BuildingTypology::PUBLIC,
            "commercial" => BuildingTypology::COMMERCIAL,
            "industrial" => BuildingTypology::INDUSTRIAL,
            "infrastructure" => BuildingTypology::INFRASTRUCTURE,
            "agricultural" => BuildingTypology::AGRICULTURAL,
            "educational" => BuildingTypology::EDUCATIONAL,
            "health" => BuildingTypology::HEALTH,
            _ => BuildingTypology::OTHER,
        }
    }
}
