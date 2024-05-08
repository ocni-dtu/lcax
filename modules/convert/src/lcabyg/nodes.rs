use field_access::FieldAccess;
use lcax_core::country::Country;
use lcax_core::utils::get_version;
use lcax_models::epd::{Standard, SubType, EPD};
use lcax_models::life_cycle_base::{ImpactCategory, ImpactCategoryKey, LifeCycleStage};
use lcax_models::shared::{Conversion, Source, Unit};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    Stage(Stage),
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

#[derive(Serialize, Deserialize, FieldAccess, Clone)]
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

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Languages {
    pub english: Option<String>,
    pub german: Option<String>,
    pub norwegian: Option<String>,
    pub danish: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Stage {
    pub id: String,
    pub name: Languages,
    pub comment: Languages,
    pub source: String,
    pub valid_to: String,
    pub stage: String,
    pub stage_unit: String,
    pub stage_factor: f64,
    pub mass_factor: f64,
    pub scale_factor: f64,
    pub external_source: String,
    pub external_id: String,
    pub external_version: String,
    pub external_url: String,
    pub compliance: String,
    pub data_type: String,
    pub indicators: StageIndicators,
}

pub fn epd_from_lcabyg_stages(stages: &Vec<Stage>) -> EPD {
    let mut impacts = HashMap::from([
        (ImpactCategoryKey::EP, ImpactCategory::new()),
        (ImpactCategoryKey::ODP, ImpactCategory::new()),
        (ImpactCategoryKey::POCP, ImpactCategory::new()),
        (ImpactCategoryKey::PERT, ImpactCategory::new()),
        (ImpactCategoryKey::ADPE, ImpactCategory::new()),
        (ImpactCategoryKey::AP, ImpactCategory::new()),
        (ImpactCategoryKey::GWP, ImpactCategory::new()),
        (ImpactCategoryKey::ADPF, ImpactCategory::new()),
        (ImpactCategoryKey::PENRT, ImpactCategory::new()),
    ]);

    for stage in stages {
        for (category_key, impact_category) in &mut impacts {
            let mut category_name = category_key.to_string().to_lowercase();
            if category_name == "penrt" {
                category_name = String::from("penr");
            } else if category_name == "pert" {
                category_name = String::from("per");
            }
            match impact_category.get(&LifeCycleStage::try_from(stage.stage.as_str()).unwrap()) {
                None => {
                    impact_category.insert(
                        LifeCycleStage::try_from(stage.stage.as_str()).unwrap(),
                        Some(
                            stage
                                .indicators
                                .field(&category_name)
                                .unwrap()
                                .as_f64()
                                .unwrap(),
                        ),
                    );
                }
                Some(stage_value) => {
                    let value = stage
                        .indicators
                        .field(&category_name)
                        .unwrap()
                        .as_f64()
                        .unwrap();
                    match stage_value {
                        None => impact_category.insert(
                            LifeCycleStage::try_from(stage.stage.as_str()).unwrap(),
                            Some(value),
                        ),
                        Some(_stage_value) => impact_category.insert(
                            LifeCycleStage::try_from(stage.stage.as_str()).unwrap(),
                            Some(value + _stage_value),
                        ),
                    };
                }
            }
        }
    }

    let node = &stages[0];
    let epd = EPD {
        id: node.id.to_string(),
        name: node.name.english.clone().unwrap(),
        declared_unit: Unit::from(&node.stage_unit),
        version: node.external_version.clone(),
        published_date: Default::default(),
        valid_until: Default::default(),
        comment: node.comment.english.clone(),
        source: Some(Source {
            name: node.external_source.clone(),
            url: Some(node.external_url.clone()),
        }),
        subtype: SubType::from(&Some(node.data_type.clone())),
        standard: if node.compliance == "A1" {
            Standard::EN15804A1
        } else {
            Standard::EN15804A2
        },
        impacts,
        format_version: get_version(),
        reference_service_life: None,
        location: Country::DNK,
        conversions: Some(vec![Conversion {
            to: Unit::KG,
            value: node.mass_factor,
            meta_data: "".to_string(),
        }]),
        meta_data: None,
    };

    epd
}
