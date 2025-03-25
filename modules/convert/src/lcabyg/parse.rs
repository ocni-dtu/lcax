use chrono::{NaiveDate, Utc};
use field_access::FieldAccess;
use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::collections::HashMap;

use lcax_core::country::Country;
use lcax_core::utils::get_version;
use lcax_models::assembly::{Assembly, AssemblyReference, Classification};
use lcax_models::epd::{EPDReference, Standard, SubType, EPD};
use lcax_models::life_cycle_base::{ImpactCategory, ImpactCategoryKey, Impacts, LifeCycleModule};
use lcax_models::product::{ImpactData, Product, ProductReference};
use lcax_models::project::{
    AreaType, BuildingInfo, BuildingType, BuildingTypology, GeneralEnergyClass, Location, Project,
    ProjectInfo, RoofType, SoftwareInfo,
};
use lcax_models::shared::{Conversion, Source, Unit};

use crate::lcabyg::categories;
use crate::lcabyg::edges::EdgeType;
use crate::lcabyg::nodes::{
    Building as LCAbygBuilding, Construction as LCAbygConstruction, Element as LCAbygElement, Node,
    Operation as LCAbygOperation, Product as LCAbygProduct, Project as LCAbygProject,
    Stage as LCAbygStage,
};
use crate::lcabyg::results::{LCAbygResults, Model, YearResult};
use log;

#[cfg(feature = "pybindings")]
use pyo3::prelude::*;

use crate::br_standard::br18_generic_data::{
    get_district_heating_data, get_electricity_data, get_energy_data, get_lng_data,
};
use lcax_core::value::AnyValue;
use lcax_models::generic_impact_data::{GenericData, GenericDataReference};
#[cfg(feature = "jsbindings")]
use tsify::Tsify;

type Edge = (EdgeType, String, String);

#[derive(Deserialize, Serialize)]
pub enum NodesAndEdges {
    Node(Node),
    Edge(Edge),
    Secret(Vec<i32>),
}

#[derive(Deserialize, Serialize, PartialEq)]
#[cfg_attr(
    feature = "jsbindings",
    derive(Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "pybindings", pyclass, derive(FromPyObject))]
pub enum LCABygResult {
    Project(Project),
    Assemblies(Vec<Assembly>),
    Products(Vec<Product>),
    EPDs(Vec<EPD>),
}

