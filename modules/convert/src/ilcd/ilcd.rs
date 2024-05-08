#[allow(dead_code)]
use serde::Deserialize;
use serde::Serialize;

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

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialProperty {
    pub name: String,
    pub value: String,
    pub unit: String,
    pub unit_description: Option<String>,
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

impl From<&AnieValue> for f64 {
    fn from(value: &AnieValue) -> Self {
        match value {
            AnieValue::ValueString(s) => {
                // Parse the string into a float
                let float_value = s.parse::<f64>().unwrap();
                float_value
            }
            AnieValue::ValueObject(_) => {
                panic!("Cannot convert AnieValue::ValueObject to f64");
            }
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
    pub other: Anies,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Anies {
    pub anies: Vec<Anie>,
}

#[derive(Deserialize)]
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
