use crate::assembly::AssemblyReference;
use crate::life_cycle_base::{ImpactCategoryKey, Impacts, LifeCycleStage};
use crate::shared::{MetaData, Unit};
use lcax_core::country::Country;
use lcax_core::utils::get_version;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(feature = "jsbindings")]
use tsify::Tsify;

#[cfg(feature = "pybindings")]
use pyo3::exceptions::PyTypeError;

#[cfg(feature = "pybindings")]
use pyo3::prelude::*;

#[cfg(feature = "pybindings")]
use pyo3::types::PyType;

#[derive(Deserialize, Serialize, JsonSchema, Default, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "jsbindings",
    derive(Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
#[cfg_attr(feature = "pybindings", pyclass(get_all, set_all))]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub comment: Option<String>,
    pub location: Location,
    pub owner: Option<String>,
    pub format_version: String,
    pub lcia_method: Option<String>,
    pub classification_systems: Option<Vec<String>>,
    pub reference_study_period: Option<u8>,
    pub life_cycle_stages: Vec<LifeCycleStage>,
    pub impact_categories: Vec<ImpactCategoryKey>,
    pub assemblies: Vec<AssemblyReference>,
    pub results: Option<Impacts>,
    pub project_info: Option<ProjectInfo>,
    pub project_phase: ProjectPhase,
    pub software_info: SoftwareInfo,
    pub meta_data: Option<MetaData>,
}

#[cfg_attr(feature = "pybindings", pymethods)]
impl Project {
    #[cfg(feature = "pybindings")]
    #[new]
    #[pyo3(signature=(name, location, project_phase, software_info, life_cycle_stages, impact_categories, assemblies, id=None, description=None, comment=None, owner=None, format_version=None, lcia_method=None, classification_systems=None, reference_study_period=None, results=None, project_info=None, meta_data=None ))]
    pub fn new_py(
        name: &str,
        location: Location,
        project_phase: ProjectPhase,
        software_info: SoftwareInfo,
        life_cycle_stages: Vec<LifeCycleStage>,
        impact_categories: Vec<ImpactCategoryKey>,
        assemblies: Vec<AssemblyReference>,
        id: Option<String>,
        description: Option<String>,
        comment: Option<String>,
        owner: Option<String>,
        format_version: Option<String>,
        lcia_method: Option<String>,
        classification_systems: Option<Vec<String>>,
        reference_study_period: Option<u8>,
        results: Option<Impacts>,
        project_info: Option<ProjectInfo>,
        meta_data: Option<MetaData>,
    ) -> Project {
        Project::new(
            id,
            name,
            description,
            comment,
            location,
            owner,
            format_version,
            lcia_method,
            classification_systems,
            reference_study_period,
            life_cycle_stages,
            impact_categories,
            assemblies,
            results,
            project_info,
            project_phase,
            software_info,
            meta_data,
        )
    }

    #[cfg(feature = "pybindings")]
    #[classmethod]
    #[pyo3(name = "loads")]
    pub fn loads_py(_cls: &Bound<'_, PyType>, value: &str) -> PyResult<Self> {
        match Project::loads(value) {
            Ok(project) => Ok(project),
            Err(error) => Err(PyTypeError::new_err(error.to_string())),
        }
    }

    #[cfg(feature = "pybindings")]
    #[pyo3(name = "dumps")]
    pub fn dumps_py(&self) -> PyResult<String> {
        match Project::dumps(self) {
            Ok(project) => Ok(project),
            Err(error) => Err(PyTypeError::new_err(error.to_string())),
        }
    }
}

impl Project {
    pub fn new(
        id: Option<String>,
        name: &str,
        description: Option<String>,
        comment: Option<String>,
        location: Location,
        owner: Option<String>,
        format_version: Option<String>,
        lcia_method: Option<String>,
        classification_systems: Option<Vec<String>>,
        reference_study_period: Option<u8>,
        life_cycle_stages: Vec<LifeCycleStage>,
        impact_categories: Vec<ImpactCategoryKey>,
        assemblies: Vec<AssemblyReference>,
        results: Option<Impacts>,
        project_info: Option<ProjectInfo>,
        project_phase: ProjectPhase,
        software_info: SoftwareInfo,
        meta_data: Option<MetaData>,
    ) -> Self {
        let _id = id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let _format_version = format_version.unwrap_or_else(|| get_version().to_string());
        Project {
            id: _id,
            name: name.to_string(),
            description,
            comment,
            location,
            owner,
            format_version: _format_version,
            lcia_method,
            classification_systems,
            reference_study_period,
            life_cycle_stages,
            impact_categories,
            assemblies,
            results,
            project_info,
            project_phase,
            software_info,
            meta_data,
        }
    }

    pub fn loads(value: &str) -> Result<Project, serde_json::Error> {
        serde_json::from_str(value)
    }

