use std::collections::HashMap;

use chrono::NaiveDate;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[cfg(feature = "jsbindings")]
use tsify::Tsify;

use lcax_core::country::Country;
use lcax_core::dates::{deserialize_yyyy_mm_dd, serialize_yyyy_mm_dd};
use lcax_core::utils::get_version;

use crate::life_cycle_base::{ImpactCategory, ImpactCategoryKey};
use crate::shared::{Conversion, Source, Unit};

#[derive(Serialize, Deserialize, JsonSchema, Clone)]
#[cfg_attr(
    feature = "jsbindings",
    derive(Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
pub struct EPD {
    pub id: String,
    pub name: String,
    pub declared_unit: Unit,
    pub version: String,

    #[serde(serialize_with = "serialize_yyyy_mm_dd")]
    #[serde(deserialize_with = "deserialize_yyyy_mm_dd")]
    pub published_date: NaiveDate,

    #[serde(serialize_with = "serialize_yyyy_mm_dd")]
    #[serde(deserialize_with = "deserialize_yyyy_mm_dd")]
    pub valid_until: NaiveDate,

    pub format_version: String,
    pub source: Option<Source>,
    pub reference_service_life: Option<u32>,
    pub standard: Standard,
    pub comment: Option<String>,
    pub location: Country,
    pub subtype: SubType,
    pub conversions: Option<Vec<Conversion>>,
    pub impacts: HashMap<ImpactCategoryKey, ImpactCategory>,
    pub meta_data: Option<HashMap<String, String>>,
}

impl EPD {
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: "".to_string(),
            declared_unit: Unit::UNKNOWN,
            version: get_version(),
            published_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            valid_until: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            format_version: "".to_string(),
            source: None,
            reference_service_life: None,
            standard: Standard::UNKNOWN,
            comment: None,
            location: Country::UNKNOWN,
            subtype: SubType::Generic,
            conversions: None,
            impacts: Default::default(),
            meta_data: None,
        }
    }
}

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub enum Standard {
    EN15804A1,
    EN15804A2,
    UNKNOWN,
}

impl From<&String> for Standard {
    fn from(value: &String) -> Self {
        if value.to_ascii_lowercase().contains("15804+a2") {
            Standard::EN15804A2
        } else if value.to_ascii_lowercase().contains("15804") {
            Standard::EN15804A1
        } else {
            Standard::UNKNOWN
        }
    }
}

#[derive(Deserialize, Serialize, JsonSchema, Clone)]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
pub enum SubType {
    Generic,
    Specific,
    Industry,
    Representative,
}

impl From<&Option<String>> for SubType {
    fn from(value: &Option<String>) -> Self {
        match value {
            Some(_value) if _value.to_lowercase().contains("representative") => {
                SubType::Representative
            }
            Some(_value) if _value.to_lowercase().contains("specific") => SubType::Specific,
            Some(_value) if _value.to_lowercase().contains("industry") => SubType::Industry,
            _ => SubType::Generic,
        }
    }
}

// mod my_date_format {
//     use chrono::{NaiveDate};
//     use serde::{self, Serializer, Deserializer, Deserialize};
//
//     const FORMAT: &'static str = "%Y-%m-%d";
//
//     pub fn serialize<S>(
//         date: &NaiveDate,
//         serializer: S,
//     ) -> Result<S::Ok, S::Error>
//         where
//             S: Serializer,
//     {
//         let s = format!("{}", date.format(FORMAT));
//         serializer.serialize_str(&s)
//     }
//
//     pub fn deserialize<'de, D>(
//         deserializer: D,
//     ) -> Result<NaiveDate, D::Error>
//         where
//             D: Deserializer<'de>,
//     {
//         let s = String::deserialize(deserializer)?;
//         let dt = NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
//         Ok(dt)
//     }
// }
