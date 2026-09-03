use lcax_calculation::calculate::calculate_product;
use lcax_calculation::models::CalculationOptions;
use lcax_core::value::{AnyValue, Number};
use lcax_models::epd::{EPDReference, Standard, SubType, EPD};
use lcax_models::generic_impact_data::{GenericData, GenericDataReference};
use lcax_models::life_cycle_base::{ImpactCategory, ImpactCategoryKey, Impacts, LifeCycleModule};
use lcax_models::product::{ImpactData, Product};
use lcax_models::shared::Unit;
use std::collections::HashMap;
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

fn generic_gwp_b6(name: &str, value: f64) -> ImpactData {
    ImpactData::GenericData(GenericDataReference::GenericData(GenericData {
        id: name.to_string(),
        name: name.to_string(),
        declared_unit: Unit::KWH,
        source: None,
        comment: None,
        conversions: None,
        impacts: gwp_impacts(LifeCycleModule::B6, value),
        meta_data: None,
    }))
}

#[test]
fn test_rate_based_product_scales_with_reference_study_period() -> Result<(), String> {
    let mut product = Product {
        id: "1".to_string(),
        name: "Electricity".to_string(),
        description: None,
        reference_service_life: 20,
        impact_data: vec![generic_gwp_b6("Electricity", 0.15)],
        quantity: 10.0,
        unit: Unit::KWH,
        transport: None,
        results: None,
        meta_data: Some(HashMap::from([(
            "isAnnual".to_string(),
            Some(AnyValue::Bool(true)),
        )])),
    };

    // With RSP = 50: 10.0 * 0.15 * 50 = 75.0
    let options_50 = CalculationOptions {
        reference_study_period: Some(50),
        life_cycle_modules: vec![LifeCycleModule::B6],
        impact_categories: vec![ImpactCategoryKey::GWP],
        overwrite_existing_results: true,
    };
    let result_50 = calculate_product(&mut product, &options_50)?;
    assert_eq!(gwp_at(&result_50, LifeCycleModule::B6), Some(75.0));

    // With RSP = 30: 10.0 * 0.15 * 30 = 45.0
    let options_30 = CalculationOptions {
        reference_study_period: Some(30),
        life_cycle_modules: vec![LifeCycleModule::B6],
        impact_categories: vec![ImpactCategoryKey::GWP],
        overwrite_existing_results: true,
    };
    let result_30 = calculate_product(&mut product, &options_30)?;
    assert_eq!(gwp_at(&result_30, LifeCycleModule::B6), Some(45.0));

    // With RSP = None: falls back to product.reference_service_life (20) -> 10.0 * 0.15 * 20 = 30.0
    let options_none = CalculationOptions {
        reference_study_period: None,
        life_cycle_modules: vec![LifeCycleModule::B6],
        impact_categories: vec![ImpactCategoryKey::GWP],
        overwrite_existing_results: true,
    };
    let result_none = calculate_product(&mut product, &options_none)?;
    assert_eq!(gwp_at(&result_none, LifeCycleModule::B6), Some(30.0));

    Ok(())
}

#[test]
fn test_non_rate_based_product_ignores_reference_study_period() -> Result<(), String> {
    let mut product = Product {
        id: "1".to_string(),
        name: "Electricity Lifetime".to_string(),
        description: None,
        reference_service_life: 20,
        impact_data: vec![generic_gwp_b6("Electricity", 0.15)],
        quantity: 500.0,
        unit: Unit::KWH,
        transport: None,
        results: None,
        meta_data: None, // Not rate-based
    };

    let options_50 = CalculationOptions {
        reference_study_period: Some(50),
        life_cycle_modules: vec![LifeCycleModule::B6],
        impact_categories: vec![ImpactCategoryKey::GWP],
        overwrite_existing_results: true,
    };
    let result_50 = calculate_product(&mut product, &options_50)?;
    // Cumulative quantity: 500.0 * 0.15 = 75.0 (not multiplied by 50)
    assert_eq!(gwp_at(&result_50, LifeCycleModule::B6), Some(75.0));
    Ok(())
}