    pub fn dumps(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn resolve_impact_categories(&mut self) {
        self.impact_categories = match self.results {
            Some(ref results) => {
                let mut keys = Vec::new();
                for (key, _) in results.iter() {
                    keys.push(key.clone());
                }
                keys
            }
            None => vec![],
        }
    }
    pub fn resolve_life_cycle_stages(&mut self) {
        self.life_cycle_stages = match self.results {
            Some(ref results) => {
                let mut keys = Vec::new();
                for (_, value) in results.iter() {
                    for (key, _) in value.iter() {
                        if keys.contains(key) {
                            continue;
                        }
                        keys.push(key.clone());
                    }
                }
                keys
            }
            None => vec![],
        }
    }
}

#[derive(Deserialize, Serialize, JsonSchema, Default, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
#[cfg_attr(feature = "pybindings", pyclass(get_all, set_all))]
pub struct SoftwareInfo {
    pub lca_software: String,
    pub lca_software_version: Option<String>,
    pub goal_and_scope_definition: Option<String>,
    pub calculation_type: Option<String>,
}

#[cfg_attr(feature = "pybindings", pymethods)]
impl SoftwareInfo {
    #[cfg(feature = "pybindings")]
    #[new]
    #[pyo3(signature=(lca_software, lca_software_version=None, goal_and_scope_definition=None, calculation_type=None))]
    pub fn new_py(
        lca_software: String,
        lca_software_version: Option<String>,
        goal_and_scope_definition: Option<String>,
        calculation_type: Option<String>,
    ) -> SoftwareInfo {
        SoftwareInfo::new(
            lca_software,
            lca_software_version,
            goal_and_scope_definition,
            calculation_type,
        )
    }
}

impl SoftwareInfo {
    pub fn new(
        lca_software: String,
        lca_software_version: Option<String>,
        goal_and_scope_definition: Option<String>,
        calculation_type: Option<String>,
    ) -> Self {
        SoftwareInfo {
            lca_software,
            lca_software_version,
            goal_and_scope_definition,
            calculation_type,
        }
    }
}

#[derive(Deserialize, Serialize, JsonSchema, Default, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
#[cfg_attr(feature = "pybindings", pyclass(eq, eq_int))]
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

#[derive(Deserialize, Serialize, JsonSchema, Default, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
#[cfg_attr(feature = "pybindings", pyclass(get_all, set_all))]
pub struct Location {
    pub country: Country,
    pub city: Option<String>,
    pub address: Option<String>,
}

#[cfg_attr(feature = "pybindings", pymethods)]
impl Location {
    #[cfg(feature = "pybindings")]
    #[new]
    #[pyo3(signature=(country, city=None, address=None))]
    pub fn new_py(country: Country, city: Option<String>, address: Option<String>) -> Location {
        Location::new(country, city, address)
    }
}

impl Location {
    pub fn new(country: Country, city: Option<String>, address: Option<String>) -> Self {
        Location {
            country,
            city,
            address,
        }
    }
}

#[derive(Deserialize, Serialize, JsonSchema, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
#[cfg_attr(feature = "pybindings", pyclass(eq))]
pub enum ProjectInfo {
    BuildingInfo(BuildingInfo),
    InfrastructureInfo(HashMap<String, String>),
}

#[derive(Deserialize, Serialize, JsonSchema, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
#[cfg_attr(feature = "pybindings", pyclass(get_all, set_all))]
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
    pub building_completion_year: Option<u16>,
    pub building_permit_year: Option<u16>,
    pub energy_demand_heating: Option<f64>,
    pub energy_supply_heating: Option<f64>,
    pub energy_demand_electricity: Option<f64>,
    pub energy_supply_electricity: Option<f64>,
    pub exported_electricity: Option<f64>,
    pub general_energy_class: GeneralEnergyClass,
    pub local_energy_class: Option<String>,
    pub building_users: Option<u32>,
    pub building_model_scope: Option<Vec<BuildingModelScope>>,
}

#[derive(Deserialize, Serialize, JsonSchema, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
#[cfg_attr(feature = "pybindings", pyclass(get_all, set_all))]
pub struct AreaType {
    pub value: f64,
    pub unit: Unit,
    pub definition: String,
}

#[derive(Deserialize, Serialize, JsonSchema, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
#[cfg_attr(feature = "pybindings", pyclass(get_all, set_all))]
pub struct ValueUnit {
    pub value: f64,
    pub unit: Unit,
}

#[derive(Deserialize, Serialize, JsonSchema, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
#[cfg_attr(feature = "pybindings", pyclass(eq, eq_int))]
pub enum RoofType {
    FLAT,
    PITCHED,
    SADDLE,
    PYRAMID,
    UNKNOWN,
    OTHER,
}

#[derive(Deserialize, Serialize, JsonSchema, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
#[cfg_attr(feature = "pybindings", pyclass(eq, eq_int))]
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

#[derive(Deserialize, Serialize, JsonSchema, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
#[cfg_attr(feature = "pybindings", pyclass(eq, eq_int))]
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

#[derive(Deserialize, Serialize, JsonSchema, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
#[cfg_attr(feature = "pybindings", pyclass(eq, eq_int))]
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

#[derive(Deserialize, Serialize, JsonSchema, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
#[cfg_attr(feature = "pybindings", pyclass(eq, eq_int))]
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
