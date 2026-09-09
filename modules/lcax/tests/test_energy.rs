use lcax::get_energy_assemblies;
use lcax_calculation::calculate::calculate_project;
use lcax_core::value::AnyValue;
use lcax_models::assembly::AssemblyReference;
use lcax_models::generic_impact_data::GenericDataReference;
use lcax_models::life_cycle_base::{ImpactCategoryKey, LifeCycleModule};
use lcax_models::product::{ImpactData, ProductReference};
use lcax_models::project::Project;
use lcax_models::shared::Unit;

#[test]
fn test_get_energy_assemblies_br18_unit_test() {
    let assemblies = get_energy_assemblies("BR18").expect("BR18 should be supported");
    assert_eq!(assemblies.len(), 3);

    let expected_carriers = [
        (
            "br18-electricity",
            "BR18 Grid Electricity",
            "El",
            "Operational Electricity",
            "Grid Electricity",
        ),
        (
            "br18-district-heating",
            "BR18 District Heating",
            "Fjernvarme",
            "Operational Heating",
            "District Heating",
        ),
        (
            "br18-natural-gas",
            "BR18 Natural Gas",
            "Ledningsgas",
            "Operational Gas",
            "Natural Gas",
        ),
    ];

    for (i, (expected_id, expected_name, exp_code, exp_class_name, exp_prod_name)) in
        expected_carriers.iter().enumerate()
    {
        let assembly = &assemblies[i];
        assert_eq!(&assembly.id, expected_id);
        assert_eq!(&assembly.name, expected_name);
        assert_eq!(assembly.unit, Unit::KWH);
        assert_eq!(assembly.quantity, 1.0);

        let classification = assembly
            .classification
            .as_ref()
            .expect("Classification should be present");
        assert_eq!(classification.len(), 1);
        assert_eq!(classification[0].system, "BR18");
        assert_eq!(classification[0].code, *exp_code);
        assert_eq!(classification[0].name, *exp_class_name);

        assert_eq!(assembly.products.len(), 1);
        let product = match &assembly.products[0] {
            ProductReference::Product(p) => p,
            _ => panic!("Expected ProductReference::Product"),
        };

        assert_eq!(&product.name, exp_prod_name);
        assert_eq!(product.unit, Unit::KWH);
        assert_eq!(product.quantity, 1.0);
        assert_eq!(product.reference_service_life, 50);
        assert_eq!(
            product.description.as_deref(),
            Some("Impact data should be linearly interpolated")
        );

        let meta = product
            .meta_data
            .as_ref()
            .expect("Meta data should be present");
        match meta.get("isAnnual") {
            Some(Some(AnyValue::Bool(b))) => assert!(!b, "isAnnual should be false"),
            _ => panic!("Expected isAnnual to be false in product meta_data"),
        }

        // Validates that each product has 5 GenericData milestone entries (years 2023, 2025, 2030, 2035, 2040)
        assert_eq!(product.impact_data.len(), 5);
        let expected_years = [2023, 2025, 2030, 2035, 2040];
        for (j, &exp_year) in expected_years.iter().enumerate() {
            let item = &product.impact_data[j];
            let gd = match item {
                ImpactData::GenericData(GenericDataReference::GenericData(gd)) => gd,
                _ => panic!("Expected ImpactData::GenericData"),
            };
            let year = gd
                .meta_data
                .as_ref()
                .and_then(|m| m.get("year"))
                .and_then(|v| match v {
                    Some(AnyValue::Number(lcax_core::value::Number::Int(y))) => Some(*y as u16),
                    _ => None,
                })
                .expect("Milestone GenericData should have milestone year in metadata");
            assert_eq!(year, exp_year);
        }
    }
}

#[test]
fn test_get_energy_assemblies_case_insensitive() {
    assert!(get_energy_assemblies("br18").is_ok());
    assert!(get_energy_assemblies("Br18").is_ok());
    assert!(get_energy_assemblies("BR18").is_ok());
}

#[test]
fn test_get_energy_assemblies_unsupported_standard() {
    match get_energy_assemblies("unknown") {
        Err(err) => assert!(err.contains("Unsupported standard: unknown")),
        Ok(_) => panic!("Expected error for unsupported standard"),
    }

    match get_energy_assemblies("LEED") {
        Err(err) => assert!(err.contains("Unsupported standard: LEED")),
        Ok(_) => panic!("Expected error for unsupported standard"),
    }
}

#[test]
fn test_energy_assembly_calculation_integration() -> Result<(), String> {
    let assemblies = get_energy_assemblies("BR18").expect("BR18 should be supported");
    let mut electricity_assembly = assemblies[0].clone();

    // Modify product quantity to 1000.0 kWh
    let product = match &mut electricity_assembly.products[0] {
        ProductReference::Product(p) => p,
        _ => panic!("Expected ProductReference::Product"),
    };
    product.quantity = 1000.0;

    let mut project = Project {
        id: "project-energy-test".to_string(),
        name: "Energy Test Project".to_string(),
        description: None,
        comment: None,
        location: Default::default(),
        owner: None,
        format_version: "2.0.0".to_string(),
        lcia_method: None,
        classification_systems: None,
        reference_study_period: Some(50),
        life_cycle_modules: vec![LifeCycleModule::B6],
        impact_categories: vec![ImpactCategoryKey::GWP],
        assemblies: vec![AssemblyReference::Assembly(electricity_assembly)],
        results: None,
        project_info: None,
        project_phase: Default::default(),
        software_info: Default::default(),
        meta_data: None,
    };

    calculate_project(&mut project, None)?;

    let gwp = project
        .results
        .as_ref()
        .and_then(|res| res.get(&ImpactCategoryKey::GWP))
        .and_then(|cat| cat.get(&LifeCycleModule::B6))
        .and_then(|val| *val)
        .expect("B6 GWP result should be present");

    // Analytical calculation for BR18 electricity (2023..2073, study_period = 50):
    // Milestones from BR18 generic data:
    // 2023: 0.187
    // 2025: 0.135
    // 2030: 0.047
    // 2035: 0.0414
    // 2040: 0.0403
    // Interval [2023, 2025]: (0.187 + 0.135) / 2 * 2 = 0.322
    // Interval [2025, 2030]: (0.135 + 0.047) / 2 * 5 = 0.455
    // Interval [2030, 2035]: (0.047 + 0.0414) / 2 * 5 = 0.221
    // Interval [2035, 2040]: (0.0414 + 0.0403) / 2 * 5 = 0.20425
    // Interval [2040, 2073]: 0.0403 * 33 = 1.3299
    // Total integral = 0.322 + 0.455 + 0.221 + 0.20425 + 1.3299 = 2.53215 kg CO2e / (kWh * year)
    // Time-weighted average = 2.53215 / 50 = 0.050643 kg CO2e / kWh
    // Product quantity = 1000.0 kWh (lifetime total)
    // Since isAnnual = false, rate_scale = 1.0 (no 50x double scaling)
    // Total GWP = 0.050643 * 1000.0 = 50.643 kg CO2e
    let expected_gwp = 50.643;
    assert!(
        (gwp - expected_gwp).abs() < 1e-4,
        "Expected GWP to be close to {}, got {}",
        expected_gwp,
        gwp
    );

    Ok(())
}
