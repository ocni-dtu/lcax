use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Error;
use crate::lcabyg::edges::EdgeType;

use crate::project::LCAxProject;

#[derive(Deserialize, Serialize)]
enum NodesAndEdges {
    Node(crate::lcabyg::nodes::Node),
    Edge((EdgeType, String, String))
}


/// Parse an LCAByg project exported as json files.
///
/// # Arguments
///
/// * `project_data`: JSON formatted string containing the LCAbyg project data
/// * `result_data`: Optional JSON formatted string containing the result data from the LCAByg project
///
/// returns: LCAxProject
pub fn parse_lcabyg(project_data: String, result_data: Option<String>) -> Result<LCAxProject, Error> {
    match serde_json::from_str(&project_data) {
        Ok(lcabyg_project) => {
            let lcax_project = lcax_from_lcabyg(lcabyg_project, result_data);
            Ok(lcax_project)
        }
        Err(err) => Err(err)
    }
}


fn lcax_from_lcabyg(lcabyg_project: Vec<NodesAndEdges>, result_data: Option<String>) -> LCAxProject {

    let project: LCAxProject = Default::default();
    project
}

fn add_result_from_lcabyg(lcax_project: &LCAxProject, result_data: String) -> &LCAxProject {
    lcax_project
}