#[test]
fn test_milestone_interpolation_time_weighted_average() -> Result<(), String> {
    // Milestones from Danish BR18 electricity decarbonization:
    // 2023: 0.187, 2025: 0.135, 2030: 0.047, 2035: 0.0414, 2040: 0.0403
    // Over 50 years (2023 to 2073):
    // [2023, 2025]: dt = 2, avg = (0.187 + 0.135)/2 = 0.161, area = 0.322
    // [2025, 2030]: dt = 5, avg = (0.135 + 0.047)/2 = 0.091, area = 0.455
    // [2030, 2035]: dt = 5, avg = (0.047 + 0.0414)/2 = 0.0442, area = 0.221
    // [2035, 2040]: dt = 5, avg = (0.0414 + 0.0403)/2 = 0.04085, area = 0.20425
    // [2040, 2073]: dt = 33, avg = 0.0403, area = 1.3299
    // Total area = 2.53215
    // Time-weighted avg = 2.53215 / 50 = 0.050643 kg CO2e / kWh
    let milestones = vec![
        generic_gwp_b6("Electricity 2023", 0.187),
        generic_gwp_b6("Electricity 2025", 0.135),
        generic_gwp_b6("Electricity 2030", 0.047),
        generic_gwp_b6("Electricity 2035", 0.0414),
        generic_gwp_b6("Electricity 2040", 0.0403),
    ];

    let mut product = Product {
        id: "1".to_string(),
        name: "Electricity Cumulative".to_string(),
        description: None,
        reference_service_life: 50,
        impact_data: milestones,
        quantity: 1000.0,
        unit: Unit::KWH,
        transport: None,
        results: None,
        meta_data: None, // Cumulative quantity
    };

    let options = CalculationOptions {
        reference_study_period: Some(50),
        life_cycle_modules: vec![LifeCycleModule::B6],
        impact_categories: vec![ImpactCategoryKey::GWP],
        overwrite_existing_results: true,
    };
    let result = calculate_product(&mut product, &options)?;
    let gwp = gwp_at(&result, LifeCycleModule::B6).unwrap();

    // Expected: 1000.0 * 0.050643 = 50.643
    // Blind sum would give: 1000.0 * (0.187 + 0.135 + 0.047 + 0.0414 + 0.0403) = 450.7 (9x higher!)
    assert!((gwp - 50.643).abs() < 1e-3, "Expected ~50.643, got {}", gwp);
    Ok(())
}

#[test]
fn test_milestone_interpolation_rate_based_annual() -> Result<(), String> {
    let milestones = vec![
        generic_gwp_b6("Electricity 2023", 0.187),
        generic_gwp_b6("Electricity 2025", 0.135),
        generic_gwp_b6("Electricity 2030", 0.047),
        generic_gwp_b6("Electricity 2035", 0.0414),
        generic_gwp_b6("Electricity 2040", 0.0403),
    ];

    let mut product = Product {
        id: "1".to_string(),
        name: "Electricity Annual".to_string(),
        description: Some("Impact data should be linearly interpolated".to_string()),
        reference_service_life: 50,
        impact_data: milestones,
        quantity: 20.0, // 20 kWh/year
        unit: Unit::KWH,
        transport: None,
        results: None,
        meta_data: Some(HashMap::from([(
            "isAnnual".to_string(),
            Some(AnyValue::Bool(true)),
        )])),
    };

    let options = CalculationOptions {
        reference_study_period: Some(50),
        life_cycle_modules: vec![LifeCycleModule::B6],
        impact_categories: vec![ImpactCategoryKey::GWP],
        overwrite_existing_results: true,
    };
    let result = calculate_product(&mut product, &options)?;
    let gwp = gwp_at(&result, LifeCycleModule::B6).unwrap();

    // Expected: 20.0 kWh/year * 50 years * 0.050643 kg CO2e / kWh = 50.643 kg CO2e
    assert!((gwp - 50.643).abs() < 1e-3, "Expected ~50.643, got {}", gwp);
    Ok(())
}

