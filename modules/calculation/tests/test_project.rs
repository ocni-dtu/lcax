use std::fs;
use std::path::Path;

use lcax_calculation::calculate::calculate_project;
use lcax_models::project::Project;

#[test]
fn test_calculate_project() -> Result<(), String> {
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    let file_path = root_dir.join("tests/datafixtures/project.json");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut project = serde_json::from_str::<Project>(&contents).unwrap();

    calculate_project(&mut project, None)?;
    assert!(project.results.is_some());
    Ok(())
}
