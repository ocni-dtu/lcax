use std::fs;
use std::path::Path;

fn main() {
    let project = serde_json::from_str::<Project>(
        &fs::read_to_string("project.json").unwrap()
    ).unwrap();

    let schemas = serde_yml::from_str::<Vec<ValidationSchema>>(
        &fs::read_to_string("validation_schema").unwrap()
    ).unwrap();

    let result = validate(&project, &schemas); // result = []
}