use lcax_core::utils::get_version;
use lcax_models::generic_impact_data::GenericData;
use lcax_models::life_cycle_base::{ImpactCategoryKey, LifeCycleStage};
use lcax_models::shared::{Source, Unit};
use std::collections::HashMap;
use std::string::ToString;

pub fn get_electricity_data() -> HashMap<u16, GenericData> {
    HashMap::from([
        (2023, GenericData {
            id: "de518e23-c435-40f5-b193-063959be13e0".to_string(),
            name: "Electricity 2023".to_string(),
            declared_unit: Unit::KWH,
            format_version: get_version(),
            source: Some(Source {
                name: "Bygningsreglementet".to_string(),
                url: Some("https://www.bygningsreglementet.dk/bilag/b2/bilag_2/tabel_8/#c93d7bfe-6c32-4182-b858-cf113ba2a371".to_string())
            }),
            comment: None,
            conversions: None,
            impacts: HashMap::from([
                (ImpactCategoryKey::GWP, HashMap::from([(LifeCycleStage::B6, Some(0.187))])),
            ]),
            meta_data: None,
        }),
        (2025, GenericData {
            id: "09ff896a-9fca-459f-8904-670fbf41d52b".to_string(),
            name: "Electricity 2025".to_string(),
            declared_unit: Unit::KWH,
            format_version: get_version(),
            source: Some(Source {
                name: "Bygningsreglementet".to_string(),
                url: Some("https://www.bygningsreglementet.dk/bilag/b2/bilag_2/tabel_8/#c93d7bfe-6c32-4182-b858-cf113ba2a371".to_string())
            }),
            comment: None,
            conversions: None,
            impacts: HashMap::from([
                (ImpactCategoryKey::GWP, HashMap::from([(LifeCycleStage::B6, Some(0.135))])),
            ]),
            meta_data: None,
        }),
        (2030, GenericData {
            id: "d8721117-838b-4b2d-a746-0ec8aef604f7".to_string(),
            name: "Electricity 2030".to_string(),
            declared_unit: Unit::KWH,
            format_version: get_version(),
            source: Some(Source {
                name: "Bygningsreglementet".to_string(),
                url: Some("https://www.bygningsreglementet.dk/bilag/b2/bilag_2/tabel_8/#c93d7bfe-6c32-4182-b858-cf113ba2a371".to_string())
            }),
            comment: None,
            conversions: None,
            impacts: HashMap::from([
                (ImpactCategoryKey::GWP, HashMap::from([(LifeCycleStage::B6, Some(0.047))])),
            ]),
            meta_data: None,
        }),
        (2035, GenericData {
            id: "8036eed7-ba5a-4f41-baa4-23a801c5a34f".to_string(),
            name: "Electricity 2035".to_string(),
            declared_unit: Unit::KWH,
            format_version: get_version(),
            source: Some(Source {
                name: "Bygningsreglementet".to_string(),
                url: Some("https://www.bygningsreglementet.dk/bilag/b2/bilag_2/tabel_8/#c93d7bfe-6c32-4182-b858-cf113ba2a371".to_string())
            }),
            comment: None,
            conversions: None,
            impacts: HashMap::from([
                (ImpactCategoryKey::GWP, HashMap::from([(LifeCycleStage::B6, Some(0.0414))])),
            ]),
            meta_data: None,
        }),
        (2040, GenericData {
            id: "0a5fc280-b78a-48ec-aed8-23535217a3d4".to_string(),
            name: "Electricity 2040".to_string(),
            declared_unit: Unit::KWH,
            format_version: get_version(),
            source: Some(Source {
                name: "Bygningsreglementet".to_string(),
                url: Some("https://www.bygningsreglementet.dk/bilag/b2/bilag_2/tabel_8/#c93d7bfe-6c32-4182-b858-cf113ba2a371".to_string())
            }),
            comment: None,
            conversions: None,
            impacts: HashMap::from([
                (ImpactCategoryKey::GWP, HashMap::from([(LifeCycleStage::B6, Some(0.0403))])),
            ]),
            meta_data: None,
        })
    ])
}