/// Parse an LCAByg project exported as json files.
///
/// # Arguments
///
/// * `project_data`: JSON formatted string containing the LCAbyg project data
/// * `result_data`: Optional JSON formatted string containing the result data from the LCAByg project
///
/// returns: LCAxProject
pub fn parse_lcabyg(project_data: &str, result_data: Option<&str>) -> Result<LCABygResult, Error> {
    match serde_json::from_str(project_data) {
        Ok(lcabyg_project) => match lcax_from_lcabyg(lcabyg_project, result_data) {
            Ok(lcabyg_result) => Ok(lcabyg_result),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

fn lcax_from_lcabyg(
    lcabyg_project: Vec<NodesAndEdges>,
    result_data: Option<&str>,
) -> Result<LCABygResult, Error> {
    let mut edges: Vec<Edge> = vec![];
    let mut project = None;
    let mut building = None;
    let mut operations: Vec<LCAbygOperation> = vec![];
    let mut elements: Vec<LCAbygElement> = vec![];
    let mut constructions: Vec<LCAbygConstruction> = vec![];
    let mut products: Vec<LCAbygProduct> = vec![];
    let mut stages: Vec<LCAbygStage> = vec![];
    let mut categories = HashMap::new();

    for element in lcabyg_project {
        match element {
            NodesAndEdges::Node(node) => match node {
                Node::Project(_node) => project = Some(_node),
                Node::Building(_node) => building = Some(_node),
                Node::Operation(_node) => operations.push(_node),
                Node::Element(_node) => elements.push(_node),
                Node::Construction(_node) => constructions.push(_node),
                Node::Product(_node) => products.push(_node),
                Node::Stage(_node) => stages.push(_node),
                _ => {}
            },
            NodesAndEdges::Edge(edge) => match edge {
                (EdgeType::CategoryToElement(_edge), from_id, to_id) => {
                    categories.insert(to_id, from_id);
                }
                _ => edges.append(&mut vec![edge]),
            },
            _ => continue,
        }
    }

    let connections = get_connections(&edges);

    if project.is_some() && building.is_some() {
        log::trace!("Creating LCAx Project from LCAbyg Project");
        let result_objects = match result_data {
            Some(result_data) => {
                log::info!("Found result data for project");
                Some(serde_json::from_str::<LCAbygResults>(result_data)?)
            }
            None => None,
        };

        Ok(LCABygResult::Project(Project::from_lcabyg((
            connections,
            &project.unwrap(),
            &building.unwrap(),
            &operations,
            &elements,
            &categories,
            &constructions,
            &products,
            &stages,
            result_objects.as_ref(),
        ))))
    } else if !products.is_empty() {
        log::trace!("Creating LCAx EPDs from LCAbyg Products");
        Ok(LCABygResult::EPDs(construct_epds(
            &connections,
            &products,
            &stages,
        )))
    } else {
        panic!("No valid LCAbyg nodes")
    }
}

fn get_connections(edges: &Vec<Edge>) -> HashMap<String, Vec<&Edge>> {
    let mut connections = HashMap::<String, Vec<&Edge>>::new();

    for edge in edges {
        let from = &edge.1;
        if !connections.contains_key(from) {
            connections.insert(from.clone(), vec![edge]);
        } else {
            let mut _list = connections.get_mut(from).unwrap();
            _list.push(edge)
        }
    }
    connections
}

fn construct_assemblies(
    connections: &HashMap<String, Vec<&Edge>>,
    operations: &Vec<LCAbygOperation>,
    project_completion_year: &u16,
    heated_area: &f64,
    elements: &Vec<LCAbygElement>,
    categories: &HashMap<String, String>,
    constructions: &Vec<LCAbygConstruction>,
    products: &Vec<LCAbygProduct>,
    stages: &Vec<LCAbygStage>,
    result_data: Option<&LCAbygResults>,
) -> Vec<Assembly> {
    let mut assemblies = vec![];

    for operation in operations {
        let operation_edges = connections.get(&operation.id).unwrap();
        assemblies.push(Assembly::from_lcabyg_operation((
            operation,
            operation_edges,
            project_completion_year,
            heated_area,
            result_data,
        )));
    }

    for element in elements {
        let element_edges = connections.get(&element.id).unwrap();
        let category_id = categories.get(&element.id).unwrap();
        let mut _constructions = vec![];
        for edge in &*element_edges {
            let construction_id = &edge.2;
            match constructions
                .iter()
                .find(|_con| _con.id == *construction_id)
            {
                Some(_con) => _constructions.push(_con.to_owned()),
                None => {}
            }
        }
        assemblies.push(Assembly::from_lcabyg((
            connections,
            element,
            category_id,
            &_constructions,
            products,
            stages,
            result_data,
        )));
    }

    assemblies
}

fn construct_products(
    connections: &HashMap<String, Vec<&Edge>>,
    constructions: &Vec<&LCAbygConstruction>,
    products: &Vec<LCAbygProduct>,
    stages: &Vec<LCAbygStage>,
) -> Vec<Product> {
    let mut lcax_products = vec![];

    for construction in constructions {
        let construction_edges = match connections.get(&construction.id) {
            Some(edges) => edges,
            None => continue,
        };
        let mut _products = vec![];
        for edge in &*construction_edges {
            let product_id = &edge.2;
            match products.iter().find(|_product| _product.id == *product_id) {
                Some(_product) => _products.push(_product),
                None => {}
            };
        }
        lcax_products.push(Product::from_lcabyg((
            &connections,
            &construction,
            &products,
            &stages,
        )))
    }

    lcax_products
}

fn construct_operation_products(
    operation: &LCAbygOperation,
    project_completion_year: &u16,
    edges: &Vec<&Edge>,
    result_data: Option<&LCAbygResults>,
) -> Vec<Product> {
    let mut lcax_products = vec![];

    for edge in edges {
        match edge {
            (EdgeType::ElectricitySource(source), _, to_id) => {
                let results = match result_data {
                    Some(_data) => Some(Impacts::from_lcabyg((
                        &get_operation_result_id(&operation.id, "Electricity", _data),
                        _data,
                    ))),
                    None => None,
                };
                lcax_products.push(Product {
                    id: source.clone(),
                    name: "Electricity".to_string(),
                    description: Some("Impact data should be linearly interpolated".to_string()),
                    reference_service_life: 50,
                    impact_data: construct_impact_data(project_completion_year, to_id)
                        .iter()
                        .map(|impact| {
                            ImpactData::GenericData(GenericDataReference::GenericData(
                                impact.to_owned(),
                            ))
                        })
                        .collect(),
                    quantity: operation.electricity_usage,
                    unit: Unit::KWH,
                    transport: None,
                    results,
                    meta_data: None,
                })
            }
            (EdgeType::HeatingSource(source), _, to_id) => {
                let results = match result_data {
                    Some(_data) => Some(Impacts::from_lcabyg((
                        &get_operation_result_id(&operation.id, "Heating", _data),
                        _data,
                    ))),
                    None => None,
                };

                lcax_products.push(Product {
                    id: source.clone(),
                    name: "Heating".to_string(),
                    description: Some("Impact data should be linearly interpolated".to_string()),
                    reference_service_life: 50,
                    impact_data: construct_impact_data(project_completion_year, to_id)
                        .iter()
                        .map(|impact| {
                            ImpactData::GenericData(GenericDataReference::GenericData(
                                impact.to_owned(),
                            ))
                        })
                        .collect(),
                    quantity: operation.heat_usage,
                    unit: Unit::KWH,
                    transport: None,
                    results,
                    meta_data: None,
                })
            }
            _ => {}
        }
    }

    lcax_products
}

fn construct_impact_data(project_completion_year: &u16, energy_type_id: &str) -> Vec<GenericData> {
    match energy_type_id {
        "e967c8e7-e73d-47f3-8cba-19569ad76b4d" => {
            get_energy_data(project_completion_year, get_electricity_data())
        }
        "84ddd0e5-85a1-48cb-ab90-aa19b3359458" => {
            get_energy_data(project_completion_year, get_electricity_data())
        }
        "6cdeb050-90e5-46b3-89ad-bfcc8e246b47" => {
            get_energy_data(project_completion_year, get_district_heating_data())
        }
        "4e1a9c79-f9e6-4e93-84fd-c39d7f1d1231" => {
            get_energy_data(project_completion_year, get_lng_data())
        }
        _ => panic!("Could not find energy type id: {}", energy_type_id),
    }
}

fn construct_epds(
    connections: &HashMap<String, Vec<&Edge>>,
    products: &Vec<LCAbygProduct>,
    stages: &Vec<LCAbygStage>,
) -> Vec<EPD> {
    let mut epds = vec![];

    for product in products {
        let product_edges = match connections.get(&product.id) {
            Some(edges) => edges,
            None => continue,
        };
        let mut _stages = vec![];
        for edge in &*product_edges {
            let stage_id = &edge.2;
            match stages.iter().find(|s| s.id == *stage_id) {
                Some(_stage) => _stages.push(_stage),
                None => {}
            }
        }
        if !_stages.is_empty() {
            epds.push(EPD::from_lcabyg((&product, &_stages)))
        }
    }

    epds
}

trait FromLCAByg<T> {
    fn from_lcabyg(_: T) -> Self;
}

trait FromLCABygOperation<T> {
    fn from_lcabyg_operation(_: T) -> Self;
}

impl
    FromLCAByg<(
        HashMap<String, Vec<&Edge>>,
        &LCAbygProject,
        &LCAbygBuilding,
        &Vec<LCAbygOperation>,
        &Vec<LCAbygElement>,
        &HashMap<String, String>,
        &Vec<LCAbygConstruction>,
        &Vec<LCAbygProduct>,
        &Vec<LCAbygStage>,
        Option<&LCAbygResults>,
    )> for Project
{
    fn from_lcabyg(
        (
            connections,
            project,
            building,
            operations,
            elements,
            categories,
            constructions,
            products,
            stages,
            result_data,
        ): (
            HashMap<String, Vec<&Edge>>,
            &LCAbygProject,
            &LCAbygBuilding,
            &Vec<LCAbygOperation>,
            &Vec<LCAbygElement>,
            &HashMap<String, String>,
            &Vec<LCAbygConstruction>,
            &Vec<LCAbygProduct>,
            &Vec<LCAbygStage>,
            Option<&LCAbygResults>,
        ),
    ) -> Self {
        let results = match result_data {
            Some(_data) => Some(Impacts::from_lcabyg((&building.id, _data))),
            None => None,
        };
        let mut project = Self {
            id: project.id.to_string(),
            name: project.name.danish.clone().unwrap(),
            description: Some(building.description.danish.clone().unwrap()),
            comment: None,
            location: Location {
                country: Country::DNK,
                city: Some(String::from("")),
                address: Some(project.address.to_string()),
            },
            impact_categories: vec![],
            assemblies: construct_assemblies(
                &connections,
                operations,
                &building.initial_year,
                &building.heated_floor_area,
                elements,
                categories,
                constructions,
                products,
                stages,
                result_data,
            )
            .iter()
            .map(|assembly| AssemblyReference::Assembly(assembly.to_owned()))
            .collect(),
            results,
            project_info: Some(ProjectInfo::BuildingInfo {
                0: BuildingInfo {
                    building_type: BuildingType::from_lcabyg(
                        building
                            .project_type
                            .clone()
                            .unwrap_or("".to_string())
                            .as_str(),
                    ),
                    building_typology: vec![BuildingTypology::from(&building.building_type)],
                    certifications: None,
                    building_mass: None,
                    building_height: None,
                    gross_floor_area: Some(AreaType {
                        value: building.gross_area,
                        unit: Unit::M2,
                        definition: "".to_string(),
                    }),
                    heated_floor_area: Some(AreaType {
                        value: building.heated_floor_area,
                        unit: Unit::M2,
                        definition: "".to_string(),
                    }),
                    building_footprint: None,
                    floors_above_ground: building.storeys_above_ground,
                    floors_below_ground: Some(building.storeys_below_ground),
                    roof_type: RoofType::OTHER,
                    frame_type: Some("".to_string()),
                    building_completion_year: Some(building.initial_year as u16),
                    building_permit_year: None,
                    energy_demand_heating: Some(0.0),
                    energy_supply_heating: Some(0.0),
                    energy_demand_electricity: Some(0.0),
                    energy_supply_electricity: Some(0.0),
                    exported_electricity: Some(0.0),
                    general_energy_class: GeneralEnergyClass::from(&building.energy_class),
                    local_energy_class: None,
                    building_model_scope: None,
                    building_users: Some(building.person_count as u32),
                },
            }),
            life_cycle_modules: vec![],
            owner: Some(project.owner.to_string()),
            format_version: get_version(),
            lcia_method: None,
            classification_systems: Some(vec![String::from("LCAByg")]),
            software_info: SoftwareInfo {
                goal_and_scope_definition: None,
                lca_software_version: None,
                lca_software: String::from("LCAByg"),
                calculation_type: Some(project.building_regulation_version.to_string()),
            },
            reference_study_period: Some(building.calculation_timespan as u8),
            project_phase: Default::default(),
            meta_data: Some(HashMap::from([(
                "convertedAt".to_string(),
                AnyValue::String(format!("{}", Utc::now().naive_utc().format("%Y-%m-%d"))),
            )])),
        };
        project.resolve_impact_categories();
        project.resolve_life_cycle_stages();
        project
    }
}

impl
    FromLCAByg<(
        &HashMap<String, Vec<&Edge>>,
        &LCAbygElement,
        &str,
        &Vec<&LCAbygConstruction>,
        &Vec<LCAbygProduct>,
        &Vec<LCAbygStage>,
        Option<&LCAbygResults>,
    )> for Assembly
{
    fn from_lcabyg(
        (connections, element, category_id, constructions, products, stages, result_data): (
            &HashMap<String, Vec<&Edge>>,
            &LCAbygElement,
            &str,
            &Vec<&LCAbygConstruction>,
            &Vec<LCAbygProduct>,
            &Vec<LCAbygStage>,
            Option<&LCAbygResults>,
        ),
    ) -> Self {
        let mut quantity = 0.0;
        for edge in connections.get(&element.id).unwrap() {
            match &edge.0 {
                EdgeType::ElementToConstruction(_edge) => quantity = _edge.amount,
                _ => {}
            }
        }

        let results = match result_data {
            Some(_data) => Some(Impacts::from_lcabyg((
                &get_result_id(&element.id, _data),
                _data,
            ))),
            None => None,
        };

        Self {
            id: element.id.clone().to_string(),
            name: element.name.get(),
            description: Some("".to_string()),
            comment: Some(element.comment.get()),
            quantity,
            unit: Unit::M,
            classification: Some(vec![Classification::from_lcabyg(category_id)]),
            products: construct_products(connections, constructions, products, stages)
                .iter()
                .map(|product| ProductReference::Product(product.to_owned()))
                .collect(),
            results,
            meta_data: None,
        }
    }
}

impl
    FromLCABygOperation<(
        &LCAbygOperation,
        &Vec<&Edge>,
        &u16,
        &f64,
        Option<&LCAbygResults>,
    )> for Assembly
{
    fn from_lcabyg_operation(
        (operation, operation_edges, project_completion_year, heated_floor_area, result_data): (
            &LCAbygOperation,
            &Vec<&Edge>,
            &u16,
            &f64,
            Option<&LCAbygResults>,
        ),
    ) -> Self {
        let results = match result_data {
            Some(_data) => Some(Impacts::from_lcabyg((&operation.id, _data))),
            None => None,
        };
        Self {
            id: operation.id.clone(),
            name: "Drift".to_string(),
            description: None,
            comment: None,
            unit: Unit::M2,
            quantity: heated_floor_area.clone(),
            classification: Some(vec![Classification::from_lcabyg("drift")]),
            products: construct_operation_products(
                operation,
                project_completion_year,
                operation_edges,
                result_data,
            )
            .iter()
            .map(|product| ProductReference::Product(product.to_owned()))
            .collect(),
            results,
            meta_data: None,
        }
    }
}

impl FromLCAByg<&str> for Classification {
    fn from_lcabyg(category_id: &str) -> Self {
        Self {
            system: "LCAByg".to_string(),
            code: category_id.to_string(),
            name: categories::category_id_to_names(category_id).to_string(),
        }
    }
}

impl
    FromLCAByg<(
        &HashMap<String, Vec<&Edge>>,
        &LCAbygConstruction,
        &Vec<LCAbygProduct>,
        &Vec<LCAbygStage>,
    )> for Product
{
    fn from_lcabyg(
        (connections, construction, products, stages): (
            &HashMap<String, Vec<&Edge>>,
            &LCAbygConstruction,
            &Vec<LCAbygProduct>,
            &Vec<LCAbygStage>,
        ),
    ) -> Self {
        let mut quantity = 0.0;
        let mut unit = Unit::UNKNOWN;
        let mut reference_service_life = 0;

        for edge in connections.get(&construction.id).unwrap() {
            match &edge.0 {
                EdgeType::ConstructionToProduct(_edge) => {
                    quantity = _edge.amount;
                    reference_service_life = _edge.lifespan;
                    unit = Unit::from(&_edge.unit)
                }
                _ => {}
            }
        }
        Self {
            id: construction.id.clone().to_string(),
            name: construction.name.get(),
            description: Some("".to_string()),
            reference_service_life,
            quantity,
            unit,
            results: None,
            meta_data: None,
            impact_data: construct_epds(connections, products, stages)
                .iter()
                .map(|epd| ImpactData::EPD(EPDReference::EPD(epd.to_owned())))
                .collect(),
            transport: None,
        }
    }
}

impl FromLCAByg<(&LCAbygProduct, &Vec<&LCAbygStage>)> for EPD {
    fn from_lcabyg((product, stages): (&LCAbygProduct, &Vec<&LCAbygStage>)) -> Self {
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
                match impact_category.get(&LifeCycleModule::try_from(stage.stage.as_str()).unwrap())
                {
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
        Self {
            id: product.id.to_string(),
            name: product.name.get(),
            declared_unit: Unit::from(&node.stage_unit),
            version: node.external_version.clone(),
            published_date: Default::default(),
            valid_until: NaiveDate::parse_from_str(
                &node.valid_to.clone().unwrap_or("2020-01-01".to_string()),
                "%Y-%m-%d",
            )
            .unwrap(),
            comment: Some(node.comment.get()),
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
        }
    }
}

impl FromLCAByg<&str> for BuildingType {
    fn from_lcabyg(_type: &str) -> Self {
        match _type.to_ascii_lowercase().as_str() {
            "new" => Self::NEW_CONSTRUCTION_WORKS,
            "" => Self::OTHER,
            _ => panic!("Unknown building type: {}", _type),
        }
    }
}

impl FromLCAByg<(&str, &LCAbygResults)> for Impacts {
    fn from_lcabyg((object_id, results): (&str, &LCAbygResults)) -> Self {
        match results.results.get(object_id) {
            Some(object_result) => {
                let mut result = HashMap::new();

                for (stage_key, field) in object_result.fields() {
                    match field.get::<Option<YearResult>>().unwrap() {
                        Some(year_result) => {
                            let end_of_time_result = &year_result.end_of_time;
                            for (category_key, result_field) in end_of_time_result.fields() {
                                let value = result_field.get::<f64>().unwrap();
                                result
                                    .entry(ImpactCategoryKey::from_lcabyg(category_key))
                                    .or_insert_with(HashMap::new)
                                    .insert(
                                        LifeCycleModule::try_from(stage_key).unwrap(),
                                        Some(*value),
                                    );
                            }
                        }
                        _ => {}
                    }
                }
                result
            }
            None => HashMap::new(),
        }
    }
}

impl FromLCAByg<&str> for ImpactCategoryKey {
    fn from_lcabyg(category: &str) -> Self {
        match category.to_ascii_lowercase().as_str() {
            "ap" => Self::AP,
            "adpf" => Self::ADPF,
            "adpe" => Self::ADPE,
            "odp" => Self::ODP,
            "gwp" => Self::GWP,
            "gwp_fossil" => Self::GWP_FOS,
            "gwp_iobc" => Self::GWP_BIO,
            "penr" => Self::PENRT,
            "per" => Self::PERT,
            "ep" => Self::EP,
            "pocp" => Self::POCP,
            _ => panic!("Unknown impact category: {}", category),
        }
    }
}

fn get_result_id(object_id: &str, results: &LCAbygResults) -> String {
    for result in &results.model {
        match result {
            Model::InstanceModel(instance) if instance.model_id == object_id => {
                return instance.id.to_string()
            }
            _ => continue,
        }
    }
    "".to_string()
}

fn get_operation_result_id(object_id: &str, _type: &str, results: &LCAbygResults) -> String {
    for result in &results.model {
        match result {
            Model::OperationUtilityInstance(instance) if instance.parent == object_id => {
                return instance.id.to_string()
            }
            _ => continue,
        }
    }
    "".to_string()
}
