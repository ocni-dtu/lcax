use lcax_core::value::AnyValue;
#[allow(dead_code)]
use std::collections::HashMap;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ILCD {
    pub process_information: ProcessInformation,
    pub modelling_and_validation: ModellingAndValidation,
    pub exchanges: Exchanges,
    #[serde(alias = "LCIAResults")]
    pub lcia_results: LCIAResults,
    pub version: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Exchanges {
    pub exchange: Vec<Exchange>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Exchange {
    pub reference_to_flow_data_set: ReferenceToDescription,
    pub mean_amount: Option<f64>,
    pub resulting_amount: Option<f64>,
    #[serde(alias = "dataSetInternalID")]
    pub data_set_internal_id: Option<u32>,
    pub reference_flow: Option<bool>,
    #[serde(alias = "resultingflowAmount")]
    pub resulting_flow_amount: Option<f64>,
    pub flow_properties: Option<Vec<FlowProperty>>,
    pub material_properties: Option<Vec<MaterialProperty>>,

    #[serde(alias = "exchange direction")]
    pub exchange_direction: Option<String>,
    pub other: Option<LCIAAnies>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlowProperty {
    pub name: Vec<ValueLang>,
    pub uuid: String,
    pub mean_value: f64,
    pub reference_flow_property: Option<bool>,
    pub reference_unit: Option<String>,
    #[serde(alias = "unitGroupUUID")]
    pub unit_group_uuid: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MaterialProperty {
    pub name: String,
    pub value: String,
    pub unit: String,
    pub unit_description: Option<String>,
}

impl Into<HashMap<String, AnyValue>> for MaterialProperty {
    fn into(self) -> HashMap<String, AnyValue> {
        HashMap::from([
            ("name".to_string(), AnyValue::from(self.name)),
            ("value".to_string(), AnyValue::from(self.value)),
            ("unit".to_string(), AnyValue::from(self.unit)),
            (
                "unit_description".to_string(),
                AnyValue::from(self.unit_description),
            ),
        ])
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModellingAndValidation {
    #[serde(alias = "LCIMethodAndAllocation")]
    pub lci_method_and_allocation: LCIMethodAndAllocation,
    pub compliance_declarations: ComplianceDeclarations,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComplianceDeclarations {
    pub compliance: Vec<Compliance>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Compliance {
    pub reference_to_compliance_system: ReferenceToDescription,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceToDescription {
    pub short_description: Vec<ValueLang>,
    pub _type: String,
    // pub ref_object_id: Option<String>,
    pub version: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LCIAResults {
    #[serde(alias = "LCIAResult")]
    pub lcia_result: Vec<LCIAResult>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LCIAResult {
    #[serde(alias = "referenceToLCIAMethodDataSet")]
    pub reference_to_lcia_method_dataset: ReferenceToLCIAMethodDataSet,
    pub other: LCIAAnies,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LCIAAnies {
    pub anies: Vec<ModuleAnie>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModuleAnie {
    pub module: Option<String>,
    pub value: Option<AnieValue>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum AnieValue {
    ValueString(String),
    ValueObject(ValueObject),
}

impl TryFrom<&AnieValue> for f64 {
    type Error = ();
    fn try_from(value: &AnieValue) -> Result<Self, Self::Error> {
        match value {
            AnieValue::ValueString(s) => {
                // Parse the string into a float
                match s.parse::<f64>() {
                    Ok(f) => Ok(f),
                    Err(_) => Err(()),
                }
            }
            AnieValue::ValueObject(_) => Err(()),
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueObject {
    _type: String,
    // ref_object_id: String,
}

#[derive(Deserialize)]
pub enum ModuleValue {
    Value(String),
    Name(ModuleMap),
}

#[derive(Deserialize)]
pub struct ModuleMap {
    #[allow(dead_code)]
    name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceToLCIAMethodDataSet {
    pub short_description: Vec<ValueLang>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LCIMethodAndAllocation {
    pub other: Option<Anies>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Anies {
    #[serde(deserialize_with = "ok_or_default")]
    pub anies: Vec<Anie>,
}

fn ok_or_default<'a, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'a> + Default,
    D: Deserializer<'a>,
{
    let v: Value = Deserialize::deserialize(deserializer)?;
    Ok(T::deserialize(v).unwrap_or_default())
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Anie {
    pub name: String,
    pub value: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessInformation {
    pub data_set_information: DataSetInformation,
    pub time: TimeData,
    pub geography: Geography,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Geography {
    pub location_of_operation_supply_or_production: LocationOfOperationSupplyOrProduction,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationOfOperationSupplyOrProduction {
    pub location: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeData {
    pub reference_year: i32,
    pub data_set_valid_until: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSetInformation {
    #[serde(alias = "UUID")]
    pub uuid: String,
    pub name: DataSetName,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSetName {
    pub base_name: Vec<ValueLang>,
}

#[derive(Deserialize, Debug)]
pub struct ValueLang {
    pub value: Option<String>,
    pub lang: String,
}
