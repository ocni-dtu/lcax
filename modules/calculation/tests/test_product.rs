use lcax_calculation::calculate::calculate_product;
use lcax_calculation::models::CalculationOptions;
use lcax_models::epd::{EPDReference, Standard, SubType, EPD};
use lcax_models::generic_impact_data::{GenericData, GenericDataReference};
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

#[test]
fn test_multiple_epds_sum_overlapping_modules_order_independent() -> Result<(), String> {
    let options = gwp_options(vec![LifeCycleModule::A1A3]);
    let first_then_second = calculate_product(
        &mut product_with_impact_data(
            vec![epd_gwp_a1a3("818", 818.0), epd_gwp_a1a3("100", 100.0)],
            1.0,
        ),
        &options,
    )?;
    let second_then_first = calculate_product(
        &mut product_with_impact_data(
            vec![epd_gwp_a1a3("100", 100.0), epd_gwp_a1a3("818", 818.0)],
            1.0,
        ),
        &options,
    )?;

    assert_eq!(
        gwp_at(&first_then_second, LifeCycleModule::A1A3),
        Some(918.0)
    );
    assert_eq!(
        gwp_at(&second_then_first, LifeCycleModule::A1A3),
        Some(918.0)
    );
    Ok(())
}

#[test]
fn test_mixed_epd_and_generic_data_sum_overlapping_modules() -> Result<(), String> {
    let mut product = product_with_impact_data(
        vec![epd_gwp_a1a3("818", 818.0), generic_gwp_a1a3("100", 100.0)],
        1.0,
    );
    let result = calculate_product(&mut product, &gwp_options(vec![LifeCycleModule::A1A3]))?;
    assert_eq!(gwp_at(&result, LifeCycleModule::A1A3), Some(918.0));
    Ok(())
}

#[test]
fn test_disjoint_modules_from_multiple_impact_data_all_appear() -> Result<(), String> {
    let mut product = product_with_impact_data(
        vec![
            epd_with_gwp("a1a3", LifeCycleModule::A1A3, 818.0),
            epd_with_gwp("a4", LifeCycleModule::A4, 100.0),
        ],
        1.0,
    );
    let result = calculate_product(
        &mut product,
        &gwp_options(vec![LifeCycleModule::A1A3, LifeCycleModule::A4]),
    )?;
    assert_eq!(gwp_at(&result, LifeCycleModule::A1A3), Some(818.0));
    assert_eq!(gwp_at(&result, LifeCycleModule::A4), Some(100.0));
    Ok(())
}

fn gwp_options(life_cycle_modules: Vec<LifeCycleModule>) -> CalculationOptions {
    CalculationOptions {
        reference_study_period: None,
        life_cycle_modules,
        impact_categories: vec![ImpactCategoryKey::GWP],
        overwrite_existing_results: true,
    }
}

fn gwp_at(impacts: &Impacts, module: LifeCycleModule) -> Option<f64> {
    *impacts
        .get(&ImpactCategoryKey::GWP)
        .unwrap()
        .get(&module)
        .unwrap()
}

fn gwp_impacts(module: LifeCycleModule, value: f64) -> Impacts {
    Impacts::from([(
        ImpactCategoryKey::GWP,
        ImpactCategory::from([(module, Some(value))]),
    )])
}

fn product_with_impact_data(impact_data: Vec<ImpactData>, quantity: f64) -> Product {
    Product {
        id: "1".to_string(),
        name: "Product 1".to_string(),
        description: None,
        reference_service_life: 20,
        impact_data,
        quantity,
        unit: Unit::M,
        transport: None,
        results: None,
        meta_data: None,
    }
}

fn epd_gwp_a1a3(id: &str, value: f64) -> ImpactData {
    epd_with_gwp(id, LifeCycleModule::A1A3, value)
}

fn epd_with_gwp(id: &str, module: LifeCycleModule, value: f64) -> ImpactData {
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
        impacts: gwp_impacts(module, value),
        meta_data: None,
    }))
}

fn generic_gwp_a1a3(id: &str, value: f64) -> ImpactData {
    ImpactData::GenericData(GenericDataReference::GenericData(GenericData {
        id: id.to_string(),
        name: id.to_string(),
        declared_unit: Unit::M,
        source: None,
        comment: None,
        conversions: None,
        impacts: gwp_impacts(LifeCycleModule::A1A3, value),
        meta_data: None,
    }))
}
