use crate::lcabyg::edges::EdgeType;
use crate::lcabyg::nodes::{epd_from_lcabyg_stages, Node};
use crate::lcabyg::results::Model::InstanceModel;
use crate::lcabyg::results::{LCAbygResults, YearResult};
use crate::lcabyg::{categories, edges, nodes};
use field_access::FieldAccess;
use lcax_core::country::Country;
use lcax_core::utils::get_version;
use lcax_models::assembly::{Assembly, Classification};
use lcax_models::life_cycle_base::{ImpactCategoryKey, LifeCycleStage};
use lcax_models::product::{ImpactDataSource, Product as LCAxProduct};
use lcax_models::project::{
    AreaType, BuildingInfo, BuildingType, BuildingTypology, GeneralEnergyClass, Location,
    Project as LCAxProject, ProjectInfo, RoofType, SoftwareInfo,
};
use lcax_models::shared::Unit;
use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::collections::HashMap;

type Edge = (EdgeType, String, String);

#[derive(Deserialize, Serialize)]
pub enum NodesAndEdges {
    Node(Node),
    Edge(Edge),
}

/// Parse an LCAByg project exported as json files.
///
/// # Arguments
///
/// * `project_data`: JSON formatted string containing the LCAbyg project data
/// * `result_data`: Optional JSON formatted string containing the result data from the LCAByg project
///
/// returns: LCAxProject
pub fn parse_lcabyg(project_data: &str, result_data: Option<&str>) -> Result<LCAxProject, Error> {
    match serde_json::from_str(project_data) {
        Ok(lcabyg_project) => match lcax_from_lcabyg(lcabyg_project, result_data) {
            Ok(lcabyg_project) => Ok(lcabyg_project),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}

fn lcax_from_lcabyg(
    lcabyg_project: Vec<NodesAndEdges>,
    result_data: Option<&str>,
) -> Result<LCAxProject, Error> {
    let mut project: LCAxProject = Default::default();
    let mut nodes: Vec<Node> = vec![];
    let mut edges: Vec<Edge> = vec![];
    for element in lcabyg_project {
        match element {
            NodesAndEdges::Node(node) => nodes.append(&mut vec![node]),
            NodesAndEdges::Edge(edge) => edges.append(&mut vec![edge]),
        }
    }

    for _node in &nodes {
        match _node {
            Node::Project(node) => add_project_data(&mut project, node),
            Node::Building(node) => add_building_data(&mut project, node),
            Node::Element(node) => add_element_data(&mut project, node, &nodes, &edges),
            // Node::EmbodiedRoot(node) => (),
            // Node::Operation(node) => (),
            // Node::ProductTransportRoot(node) => (),
            // Node::ConstructionProcess(node) => (),
            // Node::ElementModel(node) => (),
            // Node::ConstructionInstallation(node) => (),
            // Node::FuelConsumption(node) => (),
            // Node::DGNBOperationReference(node) => (),
            _ => {}
        }
    }
    if result_data.is_some() {
        match serde_json::from_str(result_data.unwrap()) {
            Ok(_result_data) => {
                let project_id = project.id.clone();
                Ok(add_result_from_lcabyg(
                    &mut project,
                    &get_building_id(edges, &project_id),
                    &_result_data,
                ))
            }
            Err(err) => Err(err),
        }
        .expect("Failed to parse LCAbyg results");
    }
    Ok(project)
}

fn get_building_id(edges: Vec<Edge>, project_id: &str) -> String {
    for edge in edges {
        match edge {
            (EdgeType::MainBuilding(_), _project_id, building_id) if _project_id == project_id => {
                return building_id
            }
            _ => continue,
        }
    }
    "".to_string()
}

fn add_result_from_lcabyg(
    lcax_project: &mut LCAxProject,
    building_id: &str,
    results: &LCAbygResults,
) {
    lcax_project.results = collect_lcabyg_object_results(
        building_id,
        results,
        &lcax_project.impact_categories,
        &lcax_project.life_cycle_stages,
    );
    for (assembly_id, assembly) in &mut lcax_project.assemblies {
        assembly.results = collect_lcabyg_object_results(
            &get_result_id(assembly_id, results),
            results,
            &lcax_project.impact_categories,
            &lcax_project.life_cycle_stages,
        )
    }
}

fn get_result_id(object_id: &str, results: &LCAbygResults) -> String {
    for result in &results.model {
        match result {
            InstanceModel(instance) if instance.model_id == object_id => {
                return instance.id.to_string()
            }
            _ => continue,
        }
    }
    return "".to_string();
}

fn collect_lcabyg_object_results(
    object_id: &str,
    results: &LCAbygResults,
    impact_categories: &Vec<ImpactCategoryKey>,
    life_cycle_stages: &Vec<LifeCycleStage>,
) -> Option<HashMap<ImpactCategoryKey, HashMap<LifeCycleStage, Option<f64>>>> {
    match results.results.get(object_id) {
        Some(object_result) => {
            let mut result = HashMap::new();
            for category in impact_categories {
                let mut stages = HashMap::new();
                for stage in life_cycle_stages {
                    let impact_result =
                        match &object_result.field(stage.to_string().to_lowercase().as_str()) {
                            Some(field) => match field.get::<YearResult>() {
                                Some(year) => Some(
                                    year.end_of_time
                                        .field(category.to_string().to_lowercase().as_str())
                                        .unwrap()
                                        .as_f64()
                                        .unwrap(),
                                ),
                                None => None,
                            },
                            None => None,
                        };
                    stages.insert(stage.clone(), impact_result.clone());
                }
                result.insert(category.clone(), stages.clone());
            }

            Some(result)
        }
        None => None,
    }
}

fn add_project_data(project: &mut LCAxProject, node: &nodes::Project) {
    project.id = node.id.to_string();
    project.name = node.name.danish.clone().unwrap();
    project.description = Some(String::from(""));
    project.location = Location {
        country: Country::DNK,
        city: Some(String::from("")),
        address: Some(node.address.to_string()),
    };
    project.impact_categories = vec![
        ImpactCategoryKey::AP,
        ImpactCategoryKey::ADPE,
        ImpactCategoryKey::ADPF,
        ImpactCategoryKey::EP,
        ImpactCategoryKey::POCP,
        ImpactCategoryKey::ODP,
        ImpactCategoryKey::GWP,
        ImpactCategoryKey::PENRE,
        ImpactCategoryKey::PERE,
    ];
    project.life_cycle_stages = vec![
        LifeCycleStage::A1A3,
        LifeCycleStage::C3,
        LifeCycleStage::C4,
        LifeCycleStage::D,
    ];
    project.owner = Some(node.owner.to_string());
    project.format_version = get_version();
    project.classification_system = Some(String::from("LCAByg"));
    project.software_info = SoftwareInfo {
        goal_and_scope_definition: None,
        lca_software: String::from("LCAByg"),
        calculation_type: Some(node.building_regulation_version.to_string()),
    }
}

fn add_building_data(project: &mut LCAxProject, node: &nodes::Building) {
    project.reference_study_period = Some(node.calculation_timespan as u8);
    project.project_info = Some(ProjectInfo::BuildingInfo {
        0: BuildingInfo {
            building_type: BuildingType::NEW,
            building_typology: BuildingTypology::from(&node.building_type),
            certifications: None,
            building_mass: None,
            building_height: None,
            gross_floor_area: Some(AreaType {
                value: node.gross_area,
                unit: Unit::M2,
                definition: "".to_string(),
            }),
            heated_floor_area: Some(AreaType {
                value: node.heated_floor_area,
                unit: Unit::M2,
                definition: "".to_string(),
            }),
            building_footprint: None,
            floors_above_ground: node.storeys_above_ground,
            floors_below_ground: Some(node.storeys_below_ground),
            roof_type: RoofType::OTHER,
            frame_type: Some("".to_string()),
            building_completion_year: Some(0),
            building_permit_year: None,
            energy_demand_heating: Some(0.0),
            energy_supply_heating: Some(0.0),
            energy_demand_electricity: Some(0.0),
            energy_supply_electricity: Some(0.0),
            exported_electricity: Some(0.0),
            general_energy_class: GeneralEnergyClass::from(&node.energy_class),
            local_energy_class: None,
            building_model_scope: None,
            building_users: None,
        },
    })
}

fn add_element_data(
    project: &mut LCAxProject,
    node: &nodes::Element,
    nodes: &Vec<Node>,
    edges: &Vec<Edge>,
) {
    let node_id = node.id.clone();
    let mut assembly = Assembly {
        id: node_id.to_string(),
        name: node.name.english.clone().unwrap(),
        description: "".to_string(),
        comment: node.comment.english.clone(),
        quantity: 0.0,
        unit: Unit::M,
        category: None,
        classification: None,
        products: Default::default(),
        results: None,
        meta_data: None,
    };
    let mut found = (false, false);

    for edge in edges {
        match edge {
            (EdgeType::ElementToConstruction(element_edge), parent_id, child_id)
                if parent_id == &node_id =>
            {
                add_element_to_construction_data(
                    &mut assembly,
                    child_id,
                    element_edge,
                    edges,
                    nodes,
                );
                if found == (false, true) {
                    break;
                } else {
                    found = (true, false);
                }
            }
            (EdgeType::CategoryToElement(_), category_id, element_id) if element_id == &node_id => {
                assembly.classification = Some(get_lcabyg_classification(category_id));
                if found == (true, false) {
                    break;
                } else {
                    found = (false, true);
                }
            }
            _ => continue,
        }
    }
    project
        .assemblies
        .insert(assembly.id.clone(), assembly.clone());
}

fn add_element_to_construction_data(
    assembly: &mut Assembly,
    construction_id: &String,
    edge: &edges::ElementToConstructionEdge,
    edges: &Vec<Edge>,
    nodes: &Vec<Node>,
) {
    assembly.quantity = edge.amount;
    for node in nodes {
        match node {
            Node::Construction(construction_node) if &construction_node.id == construction_id => {
                add_construction_data(assembly, construction_node, edges, nodes);
                break;
            }
            _ => continue,
        }
    }
}

fn add_construction_data(
    assembly: &mut Assembly,
    node: &nodes::Construction,
    edges: &Vec<Edge>,
    nodes: &Vec<Node>,
) {
    assembly.unit = Unit::from(&node.unit);

    for _edge in edges {
        match _edge {
            (EdgeType::ConstructionToProduct(construction_edge), parent_id, child_id)
                if parent_id == &node.id =>
            {
                let product = add_construction_to_product_data(child_id, &construction_edge, nodes);
                assembly
                    .products
                    .insert(product.id.clone(), product.clone());
                break;
            }
            _ => continue,
        }
    }
}

fn add_construction_to_product_data(
    product_id: &String,
    edge: &edges::ConstructionToProductEdge,
    nodes: &Vec<Node>,
) -> LCAxProduct {
    let mut product = LCAxProduct {
        id: product_id.clone(),
        name: "".to_string(),
        description: "".to_string(),
        reference_service_life: edge.lifespan,
        impact_data: Default::default(),
        quantity: edge.amount,
        unit: Unit::from(&edge.unit),
        transport: None,
        results: None,
        meta_data: None,
    };

    let mut stages: Vec<nodes::Stage> = vec![];

    for node in nodes {
        match node {
            Node::Product(product_node) => {
                product.name = product_node.name.english.clone().unwrap();
                product.description = product_node.comment.english.clone().unwrap();
            }
            Node::Stage(stage_node) => stages.append(&mut vec![stage_node.clone()]),
            _ => continue,
        }
    }

    let epd_data = epd_from_lcabyg_stages(&stages);

    product.impact_data = ImpactDataSource::EPD(epd_data);
    product
}

fn get_lcabyg_classification(category_id: &str) -> Vec<Classification> {
    vec![Classification {
        system: "LCAByg".to_string(),
        code: category_id.to_string(),
        name: categories::category_id_to_names(category_id).to_string(),
    }]
}