pub fn get_district_heating_data() -> HashMap<u16, GenericData> {
    HashMap::from([
        (2023, GenericData {
            id: "66fac734-fb28-4de0-bbdf-cddb66b47e59".to_string(),
            name: "District Heating 2023".to_string(),
            declared_unit: Unit::KWH,
            format_version: get_version(),
            source: Some(Source {
                name: "Bygningsreglementet".to_string(),
                url: Some("https://www.bygningsreglementet.dk/bilag/b2/bilag_2/tabel_8/#c93d7bfe-6c32-4182-b858-cf113ba2a371".to_string())
            }),
            comment: None,
            conversions: None,
            impacts: HashMap::from([
                (ImpactCategoryKey::GWP, HashMap::from([(LifeCycleStage::B6, Some(0.105))])),
            ]),
            meta_data: None,
        }),
        (2025, GenericData {
            id: "d82ad0b7-7d5f-47de-b931-8a4a4ca54768".to_string(),
            name: "District Heating 2025".to_string(),
            declared_unit: Unit::KWH,
            format_version: get_version(),
            source: Some(Source {
                name: "Bygningsreglementet".to_string(),
                url: Some("https://www.bygningsreglementet.dk/bilag/b2/bilag_2/tabel_8/#c93d7bfe-6c32-4182-b858-cf113ba2a371".to_string())
            }),
            comment: None,
            conversions: None,
            impacts: HashMap::from([
                (ImpactCategoryKey::GWP, HashMap::from([(LifeCycleStage::B6, Some(0.0878))])),
            ]),
            meta_data: None,
        }),
        (2030, GenericData {
            id: "c2acd759-e97a-4c2b-a314-2f9c613d4344".to_string(),
            name: "District Heating 2030".to_string(),
            declared_unit: Unit::KWH,
            format_version: get_version(),
            source: Some(Source {
                name: "Bygningsreglementet".to_string(),
                url: Some("https://www.bygningsreglementet.dk/bilag/b2/bilag_2/tabel_8/#c93d7bfe-6c32-4182-b858-cf113ba2a371".to_string())
            }),
            comment: None,
            conversions: None,
            impacts: HashMap::from([
                (ImpactCategoryKey::GWP, HashMap::from([(LifeCycleStage::B6, Some(0.0713))])),
            ]),
            meta_data: None,
        }),
        (2035, GenericData {
            id: "e98e4ba0-0a56-440f-a307-ddf57f8c10d3".to_string(),
            name: "District Heating 2035".to_string(),
            declared_unit: Unit::KWH,
            format_version: get_version(),
            source: Some(Source {
                name: "Bygningsreglementet".to_string(),
                url: Some("https://www.bygningsreglementet.dk/bilag/b2/bilag_2/tabel_8/#c93d7bfe-6c32-4182-b858-cf113ba2a371".to_string())
            }),
            comment: None,
            conversions: None,
            impacts: HashMap::from([
                (ImpactCategoryKey::GWP, HashMap::from([(LifeCycleStage::B6, Some(0.0688))])),
            ]),
            meta_data: None,
        }),
        (2040, GenericData {
            id: "2cffec3c-519d-43c6-9416-9f2cd34f2a70".to_string(),
            name: "District Heating 2040".to_string(),
            declared_unit: Unit::KWH,
            format_version: get_version(),
            source: Some(Source {
                name: "Bygningsreglementet".to_string(),
                url: Some("https://www.bygningsreglementet.dk/bilag/b2/bilag_2/tabel_8/#c93d7bfe-6c32-4182-b858-cf113ba2a371".to_string())
            }),
            comment: None,
            conversions: None,
            impacts: HashMap::from([
                (ImpactCategoryKey::GWP, HashMap::from([(LifeCycleStage::B6, Some(0.068))])),
            ]),
            meta_data: None,
        })
    ])
}

