use std::fs;
use std::path::Path;
use lcax_models::project::{Project as LCAxProject};
use lcax_validation::validate::validate;


#[test]
fn test_validate() -> Result<(), String> {
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let project_path = root_dir.join("tests/datafixtures/project.json");
    let project = serde_json::from_str::<LCAxProject>(&fs::read_to_string(project_path).unwrap()).unwrap();
    let rules_path = root_dir.join("tests/datafixtures/validation_rules.yaml");
    let validation_rules = serde_yml::from_str::<LCAxProject>(&fs::read_to_string(rules_path).unwrap()).unwrap();

    match validate(project, validation_rules) {
        Ok(_) => Ok(()),
        Err(msg) => Err(msg),
    }
}