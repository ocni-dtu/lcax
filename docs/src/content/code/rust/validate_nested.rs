use std::fs;
use std::path::Path;

fn main() {
    let project = serde_json::from_str::<Project>(
        &fs::read_to_string("project.json").unwrap()
    ).unwrap();

    let schemas = vec![
        ValidationSchema {
            level: Level::Project,
            field: "location.country".to_string(),
            rule: ValidationRule {
                range: None,
                includes: None,
                required: None,
                equal: Some("dnk".to_string()),
                greater: None,
                less: None,
                one_of: None,
            }
        },
        ValidationSchema {
            level: Level::Project,
            field: "projectInfo?.grossFloorArea?.value".to_string(),
            rule: ValidationRule {
                range: None,
                includes: None,
                required: None,
                equal: None,
                greater: Some(50.0),
                less: None,
                one_of: None,
            }
        }
    ];

    let result = validate(&project, &schemas); // result = []
}