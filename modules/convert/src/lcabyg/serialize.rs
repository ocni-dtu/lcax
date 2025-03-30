use crate::lcabyg::edges::{EdgeType, ProductToStageEdge};
use crate::lcabyg::nodes::{Languages, Node, Product, Stage};
use crate::lcabyg::parse::{LCABygResult, NodesAndEdges};
use lcax_models::epd::EPD;

pub fn to_lcabyg(objects: &LCABygResult) -> serde_json::Result<String> {
    match objects {
        LCABygResult::EPDs(epds) => serialize_epds(epds),
        // LCABygResult::Project(project) => serialize_project(project),
        _ => Ok("".to_string()),
    }
}

// fn serialize_project(project: &Project) -> serde_json::Result<String> {
//     let mut nodes_and_edges: Vec<NodesAndEdges> = Vec::new();
//     for epd in epds {
//         nodes_and_edges.append(&mut serialize_epd(&epd))
//     }
//     serde_json::to_string(&nodes_and_edges)
// }

pub fn serialize_epds(epds: &Vec<EPD>) -> serde_json::Result<String> {
    let mut nodes_and_edges: Vec<NodesAndEdges> = Vec::new();
    for epd in epds {
        nodes_and_edges.append(&mut serialize_epd(&epd))
    }
    serde_json::to_string(&nodes_and_edges)
}

fn serialize_epd(epd: &EPD) -> Vec<NodesAndEdges> {
    let mut nodes_and_edges: Vec<NodesAndEdges> = Vec::new();

    let product = Product {
        id: epd.id.clone(),
        name: Languages {
            english: Some(epd.name.clone()),
            german: None,
            norwegian: None,
            danish: None,
        },
        source: "User".to_string(),
        comment: Languages {
            english: epd.comment.clone(),
            german: None,
            norwegian: None,
            danish: None,
        },
        uncertainty_factor: 1.0,
    };
    nodes_and_edges.push(NodesAndEdges::Node(Node::Product(product.clone())));

    for (impact_key, category) in epd.impacts.iter() {
        for (stage, value) in category.iter() {
            for node_or_edge in &mut nodes_and_edges {
                match node_or_edge {
                    NodesAndEdges::Node(ref mut node) => match node {
                        Node::Stage(stage) => {
                            // update stage
                            stage.update_indicator(
                                &impact_key.to_string(),
                                &value.unwrap_or_else(|| 0.0),
                            );
                            continue;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }

            let lcabyg_stage = Stage::new(
                epd,
                &stage.to_string(),
                &impact_key.to_string(),
                &value.unwrap_or_else(|| 0.0),
            );
            nodes_and_edges.push(NodesAndEdges::Node(Node::Stage(lcabyg_stage.clone())));
            nodes_and_edges.push(NodesAndEdges::Edge((
                EdgeType::ProductToStage(ProductToStageEdge::new()),
                product.id.clone(),
                lcabyg_stage.id,
            )));
        }
    }
    nodes_and_edges
}
