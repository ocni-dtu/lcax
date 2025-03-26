use chrono::NaiveDate;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cfg(feature = "jsbindings")]
use tsify::Tsify;

use crate::life_cycle_base::Impacts;
use crate::shared::{Conversion, MetaData, Reference, Source, Unit};
use lcax_core::country::Country;
use lcax_core::dates::{deserialize_yyyy_mm_dd, serialize_yyyy_mm_dd};
use lcax_core::utils::get_version;

#[cfg(feature = "pybindings")]
use pyo3::exceptions::PyTypeError;
#[cfg(feature = "pybindings")]
use pyo3::prelude::*;
#[cfg(feature = "pybindings")]
use pyo3::types::PyType;

#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "jsbindings",
    derive(Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
#[cfg_attr(feature = "pybindings", pyclass(get_all, set_all))]
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
    pub impacts: Impacts,
    pub meta_data: Option<MetaData>,
}

impl Default for EPD {
    fn default() -> Self {
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
            subtype: SubType::GENERIC,
            conversions: None,
            impacts: Impacts::new(),
            meta_data: None,
        }
    }
}

#[cfg_attr(feature = "pybindings", pymethods)]
impl EPD {
    #[cfg(feature = "pybindings")]
    #[new]
    #[pyo3(signature=(name, declared_unit, version, published_date, valid_until, standard, location, subtype, impacts, id=None, format_version=None, source=None, reference_service_life=None, comment=None, conversions=None, meta_data=None ))]
    pub fn new_py(
        name: String,
        declared_unit: Unit,
        version: String,
        published_date: NaiveDate,
        valid_until: NaiveDate,
        standard: Standard,
        location: Country,
        subtype: SubType,
        impacts: Impacts,
        id: Option<String>,
        format_version: Option<String>,
        source: Option<Source>,
        reference_service_life: Option<u32>,
        comment: Option<String>,
        conversions: Option<Vec<Conversion>>,
        meta_data: Option<MetaData>,
    ) -> Self {
        let _id = id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let _format_version = format_version.unwrap_or_else(|| get_version().to_string());
        Self {
            id: _id,
            name,
            declared_unit,
            version,
            published_date,
            valid_until,
            format_version: _format_version,
            source,
            reference_service_life,
            standard,
            comment,
            location,
            subtype,
            conversions,
            impacts,
            meta_data,
        }
    }

    #[cfg(feature = "pybindings")]
    #[classmethod]
    #[pyo3(name = "loads")]
    pub fn loads_py(_cls: &Bound<'_, PyType>, value: &str) -> PyResult<Self> {
        match EPD::loads(value) {
            Ok(epd) => Ok(epd),
            Err(error) => Err(PyTypeError::new_err(error.to_string())),
        }
    }

    #[cfg(feature = "pybindings")]
    #[pyo3(name = "dumps")]
    pub fn dumps_py(&self) -> PyResult<String> {
        match EPD::dumps(self) {
            Ok(data) => Ok(data),
            Err(error) => Err(PyTypeError::new_err(error.to_string())),
        }
    }
}

impl EPD {
    pub fn new(
        id: Option<String>,
        name: String,
        declared_unit: Unit,
        version: String,
        published_date: NaiveDate,
        valid_until: NaiveDate,
        format_version: Option<String>,
        source: Option<Source>,
        reference_service_life: Option<u32>,
        standard: Standard,
        comment: Option<String>,
        location: Country,
        subtype: SubType,
        conversions: Option<Vec<Conversion>>,
        impacts: Impacts,
        meta_data: Option<MetaData>,
    ) -> Self {
        let _id = id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let _format_version = format_version.unwrap_or_else(|| get_version().to_string());
        Self {
            id: _id,
            name,
            declared_unit,
            version,
            published_date,
            valid_until,
            format_version: _format_version,
            source,
            reference_service_life,
            standard,
            comment,
            location,
            subtype,
            conversions,
            impacts,
            meta_data,
        }
    }

    pub fn loads(value: &str) -> Result<EPD, serde_json::Error> {
        serde_json::from_str(value)
    }

    pub fn dumps(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

#[derive(Deserialize, Serialize, JsonSchema, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
#[cfg_attr(feature = "pybindings", pyclass(eq, eq_int))]
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

#[derive(Deserialize, Serialize, JsonSchema, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
#[cfg_attr(feature = "pybindings", pyclass(eq, eq_int))]
pub enum SubType {
    GENERIC,
    SPECIFIC,
    INDUSTRY,
    REPRESENTATIVE,
}

impl From<&Option<String>> for SubType {
    fn from(value: &Option<String>) -> Self {
        match value {
            Some(_value) if _value.to_lowercase().contains("representative") => {
                SubType::REPRESENTATIVE
            }
            Some(_value) if _value.to_lowercase().contains("specific") => SubType::SPECIFIC,
            Some(_value) if _value.to_lowercase().contains("industry") => SubType::INDUSTRY,
            _ => SubType::GENERIC,
        }
    }
}

impl Into<String> for SubType {
    fn into(self) -> String {
        match self {
            SubType::REPRESENTATIVE => "representative".to_string(),
            SubType::SPECIFIC => "specific".to_string(),
            SubType::INDUSTRY => "industry".to_string(),
            SubType::GENERIC => "generic".to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, JsonSchema, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
#[cfg_attr(feature = "pybindings", pyclass(eq))]
pub enum EPDReference {
    #[serde(rename = "EPD")]
    EPD(EPD),
    Reference(Reference),
}

impl EPDReference {
    pub fn resolve(&self) -> Result<&EPD, String> {
        match self {
            EPDReference::EPD(epd) => Ok(epd),
            _ => Err("Handling of references not implemented yet!".to_string()),
        }
    }

    pub fn new(
        _type: &str,
        id: Option<String>,
        name: String,
        declared_unit: Unit,
        version: String,
        published_date: NaiveDate,
        valid_until: NaiveDate,
        format_version: Option<String>,
        source: Option<Source>,
        reference_service_life: Option<u32>,
        standard: Standard,
        comment: Option<String>,
        location: Country,
        subtype: SubType,
        conversions: Option<Vec<Conversion>>,
        impacts: Impacts,
        meta_data: Option<MetaData>,
    ) -> Self {
        match _type {
            "epd" => EPDReference::EPD(EPD::new(
                id,
                name,
                declared_unit,
                version,
                published_date,
                valid_until,
                format_version,
                source,
                reference_service_life,
                standard,
                comment,
                location,
                subtype,
                conversions,
                impacts,
                meta_data,
            )),
            // "reference" => EPDReference::Reference(Reference::new()),
            &_ => panic!("Unknown impact type!"),
        }
    }
}
