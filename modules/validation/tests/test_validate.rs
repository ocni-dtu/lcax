use std::fs;
use std::path::Path;
use serde::Serialize;
use valitron::available::{Gt, Range, Required, StartWith};
use valitron::{RuleExt, Validator};
use lcax_models::project::{Project as LCAxProject};
use lcax_validation::model::ValidationSchema;
use lcax_validation::validate::validate;

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
    let project = serde_json::from_str::<LCAxProject>(&fs::read_to_string(project_path).unwrap()).unwrap();
    let rules_path = root_dir.join("tests/datafixtures/validation_rules.yaml");
    let validation_rules = serde_yml::from_str::<ValidationSchema>(&fs::read_to_string(rules_path).unwrap()).unwrap();

    let result = validate(&project, &validation_rules);
    println!("{:?}", result.unwrap_err());
    Ok(())
}

#[test]
fn test_validate_pure() -> Result<(), String> {
    #[derive(Serialize, Debug)]
    struct Person {
        introduce: String,
        age: Option<u8>,
        weight: f32,
    }

    let validator = Validator::new()
        .rule("introduce", Required)
        .rule("age", Required.and(Gt(10_u8)));
        // .message([
        //     ("introduce.required", "introduce is required"),
        //     (
        //         "introduce.start_with",
        //         "introduce should be starts with `I am`",
        //     ),
        //     ("age.required", "age is required"),
        //     ("age.gt", "age should be greater than 10"),
        // ]);

    let person = Person {
        introduce: "".to_string(),
        age: Some(18),
        weight: 20.0,
    };

    let res = validator.validate(person).unwrap_err();
    println!("{:?}", res);
    assert_eq!(res.len(), 1);
    Ok(())
}