#[test]
fn test_milestone_order_independent() -> Result<(), String> {
    let ordered = vec![
        generic_gwp_b6("Electricity 2023", 0.187),
        generic_gwp_b6("Electricity 2025", 0.135),
        generic_gwp_b6("Electricity 2030", 0.047),
        generic_gwp_b6("Electricity 2035", 0.0414),
        generic_gwp_b6("Electricity 2040", 0.0403),
    ];
    let shuffled = vec![
        generic_gwp_b6("Electricity 2040", 0.0403),
        generic_gwp_b6("Electricity 2025", 0.135),
        generic_gwp_b6("Electricity 2023", 0.187),
        generic_gwp_b6("Electricity 2035", 0.0414),
        generic_gwp_b6("Electricity 2030", 0.047),
    ];

    let options = CalculationOptions {
        reference_study_period: Some(50),
        life_cycle_modules: vec![LifeCycleModule::B6],
        impact_categories: vec![ImpactCategoryKey::GWP],
        overwrite_existing_results: true,
    };

    let mut p1 = Product {
        id: "1".to_string(),
        name: "P1".to_string(),
        description: None,
        reference_service_life: 50,
        impact_data: ordered,
        quantity: 100.0,
        unit: Unit::KWH,
        transport: None,
        results: None,
        meta_data: None,
    };

    let mut p2 = Product {
        id: "2".to_string(),
        name: "P2".to_string(),
        description: None,
        reference_service_life: 50,
        impact_data: shuffled,
        quantity: 100.0,
        unit: Unit::KWH,
        transport: None,
        results: None,
        meta_data: None,
    };

    let r1 = calculate_product(&mut p1, &options)?;
    let r2 = calculate_product(&mut p2, &options)?;

    assert_eq!(
        gwp_at(&r1, LifeCycleModule::B6),
        gwp_at(&r2, LifeCycleModule::B6)
    );
    Ok(())
}

#[test]
fn test_milestone_interpolation_with_custom_start_year() -> Result<(), String> {
    // Starting in 2025:
    // [2025, 2030]: dt = 5, avg = 0.091, area = 0.455
    // [2030, 2035]: dt = 5, avg = 0.0442, area = 0.221
    // [2035, 2040]: dt = 5, avg = 0.04085, area = 0.20425
    // [2040, 2075]: dt = 35, avg = 0.0403, area = 1.4105
    // Total area = 2.29075
    // Average = 2.29075 / 50 = 0.045815
    let milestones = vec![
        generic_gwp_b6("Electricity 2023", 0.187),
        generic_gwp_b6("Electricity 2025", 0.135),
        generic_gwp_b6("Electricity 2030", 0.047),
        generic_gwp_b6("Electricity 2035", 0.0414),
        generic_gwp_b6("Electricity 2040", 0.0403),
    ];

    let mut product = Product {
        id: "1".to_string(),
        name: "Electricity 2025 Start".to_string(),
        description: None,
        reference_service_life: 50,
        impact_data: milestones,
        quantity: 1000.0,
        unit: Unit::KWH,
        transport: None,
        results: None,
        meta_data: Some(HashMap::from([(
            "startYear".to_string(),
            Some(AnyValue::Number(Number::Int(2025))),
        )])),
    };

    let options = CalculationOptions {
        reference_study_period: Some(50),
        life_cycle_modules: vec![LifeCycleModule::B6],
        impact_categories: vec![ImpactCategoryKey::GWP],
        overwrite_existing_results: true,
    };
    let result = calculate_product(&mut product, &options)?;
    let gwp = gwp_at(&result, LifeCycleModule::B6).unwrap();

    // Expected: 1000.0 * 0.045815 = 45.815
    assert!((gwp - 45.815).abs() < 1e-3, "Expected ~45.815, got {}", gwp);
    Ok(())
}
