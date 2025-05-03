use field_access::FieldAccess;
use lcax_core::country::Country;
use lcax_core::utils::get_version;
use lcax_models::epd::{Standard, SubType, EPD};
use lcax_models::life_cycle_base::{ImpactCategory, ImpactCategoryKey, Impacts, LifeCycleModule};
use lcax_models::shared::{Conversion, Source, Unit};
use serde::{Deserialize, Serialize};

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
    Transport(Transport),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Transport {
    pub id: String,
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
    pub initial_year: u16,
    pub calculation_timespan: u16,
    pub calculation_mode: String,
    pub outside_area: f64,
    pub plot_area: f64,
    pub energy_class: String,
    pub project_type: Option<String>,
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

impl StageIndicators {
    pub fn new() -> Self {
        Self {
            ser: 0.0,
            ep: 0.0,
            odp: 0.0,
            pocp: 0.0,
            per: 0.0,
            adpe: 0.0,
            ap: 0.0,
            gwp: 0.0,
            adpf: 0.0,
            penr: 0.0,
            senr: 0.0,
        }
    }

    pub fn update(&mut self, key: &str, value: &f64) {
        match key {
            "ser" => self.ser = value.clone(),
            "ep" => self.ep = value.clone(),
            "odp" => self.odp = value.clone(),
            "pocp" => self.pocp = value.clone(),
            "per" => self.per = value.clone(),
            "adpe" => self.adpe = value.clone(),
            "ap" => self.ap = value.clone(),
            "gwp" => self.gwp = value.clone(),
            "adpf" => self.adpf = value.clone(),
            "penr" => self.penr = value.clone(),
            "senr" => self.senr = value.clone(),
            _ => log::warn!("Could not find impact key {} in stage indicator", key),
        }
    }
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

#[derive(Serialize, Deserialize, Clone)]
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

impl Languages {
    pub fn get(&self) -> String {
        if self.english != None {
            self.english.clone().unwrap()
        } else if self.danish != None {
            self.danish.clone().unwrap()
        } else if self.german != None {
            self.german.clone().unwrap()
        } else if self.norwegian != None {
            self.norwegian.clone().unwrap()
        } else {
            "".to_string()
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Stage {
    pub id: String,
    pub name: Languages,
    pub comment: Languages,
    pub source: String,
    pub valid_to: Option<String>,
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

impl Stage {
    pub fn new(epd: &EPD, stage_name: &str, impact_key: &str, value: &f64) -> Self {
        let mut indicators = StageIndicators::new();
        indicators.update(impact_key, value);

        Self {
            id: epd.id.clone(),
            name: Languages {
                english: Some(epd.name.clone()),
                german: None,
                norwegian: None,
                danish: None,
            },
            comment: Languages {
                english: epd.comment.clone(),
                german: None,
                norwegian: None,
                danish: None,
            },
            source: "User".to_string(),
            valid_to: Some(epd.valid_until.to_string()),
            stage: stage_name.to_string(),
            stage_unit: epd.declared_unit.to_string(),
            stage_factor: 1.0,
            mass_factor: epd.conversions.to_owned().unwrap().first().unwrap().value,
            scale_factor: 1.0,
            external_source: epd.source.to_owned().unwrap().name,
            external_url: epd.source.to_owned().unwrap().url.unwrap_or("".to_string()),
            external_id: "".to_string(),
            external_version: "".to_string(),
            compliance: to_lcabyg_compliance(&epd.standard),
            data_type: epd.subtype.clone().into(),
            indicators,
        }
    }

    pub fn update_indicator(&mut self, key: &str, value: &f64) {
        self.indicators.update(key, value);
    }
}

fn to_lcabyg_compliance(standard: &Standard) -> String {
    match standard {
        Standard::EN15804A2 => "15804+a2".to_string(),
        Standard::EN15804A1 => "15804".to_string(),
        Standard::UNKNOWN => "unknown".to_string(),
    }
}

pub fn epd_from_lcabyg_stages(stages: &Vec<Stage>) -> EPD {
    let mut impacts = Impacts::from([
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
        for (category_key, impact_category) in impacts.iter_mut() {
            let mut category_name = category_key.to_string().to_lowercase();
            if category_name == "penrt" {
                category_name = String::from("penr");
            } else if category_name == "pert" {
                category_name = String::from("per");
            }
            match impact_category.get(&LifeCycleModule::try_from(stage.stage.as_str()).unwrap()) {
                None => {
                    impact_category.insert(
                        LifeCycleModule::try_from(stage.stage.as_str()).unwrap(),
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
                            LifeCycleModule::try_from(stage.stage.as_str()).unwrap(),
                            Some(value),
                        ),
                        Some(_stage_value) => impact_category.insert(
                            LifeCycleModule::try_from(stage.stage.as_str()).unwrap(),
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
            meta_data: None,
        }]),
        meta_data: None,
    };

    epd
}
