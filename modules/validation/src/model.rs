use field_access::FieldAccess;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use valitron::available::{Range as ValitronRange, Required, Message, MessageKind};

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Range {
    pub min: f64,
    pub max: f64,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum ValidationRules {
    Range(Range),
    Required,
}

// impl ValidationRules {
//     pub fn to_rule(&self) -> Message {
//         match self {
//             ValidationRules::Range(range) => Message::new(MessageKind::Range(ValitronRange::new(range.min..range.max))),
//             ValidationRules::Required => Required,
//         }
//     }
// }

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum NestedValidationRules<T> {
    Rules(ValidationRules),
    Nested(T),
}

#[derive(Deserialize, Serialize, JsonSchema, FieldAccess)]
#[serde(rename_all = "camelCase")]
pub struct ValidationSchema {
    pub id: Option<ValidationRules>,
    pub name: Option<ValidationRules>,
    pub description: Option<ValidationRules>,
    pub comment: Option<ValidationRules>,
    pub location: Option<NestedValidationRules<Location>>,
    pub owner: Option<ValidationRules>,
    pub format_version: Option<ValidationRules>,
    pub lcia_method: Option<ValidationRules>,
    pub classification_systems: Option<ValidationRules>,
    pub reference_study_period: Option<ValidationRules>,
    pub life_cycle_stages: Option<ValidationRules>,
    pub impact_categories: Option<ValidationRules>,
    // pub assemblies: Option<NestedValidationRules<AssemblyReference>>,
    // pub results: Option<NestedValidationRules<Impacts>>,
    // pub project_info: Option<NestedValidationRules<ProjectInfo>>,
    pub project_phase: Option<ValidationRules>,
    pub software_info: Option<ValidationRules>,
    pub meta_data: Option<ValidationRules>,
}

#[derive(Deserialize, Serialize, JsonSchema, FieldAccess)]
pub struct Location {
    pub country: Option<ValidationRules>,
    pub city: Option<ValidationRules>,
    pub address: Option<ValidationRules>,
}
