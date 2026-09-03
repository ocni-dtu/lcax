use std::collections::HashMap;
use std::fs;
use std::path::Path;

use lcax_calculation::calculate::calculate_assembly;
use lcax_calculation::models::CalculationOptions;
use lcax_core::value::AnyValue;
use lcax_models::assembly::{Assembly, AssemblyReference};
use lcax_models::epd::{EPDReference, Standard, SubType, EPD};
use lcax_models::life_cycle_base::{ImpactCategory, ImpactCategoryKey, Impacts, LifeCycleModule};
use lcax_models::product::{ImpactData, Product, ProductReference};
use lcax_models::project::Project;
use lcax_models::shared::Unit;

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
fn test_assembly_quantity_applied_after_summing_product_impact_data() -> Result<(), String> {
    let mut assembly = Assembly {
        id: "1".to_string(),
        name: "Assembly 1".to_string(),
        description: None,
        comment: None,
        quantity: 2.0,
        unit: Unit::M,
        classification: None,
        products: vec![ProductReference::Product(Product {
            id: "1".to_string(),
            name: "Product 1".to_string(),
            description: None,
            reference_service_life: 20,
            impact_data: vec![epd_gwp_a1a3("818", 818.0), epd_gwp_a1a3("100", 100.0)],
            quantity: 1.0,
            unit: Unit::M,
            transport: None,
            results: None,
            meta_data: None,
        })],
        results: None,
        meta_data: None,
    };
    let options = CalculationOptions {
        reference_study_period: None,
        life_cycle_modules: vec![LifeCycleModule::A1A3],
        impact_categories: vec![ImpactCategoryKey::GWP],
        overwrite_existing_results: true,
    };

    let result = calculate_assembly(&mut assembly, &options)?;
    assert_eq!(
        *result
            .get(&ImpactCategoryKey::GWP)
            .unwrap()
            .get(&LifeCycleModule::A1A3)
            .unwrap(),
        Some(1836.0)
    );
    Ok(())
}

#[test]
fn test_rate_based_assembly_scales_non_rate_based_products() -> Result<(), String> {
    let mut assembly = Assembly {
        id: "1".to_string(),
        name: "Annual Operation Assembly".to_string(),
        description: None,
        comment: None,
        quantity: 2.0,
        unit: Unit::PCS,
        classification: None,
        products: vec![ProductReference::Product(Product {
            id: "1".to_string(),
            name: "Product".to_string(),
            description: None,
            reference_service_life: 50,
            impact_data: vec![epd_gwp_a1a3("10", 10.0)],
            quantity: 1.0,
            unit: Unit::M,
            transport: None,
            results: None,
            meta_data: None, // Not rate-based at product level
        })],
        results: None,
        meta_data: Some(HashMap::from([(
            "isAnnual".to_string(),
            Some(AnyValue::Bool(true)),
        )])), // Rate-based at assembly level
    };
    let options = CalculationOptions {
        reference_study_period: Some(50),
        life_cycle_modules: vec![LifeCycleModule::A1A3],
        impact_categories: vec![ImpactCategoryKey::GWP],
        overwrite_existing_results: true,
    };

    // Product impact: 10.0 * 1.0 = 10.0
    // Assembly is rate-based with RSP = 50: scaled by 50 -> 500.0
    // Assembly quantity = 2.0 -> 500.0 * 2.0 = 1000.0
    let result = calculate_assembly(&mut assembly, &options)?;
    assert_eq!(
        *result
            .get(&ImpactCategoryKey::GWP)
            .unwrap()
            .get(&LifeCycleModule::A1A3)
            .unwrap(),
        Some(1000.0)
    );
    Ok(())
}

#[test]
fn test_rate_based_assembly_avoids_double_scaling_rate_based_products() -> Result<(), String> {
    let mut assembly = Assembly {
        id: "1".to_string(),
        name: "Annual Operation Assembly".to_string(),
        description: None,
        comment: None,
        quantity: 2.0,
        unit: Unit::PCS,
        classification: None,
        products: vec![ProductReference::Product(Product {
            id: "1".to_string(),
            name: "Annual Product".to_string(),
            description: None,
            reference_service_life: 50,
            impact_data: vec![epd_gwp_a1a3("10", 10.0)],
            quantity: 1.0,
            unit: Unit::M,
            transport: None,
            results: None,
            meta_data: Some(HashMap::from([(
                "isAnnual".to_string(),
                Some(AnyValue::Bool(true)),
            )])), // Rate-based at product level
        })],
        results: None,
        meta_data: Some(HashMap::from([(
            "isAnnual".to_string(),
            Some(AnyValue::Bool(true)),
        )])), // Also rate-based at assembly level
    };
    let options = CalculationOptions {
        reference_study_period: Some(50),
        life_cycle_modules: vec![LifeCycleModule::A1A3],
        impact_categories: vec![ImpactCategoryKey::GWP],
        overwrite_existing_results: true,
    };

    // Product is rate-based: 10.0 * 1.0 * 50 = 500.0
    // Assembly does NOT double-scale by 50, only multiplies by assembly quantity = 2.0 -> 1000.0
    let result = calculate_assembly(&mut assembly, &options)?;
    assert_eq!(
        *result
            .get(&ImpactCategoryKey::GWP)
            .unwrap()
            .get(&LifeCycleModule::A1A3)
            .unwrap(),
        Some(1000.0)
    );
    Ok(())
}

fn epd_gwp_a1a3(id: &str, value: f64) -> ImpactData {
    ImpactData::EPD(EPDReference::EPD(EPD {
        id: id.to_string(),
        name: id.to_string(),
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
            ImpactCategory::from([(LifeCycleModule::A1A3, Some(value))]),
        )]),
        meta_data: None,
    }))
}
