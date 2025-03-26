use lcax_models::project::Project as LCAxProject;
use lcax_validation::model::ValidationSchema;
use lcax_validation::validate::validate;
use std::fs;
use std::path::Path;

use std::sync::Once;

static INIT: Once = Once::new();

fn init_logger() {
    INIT.call_once(|| {
        env_logger::builder().is_test(true).init();
    });
}

#[test]
fn test_validate() -> Result<(), String> {
    init_logger();

    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let project_path = root_dir.join("tests/datafixtures/project.json");
    let project =
        serde_json::from_str::<LCAxProject>(&fs::read_to_string(project_path).unwrap()).unwrap();
    let rules_path = root_dir.join("tests/datafixtures/validation_rules.yaml");
    let validation_rules =
        serde_yml::from_str::<Vec<ValidationSchema>>(&fs::read_to_string(rules_path).unwrap())
            .unwrap();

    match validate(&project, &validation_rules) {
        Ok(()) => Ok(()),
        Err(msg) => {
            println!("{:?}", msg);
            assert_eq!(msg.len(), 0);
            Err(msg.iter().map(|_msg| _msg.to_string()).collect())
        }
    }
}
