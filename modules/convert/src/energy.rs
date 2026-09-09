use std::collections::HashMap;

use lcax_core::value::AnyValue;
use lcax_models::assembly::{Assembly, Classification};
use lcax_models::generic_impact_data::{GenericData, GenericDataReference};
use lcax_models::product::{ImpactData, Product, ProductReference};
use lcax_models::shared::Unit;

use crate::br_standard::br18_generic_data::{
    get_district_heating_data, get_electricity_data, get_lng_data,
};

fn milestones_to_impact_data(data: &HashMap<u16, GenericData>) -> Vec<ImpactData> {
    let mut years: Vec<u16> = data.keys().copied().collect();
    years.sort();
    years
        .into_iter()
        .filter_map(|year| data.get(&year).map(|gd| (year, gd.clone())))
        .map(|(year, mut gd)| {
            let mut meta = gd.meta_data.unwrap_or_default();
            meta.insert(
                "year".to_string(),
                Some(AnyValue::Number(lcax_core::value::Number::Int(year as i64))),
            );
            gd.meta_data = Some(meta);
            ImpactData::GenericData(GenericDataReference::GenericData(gd))
        })
        .collect()
}

pub fn get_energy_assemblies(standard: &str) -> Result<Vec<Assembly>, String> {
    match standard.to_uppercase().as_str() {
        "BR18" => Ok(get_br18_energy_assemblies()),
        _ => Err(format!("Unsupported standard: {standard}")),
    }
}

fn get_br18_energy_assemblies() -> Vec<Assembly> {
    vec![
        Assembly {
            id: "br18-electricity".to_string(),
            name: "BR18 Grid Electricity".to_string(),
            description: None,
            comment: None,
            quantity: 1.0,
            unit: Unit::KWH,
            classification: Some(vec![Classification {
                system: "BR18".into(),
                code: "El".into(),
                name: "Operational Electricity".into(),
            }]),
            products: vec![ProductReference::Product(Product {
                id: "br18-electricity-product".to_string(),
                name: "Grid Electricity".to_string(),
                description: Some("Impact data should be linearly interpolated".to_string()),
                reference_service_life: 50,
                impact_data: milestones_to_impact_data(&get_electricity_data()),
                quantity: 1.0,
                unit: Unit::KWH,
                transport: None,
                results: None,
                meta_data: Some(HashMap::from([(
                    "isAnnual".to_string(),
                    Some(AnyValue::Bool(false)),
                )])),
            })],
            results: None,
            meta_data: None,
        },
        Assembly {
            id: "br18-district-heating".to_string(),
            name: "BR18 District Heating".to_string(),
            description: None,
            comment: None,
            quantity: 1.0,
            unit: Unit::KWH,
            classification: Some(vec![Classification {
                system: "BR18".into(),
                code: "Fjernvarme".into(),
                name: "Operational Heating".into(),
            }]),
            products: vec![ProductReference::Product(Product {
                id: "br18-district-heating-product".to_string(),
                name: "District Heating".to_string(),
                description: Some("Impact data should be linearly interpolated".to_string()),
                reference_service_life: 50,
                impact_data: milestones_to_impact_data(&get_district_heating_data()),
                quantity: 1.0,
                unit: Unit::KWH,
                transport: None,
                results: None,
                meta_data: Some(HashMap::from([(
                    "isAnnual".to_string(),
                    Some(AnyValue::Bool(false)),
                )])),
            })],
            results: None,
            meta_data: None,
        },
        Assembly {
            id: "br18-natural-gas".to_string(),
            name: "BR18 Natural Gas".to_string(),
            description: None,
            comment: None,
            quantity: 1.0,
            unit: Unit::KWH,
            classification: Some(vec![Classification {
                system: "BR18".into(),
                code: "Ledningsgas".into(),
                name: "Operational Gas".into(),
            }]),
            products: vec![ProductReference::Product(Product {
                id: "br18-natural-gas-product".to_string(),
                name: "Natural Gas".to_string(),
                description: Some("Impact data should be linearly interpolated".to_string()),
                reference_service_life: 50,
                impact_data: milestones_to_impact_data(&get_lng_data()),
                quantity: 1.0,
                unit: Unit::KWH,
                transport: None,
                results: None,
                meta_data: Some(HashMap::from([(
                    "isAnnual".to_string(),
                    Some(AnyValue::Bool(false)),
                )])),
            })],
            results: None,
            meta_data: None,
        },
    ]
}
