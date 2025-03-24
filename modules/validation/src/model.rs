use field_access::FieldAccess;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Range {
    pub min: f64,
    pub max: f64,
}

#[derive(Deserialize, Serialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ValidationRules {
    Range(Range),
    Includes(String),
    Required,
    Equal(f64),
}

// #[derive(Deserialize, Serialize, JsonSchema, Debug)]
// #[serde(rename_all = "camelCase")]
// #[serde(untagged)]
// pub enum NestedValidationRules<T: FieldAccess> {
//     Rules(ValidationRules),
//     Nested(T),
// }

#[derive(Deserialize, Serialize, JsonSchema, FieldAccess)]
#[serde(rename_all = "camelCase")]
pub struct ValidationSchema {
    pub id: Option<ValidationRules>,
    pub name: Option<ValidationRules>,
    pub description: Option<ValidationRules>,
    pub comment: Option<ValidationRules>,
    pub location: Option<Location>,
    pub owner: Option<ValidationRules>,
    pub format_version: Option<ValidationRules>,
    pub lcia_method: Option<ValidationRules>,
    pub classification_systems: Option<ValidationRules>,
    pub reference_study_period: Option<ValidationRules>,
    pub life_cycle_stages: Option<ValidationRules>,
    pub impact_categories: Option<ValidationRules>,
    // pub assemblies: Option<NestedValidationRules<AssemblyReference>>,
    // pub results: Option<NestedValidationRules<Impacts>>,
    pub project_info: Option<BuildingInfo>,
    pub project_phase: Option<ValidationRules>,
    pub software_info: Option<SoftwareInfo>,
    pub meta_data: Option<ValidationRules>,
}

#[derive(Deserialize, Serialize, JsonSchema, FieldAccess)]
pub struct Location {
    pub country: Option<ValidationRules>,
    pub city: Option<ValidationRules>,
    pub address: Option<ValidationRules>,
}

#[derive(Deserialize, Serialize, JsonSchema, FieldAccess)]
pub struct SoftwareInfo {
    pub lca_software: Option<ValidationRules>,
    pub lca_software_version: Option<ValidationRules>,
    pub goal_and_scope_definition: Option<ValidationRules>,
    pub calculation_type: Option<ValidationRules>,
}

#[derive(Deserialize, Serialize, JsonSchema, FieldAccess)]
pub struct BuildingInfo {
    pub building_type: Option<ValidationRules>,
    pub building_typology: Option<ValidationRules>,
    pub certifications: Option<ValidationRules>,
    pub building_mass: Option<ValueUnit>,
    pub building_height: Option<ValueUnit>,
    pub gross_floor_area: Option<AreaType>,
    pub heated_floor_area: Option<AreaType>,
    pub building_footprint: Option<ValueUnit>,
    pub floors_above_ground: Option<ValidationRules>,
    pub floors_below_ground: Option<ValidationRules>,
    pub roof_type: Option<ValidationRules>,
    pub frame_type: Option<ValidationRules>,
    pub building_completion_year: Option<ValidationRules>,
    pub building_permit_year: Option<ValidationRules>,
    pub energy_demand_heating: Option<ValidationRules>,
    pub energy_supply_heating: Option<ValidationRules>,
    pub energy_demand_electricity: Option<ValidationRules>,
    pub energy_supply_electricity: Option<ValidationRules>,
    pub exported_electricity: Option<ValidationRules>,
    pub general_energy_class: Option<ValidationRules>,
    pub local_energy_class: Option<ValidationRules>,
    pub building_users: Option<ValidationRules>,
    pub building_model_scope: Option<ValidationRules>,
}

#[derive(Deserialize, Serialize, JsonSchema, FieldAccess)]
pub struct ValueUnit {
    pub value: Option<ValidationRules>,
    pub unit: Option<ValidationRules>,
}

#[derive(Deserialize, Serialize, JsonSchema, FieldAccess)]
pub struct AreaType {
    pub value: Option<ValidationRules>,
    pub unit: Option<ValidationRules>,
    pub definition: Option<ValidationRules>,
}