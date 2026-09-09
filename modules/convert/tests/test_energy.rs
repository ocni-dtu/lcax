use lcax_convert::get_energy_assemblies;
use lcax_core::value::AnyValue;
use lcax_models::generic_impact_data::GenericDataReference;
use lcax_models::product::{ImpactData, ProductReference};
use lcax_models::shared::Unit;

#[test]
fn test_convert_get_energy_assemblies_br18() {
    let assemblies = get_energy_assemblies("BR18").expect("BR18 should be supported");
    assert_eq!(assemblies.len(), 3);

    for assembly in &assemblies {
        assert_eq!(assembly.unit, Unit::KWH);
        assert_eq!(assembly.quantity, 1.0);
        assert_eq!(assembly.products.len(), 1);

        let product = match &assembly.products[0] {
            ProductReference::Product(p) => p,
            _ => panic!("Expected ProductReference::Product"),
        };

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
fn test_convert_get_energy_assemblies_unsupported() {
    assert!(get_energy_assemblies("foo").is_err());
}
