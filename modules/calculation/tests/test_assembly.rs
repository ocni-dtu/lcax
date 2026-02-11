use std::fs;
use std::path::Path;

use lcax_calculation::calculate::calculate_assembly;
use lcax_calculation::models::CalculationOptions;
use lcax_models::assembly::AssemblyReference;
use lcax_models::project::Project;

#[test]
fn test_calculate_assembly() -> Result<(), String> {
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    let file_path = root_dir.join("tests/datafixtures/project.json");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut project = serde_json::from_str::<Project>(&contents).unwrap();

    let assembly = match &mut project.assemblies[0] {
        AssemblyReference::Assembly(actual) => actual,
        AssemblyReference::Reference(_) => panic!("Expected actual assembly"),
    };
    let options = CalculationOptions {
        reference_study_period: project.reference_study_period.clone(),
        life_cycle_modules: project.life_cycle_modules.clone(),
        impact_categories: project.impact_categories.clone(),
        overwrite_existing_results: true,
    };

    calculate_assembly(assembly, &options)?;
    assert!(assembly.results.is_some());
    Ok(())
}
