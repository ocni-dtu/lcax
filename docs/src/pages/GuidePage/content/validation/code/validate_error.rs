use std::fs;
use std::path::Path;

fn main() {
    let project = serde_json::from_str::<Project>(
        &fs::read_to_string("project.json").unwrap()
    ).unwrap();

    let schemas = vec![
        ValidationSchema {
            level: Level::Project,
            field: "name".to_string(),
            rule: ValidationRule {
                range: None,
                includes: None,
                required: None,
                equal: Some("Te eksempel".to_string()),
                greater: None,
                less: None,
                one_of: None,
            }
        }
    ];

    let result = validate(&project, &schemas);
    // result = [{
    //  field: "name",
    //  message: "Field is not equal to: Te eksempel"
    // }]
}