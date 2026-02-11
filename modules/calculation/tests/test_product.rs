use lcax_calculation::calculate::calculate_product;
use lcax_calculation::models::CalculationOptions;
use lcax_models::epd::{EPDReference, Standard, SubType, EPD};
use lcax_models::life_cycle_base::{ImpactCategory, ImpactCategoryKey, Impacts, LifeCycleModule};
use lcax_models::product::{ImpactData, Product};
use lcax_models::shared::Unit;
use std::fs;
use std::path::Path;

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
            source: None,
            reference_service_life: None,
            standard: Standard::EN15804A1,
            comment: None,
            location: Default::default(),
            subtype: SubType::GENERIC,
            conversions: None,
            impacts: Impacts::from([(
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

    let options = CalculationOptions {
        reference_study_period: None,
        life_cycle_modules: vec![LifeCycleModule::A1A3],
        impact_categories: vec![ImpactCategoryKey::GWP],
        overwrite_existing_results: true,
    };
    let result = calculate_product(&mut product, &options)?;
    assert_eq!(
        *result
            .get(&ImpactCategoryKey::GWP)
            .unwrap()
            .get(&LifeCycleModule::A1A3)
            .unwrap(),
        Some(15.0)
    );
    Ok(())
}

#[test]
fn test_with_conversion() -> Result<(), String> {
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    let file_path = root_dir.join("tests/datafixtures/products_conversion.json");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut products = serde_json::from_str::<Vec<Product>>(&contents).unwrap();
    let product = &mut products[0];

    let options = CalculationOptions {
        reference_study_period: None,
        life_cycle_modules: vec![LifeCycleModule::A1A3],
        impact_categories: vec![ImpactCategoryKey::GWP],
        overwrite_existing_results: true,
    };
    let result = calculate_product(product, &options)?;
    assert_eq!(
        *result
            .get(&ImpactCategoryKey::GWP)
            .unwrap()
            .get(&LifeCycleModule::A1A3)
            .unwrap(),
        Some(3210650.0)
    );
    Ok(())
}

#[test]
fn test_without_conversion() -> Result<(), String> {
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    let file_path = root_dir.join("tests/datafixtures/products_conversion.json");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut products = serde_json::from_str::<Vec<Product>>(&contents).unwrap();
    let product = &mut products[1];

    let options = CalculationOptions {
        reference_study_period: None,
        life_cycle_modules: vec![LifeCycleModule::A1A3],
        impact_categories: vec![ImpactCategoryKey::GWP],
        overwrite_existing_results: true,
    };
    match calculate_product(product, &options) {
        Ok(_) => Err(String::from("Did not fail")),
        Err(_) => Ok(()),
    }
}

#[test]
fn test_wrong_conversion() -> Result<(), String> {
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    let file_path = root_dir.join("tests/datafixtures/products_conversion.json");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut products = serde_json::from_str::<Vec<Product>>(&contents).unwrap();
    let product = &mut products[2];

    let options = CalculationOptions {
        reference_study_period: None,
        life_cycle_modules: vec![LifeCycleModule::A1A3],
        impact_categories: vec![ImpactCategoryKey::GWP],
        overwrite_existing_results: true,
    };
    match calculate_product(product, &options) {
        Ok(_) => Err(String::from("Did not fail")),
        Err(_) => Ok(()),
    }
}

#[test]
fn test_with_empty_impact_category() -> Result<(), String> {
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    let file_path = root_dir.join("tests/datafixtures/products_impacts.json");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut products = serde_json::from_str::<Vec<Product>>(&contents).unwrap();
    let product = &mut products[0];

    let options = CalculationOptions {
        reference_study_period: None,
        life_cycle_modules: vec![LifeCycleModule::A1A3],
        impact_categories: vec![ImpactCategoryKey::GWP],
        overwrite_existing_results: true,
    };
    let result = calculate_product(product, &options)?;
    assert_eq!(
        *result
            .get(&ImpactCategoryKey::GWP)
            .unwrap()
            .get(&LifeCycleModule::A1A3)
            .unwrap(),
        Some(0.0)
    );
    Ok(())
}

#[test]
fn test_with_empty_life_cycle() -> Result<(), String> {
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    let file_path = root_dir.join("tests/datafixtures/products_impacts.json");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut products = serde_json::from_str::<Vec<Product>>(&contents).unwrap();
    let product = &mut products[1];

    let options = CalculationOptions {
        reference_study_period: None,
        life_cycle_modules: vec![LifeCycleModule::A4],
        impact_categories: vec![ImpactCategoryKey::GWP],
        overwrite_existing_results: true,
    };
    let result = calculate_product(product, &options)?;
    assert_eq!(
        *result
            .get(&ImpactCategoryKey::GWP)
            .unwrap()
            .get(&LifeCycleModule::A4)
            .unwrap(),
        Some(0.0)
    );
    Ok(())
}
