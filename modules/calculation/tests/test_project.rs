use std::collections::HashMap;
use std::fs;
use std::path::Path;

use lcax_calculation::calculate::calculate_project;
use lcax_core::value::AnyValue;
use lcax_models::assembly::{Assembly, AssemblyReference};
use lcax_models::epd::{EPDReference, Standard, SubType, EPD};
use lcax_models::life_cycle_base::{ImpactCategory, ImpactCategoryKey, Impacts, LifeCycleModule};
use lcax_models::product::{ImpactData, Product, ProductReference};
use lcax_models::project::Project;
use lcax_models::shared::Unit;

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

#[test]
fn test_project_propagates_reference_study_period_to_rate_based_products() -> Result<(), String> {
    let product = Product {
        id: "1".to_string(),
        name: "Annual Product".to_string(),
        description: None,
        reference_service_life: 50,
        impact_data: vec![ImpactData::EPD(EPDReference::EPD(EPD {
            id: "1".to_string(),
            name: "EPD 1".to_string(),
            declared_unit: Unit::M,
            version: "".to_string(),
            published_date: Default::default(),
            valid_until: Default::default(),
            source: None,
            reference_service_life: None,
            standard: Standard::EN15804A1,
            comment: None,
            location: Default::default(),
            subtype: SubType::GENERIC,
            conversions: None,
            impacts: Impacts::from([(
                ImpactCategoryKey::GWP,
                ImpactCategory::from([(LifeCycleModule::B6, Some(2.0))]),
            )]),
            meta_data: None,
        }))],
        quantity: 5.0,
        unit: Unit::M,
        transport: None,
        results: None,
        meta_data: Some(HashMap::from([(
            "isAnnual".to_string(),
            Some(AnyValue::Bool(true)),
        )])),
    };

    let assembly = Assembly {
        id: "1".to_string(),
        name: "Assembly 1".to_string(),
        description: None,
        comment: None,
        quantity: 1.0,
        unit: Unit::PCS,
        classification: None,
        products: vec![ProductReference::Product(product)],
        results: None,
        meta_data: None,
    };

    let mut project = Project {
        id: "1".to_string(),
        name: "Project 1".to_string(),
        description: None,
        comment: None,
        location: Default::default(),
        owner: None,
        format_version: "".to_string(),
        lcia_method: None,
        classification_systems: None,
        reference_study_period: Some(50),
        life_cycle_modules: vec![LifeCycleModule::B6],
        impact_categories: vec![ImpactCategoryKey::GWP],
        assemblies: vec![AssemblyReference::Assembly(assembly)],
        results: None,
        project_info: None,
        project_phase: Default::default(),
        software_info: Default::default(),
        meta_data: None,
    };

    // Calculate with options: None -> should use project.reference_study_period (50)
    // Result: 5.0 quantity * 2.0 impact * 50 rsp * 1.0 assembly = 500.0
    calculate_project(&mut project, None)?;

    let gwp = *project
        .results
        .as_ref()
        .unwrap()
        .get(&ImpactCategoryKey::GWP)
        .unwrap()
        .get(&LifeCycleModule::B6)
        .unwrap();

    assert_eq!(gwp, Some(500.0));
    Ok(())
}
