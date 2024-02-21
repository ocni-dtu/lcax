use epdx::epd::{Unit, EPD};
use pkg_version::*;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Error;

use crate::lcabyg::edges::EdgeType;
use crate::lcabyg::nodes::Node;
use crate::lcabyg::{categories, edges, nodes};
use crate::project as lcax_project;
use crate::project::{
    Assembly, BuildingInfo, BuildingType, Classification, EPDProduct, EPDSource, LCAxProject,
    Location, SoftwareInfo,
};

type Edge = (EdgeType, String, String);

#[derive(Deserialize, Serialize)]
enum NodesAndEdges {
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
pub fn parse_lcabyg(
    project_data: String,
    result_data: Option<String>,
) -> Result<LCAxProject, Error> {
    match serde_json::from_str(&project_data) {
        Ok(lcabyg_project) => Ok(lcax_from_lcabyg(lcabyg_project, result_data)),
        Err(err) => Err(err),
    }
}

fn lcax_from_lcabyg(
    lcabyg_project: Vec<NodesAndEdges>,
    result_data: Option<String>,
) -> LCAxProject {
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
    project
}

fn add_result_from_lcabyg(lcax_project: &LCAxProject, result_data: String) -> &LCAxProject {
    lcax_project
}

fn add_project_data(project: &mut LCAxProject, node: &nodes::Project) {
    project.id = node.id.to_string();
    project.name = node.name.danish.clone().unwrap();
    project.description = String::from("");
    project.location = Location {
        country: String::from("Denmark"),
        city: String::from(""),
        address: node.address.to_string(),
    };
    project.owner = node.owner.to_string();
    project.format_version = format!(
        "{}.{}.{}",
        pkg_version_major!(),
        pkg_version_minor!(),
        pkg_version_patch!()
    );
    project.classification_system = Some(String::from("LCAByg"));
    project.software_info = SoftwareInfo {
        goal_and_scope_definition: None,
        lca_software: String::from("LCAByg"),
        calculation_type: Some(node.building_regulation_version.to_string()),
    }
}

fn add_building_data(project: &mut LCAxProject, node: &nodes::Building) {
    project.reference_study_period = Some(node.calculation_timespan as u8);
    project.project_info = Some(lcax_project::ProjectInfo::BuildingInfo {
        0: BuildingInfo {
            building_type: BuildingType::NEW,
            building_typology: lcax_project::BuildingTypology::from(&node.building_type),
            certifications: "".to_string(),
            building_mass: "".to_string(),
            gross_floor_area: node.gross_area,
            gross_floor_area_definition: "".to_string(),
            heated_floor_area: node.heated_floor_area,
            heated_floor_area_definition: "".to_string(),
            floors_above_ground: node.storeys_above_ground,
            floors_below_ground: node.storeys_below_ground,
            frame_type: "".to_string(),
            building_completion_year: 0,
            energy_demand_heating: 0.0,
            energy_supply_heating: 0.0,
            energy_demand_electricity: 0.0,
            energy_supply_electricity: 0.0,
            exported_electricity: 0.0,
            energy_class: node.energy_class.to_string(),
            building_model_scope: None,
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
            (EdgeType::CategoryToElement(category_edge), category_id, element_id)
                if element_id == &node_id =>
            {
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
) -> EPDProduct {
    let mut product = EPDProduct {
        id: product_id.clone(),
        name: "".to_string(),
        description: "".to_string(),
        reference_service_life: edge.lifespan,
        epd_source: Default::default(),
        quantity: edge.amount,
        unit: Unit::from(&edge.unit),
        transport: None,
        results: None,
        meta_data: None,
    };

    let mut stages: Vec<epdx::lcabyg::Stage> = vec![];

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

    let epd_data = EPD::from(&stages);

    product.epd_source = EPDSource::EPD(epd_data);
    product
}

fn get_lcabyg_classification(category_id: &str) -> Vec<Classification> {
    vec![Classification {
        system: "LCAByg".to_string(),
        code: category_id.to_string(),
        name: categories::category_id_to_names(category_id).to_string(),
    }]
}
