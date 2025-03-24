use std::collections::HashMap;
use std::fs;
use std::path::Path;

use lcax_calculation::calculate::{
    calculate_assembly, calculate_product, calculate_project, CalculationOptions,
};
use lcax_models::assembly::AssemblyReference;
use lcax_models::epd::{EPDReference, Standard, SubType, EPD};
use lcax_models::life_cycle_base::{ImpactCategory, ImpactCategoryKey, LifeCycleModule};
use lcax_models::product::{ImpactData, Product};
use lcax_models::project::Project;
use lcax_models::shared::Unit;

#[test]
fn test_calculate_project() -> Result<(), String> {
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    let file_path = root_dir.join("tests/datafixtures/project.json");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut project = serde_json::from_str::<Project>(&contents).unwrap();

    calculate_project(&mut project, None).unwrap();
    assert!(project.results.is_some());
    Ok(())
}

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

#[test]
fn test_calculate_product() -> Result<(), String> {
    let mut product = Product {
        id: "1".to_string(),
        name: "Product 1".to_string(),
        description: None,
        reference_service_life: 20,
        impact_data: vec![ImpactData::EPD(EPDReference::EPD(EPD {
            id: "1".to_string(),
            name: "EPD 1".to_string(),
            declared_unit: Unit::M,
            version: "".to_string(),
            published_date: Default::default(),
            valid_until: Default::default(),
            format_version: "".to_string(),
            source: None,
            reference_service_life: None,
            standard: Standard::EN15804A1,
            comment: None,
            location: Default::default(),
            subtype: SubType::GENERIC,
            conversions: None,
            impacts: HashMap::from([(
                ImpactCategoryKey::GWP,
                ImpactCategory::from([(LifeCycleModule::A1A3, Some(3.0))]),
            )]),
            meta_data: None,
        }))],
        quantity: 5.0,
        unit: Unit::M,
        transport: None,
        results: None,
        meta_data: None,
    };

    let options = lcax_calculation::calculate::CalculationOptions {
        reference_study_period: None,
        life_cycle_modules: vec![LifeCycleModule::A1A3],
        impact_categories: vec![ImpactCategoryKey::GWP],
        overwrite_existing_results: true,
    };
    let result = calculate_product(&mut product, &options).unwrap();
    assert_eq!(
        result[&ImpactCategoryKey::GWP][&LifeCycleModule::A1A3],
        Some(15.0)
    );
    Ok(())
}