pub fn get_lng_data() -> HashMap<u16, GenericData> {
    HashMap::from([
        (2023, GenericData {
            id: "1b8f6cd6-1ba3-4d5e-b8aa-14009d945177".to_string(),
            name: "Liquefied Natural Gas 2023".to_string(),
            declared_unit: Unit::KWH,
            format_version: get_version(),
            source: Some(Source {
                name: "Bygningsreglementet".to_string(),
                url: Some("https://www.bygningsreglementet.dk/bilag/b2/bilag_2/tabel_8/#c93d7bfe-6c32-4182-b858-cf113ba2a371".to_string())
            }),
            comment: None,
            conversions: None,
            impacts: HashMap::from([
                (ImpactCategoryKey::GWP, HashMap::from([(LifeCycleStage::B6, Some(0.105))])),
            ]),
            meta_data: None,
        }),
        (2025, GenericData {
            id: "6b2db830-083e-434f-bc8f-e9be322e13a9".to_string(),
            name: "Liquefied Natural Gas 2025".to_string(),
            declared_unit: Unit::KWH,
            format_version: get_version(),
            source: Some(Source {
                name: "Bygningsreglementet".to_string(),
                url: Some("https://www.bygningsreglementet.dk/bilag/b2/bilag_2/tabel_8/#c93d7bfe-6c32-4182-b858-cf113ba2a371".to_string())
            }),
            comment: None,
            conversions: None,
            impacts: HashMap::from([
                (ImpactCategoryKey::GWP, HashMap::from([(LifeCycleStage::B6, Some(0.0878))])),
            ]),
            meta_data: None,
        }),
        (2030, GenericData {
            id: "494aaf28-47b7-4b9f-a533-9aaaa223262a".to_string(),
            name: "Liquefied Natural Gas 2030".to_string(),
            declared_unit: Unit::KWH,
            format_version: get_version(),
            source: Some(Source {
                name: "Bygningsreglementet".to_string(),
                url: Some("https://www.bygningsreglementet.dk/bilag/b2/bilag_2/tabel_8/#c93d7bfe-6c32-4182-b858-cf113ba2a371".to_string())
            }),
            comment: None,
            conversions: None,
            impacts: HashMap::from([
                (ImpactCategoryKey::GWP, HashMap::from([(LifeCycleStage::B6, Some(0.0713))])),
            ]),
            meta_data: None,
        }),
        (2035, GenericData {
            id: "e62e74e1-9d7a-486b-8d78-294766f8eab0".to_string(),
            name: "Liquefied Natural Gas 2035".to_string(),
            declared_unit: Unit::KWH,
            format_version: get_version(),
            source: Some(Source {
                name: "Bygningsreglementet".to_string(),
                url: Some("https://www.bygningsreglementet.dk/bilag/b2/bilag_2/tabel_8/#c93d7bfe-6c32-4182-b858-cf113ba2a371".to_string())
            }),
            comment: None,
            conversions: None,
            impacts: HashMap::from([
                (ImpactCategoryKey::GWP, HashMap::from([(LifeCycleStage::B6, Some(0.0688))])),
            ]),
            meta_data: None,
        }),
        (2040, GenericData {
            id: "1ad3e994-8142-4c44-acf4-1adaa7b38191".to_string(),
            name: "Liquefied Natural Gas 2040".to_string(),
            declared_unit: Unit::KWH,
            format_version: get_version(),
            source: Some(Source {
                name: "Bygningsreglementet".to_string(),
                url: Some("https://www.bygningsreglementet.dk/bilag/b2/bilag_2/tabel_8/#c93d7bfe-6c32-4182-b858-cf113ba2a371".to_string())
            }),
            comment: None,
            conversions: None,
            impacts: HashMap::from([
                (ImpactCategoryKey::GWP, HashMap::from([(LifeCycleStage::B6, Some(0.068))])),
            ]),
            meta_data: None,
        })
    ])
}

pub fn get_energy_data(year: &u16, data: HashMap<u16, GenericData>) -> Vec<GenericData> {
    match year {
        year if year < &2025 => {
            vec![
                data[&2023].clone(),
                data[&2025].clone(),
                data[&2030].clone(),
                data[&2035].clone(),
                data[&2040].clone(),
            ]
        }
        year if year < &2030 => {
            vec![
                data[&2025].clone(),
                data[&2030].clone(),
                data[&2035].clone(),
                data[&2040].clone(),
            ]
        }
        year if year < &2035 => {
            vec![
                data[&2030].clone(),
                data[&2035].clone(),
                data[&2040].clone(),
            ]
        }
        year if year < &2040 => {
            vec![data[&2035].clone(), data[&2040].clone()]
        }
        _ => vec![data[&2040].clone()],
    }
}
