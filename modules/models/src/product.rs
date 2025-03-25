use chrono::NaiveDate;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cfg(feature = "jsbindings")]
use tsify::Tsify;

use crate::epd::{EPDReference, Standard, SubType};
use crate::generic_impact_data::GenericDataReference;
use crate::life_cycle_base::{Impacts, LifeCycleModule};
use crate::shared::{Conversion, MetaData, Reference, Source, Unit};
use lcax_core::country::Country;
#[cfg(feature = "pybindings")]
use pyo3::prelude::*;

#[derive(Deserialize, Serialize, JsonSchema, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
#[cfg_attr(feature = "pybindings", pyclass(get_all, set_all))]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub reference_service_life: u32,
    pub impact_data: Vec<ImpactData>,
    pub quantity: f64,
    pub unit: Unit,
    pub transport: Option<Vec<Transport>>,
    pub results: Option<Impacts>,
    pub meta_data: Option<MetaData>,
}

#[cfg_attr(feature = "pybindings", pymethods)]
impl Product {
    #[cfg(feature = "pybindings")]
    #[new]
    #[pyo3(signature=(name, reference_service_life, impact_data, quantity, unit, id=None, description=None, transport=None, results=None, meta_data=None ))]
    pub fn new_py(
        name: &str,
        reference_service_life: u32,
        impact_data: Vec<ImpactData>,
        quantity: f64,
        unit: Unit,
        id: Option<String>,
        description: Option<String>,
        transport: Option<Vec<Transport>>,
        results: Option<Impacts>,
        meta_data: Option<MetaData>,
    ) -> Product {
        Product::new(
            id,
            name,
            description,
            reference_service_life,
            impact_data,
            quantity,
            unit,
            transport,
            results,
            meta_data,
        )
    }
}

impl Product {
    pub fn new(
        id: Option<String>,
        name: &str,
        description: Option<String>,
        reference_service_life: u32,
        impact_data: Vec<ImpactData>,
        quantity: f64,
        unit: Unit,
        transport: Option<Vec<Transport>>,
        results: Option<Impacts>,
        meta_data: Option<MetaData>,
    ) -> Self {
        let _id = id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        Self {
            id: _id,
            name: name.to_string(),
            description,
            reference_service_life,
            impact_data,
            quantity,
            unit,
            transport,
            results,
            meta_data,
        }
    }
}

#[derive(Deserialize, Serialize, JsonSchema, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
#[cfg_attr(feature = "pybindings", pyclass(eq))]
pub enum ProductReference {
    Product(Product),
    Reference(Reference),
}

impl ProductReference {
    pub fn resolve(&self) -> Result<&Product, String> {
        match self {
            ProductReference::Product(data) => Ok(data),
            _ => Err("Handling of references not implemented yet!".to_string()),
        }
    }

    pub fn resolve_mut(&mut self) -> Result<&mut Product, String> {
        match self {
            ProductReference::Product(data) => Ok(data),
            _ => Err("Handling of references not implemented yet!".to_string()),
        }
    }
}

#[derive(Deserialize, Serialize, JsonSchema, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
#[cfg_attr(feature = "pybindings", pyclass(get_all, set_all))]
pub struct Transport {
    pub id: String,
    pub name: String,
    pub life_cycle_modules: Vec<LifeCycleModule>,
    pub distance: f64,
    pub distance_unit: Unit,
    pub impact_data: ImpactData,
}

#[derive(Deserialize, Serialize, JsonSchema, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
#[cfg_attr(feature = "pybindings", pyclass(eq))]
pub enum ImpactData {
    EPD(EPDReference),
    GenericData(GenericDataReference),
}

impl Default for ImpactData {
    fn default() -> ImpactData {
        ImpactData::GenericData(GenericDataReference::default())
    }
}

#[cfg_attr(feature = "pybindings", pymethods)]
impl ImpactData {
    #[cfg(feature = "pybindings")]
    #[new]
    #[pyo3(signature=(_type, name, declared_unit, version, published_date, valid_until, standard, location, subtype, impacts, id=None, format_version=None, source=None, reference_service_life=None, comment=None, conversions=None, meta_data=None))]
    pub fn new_py(
        _type: &str,
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
    ) -> ImpactData {
        ImpactData::new(
            _type,
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
        )
    }
}

impl ImpactData {
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
            "epd" => ImpactData::EPD(EPDReference::new(
                _type,
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
            "generic_data" => ImpactData::GenericData(GenericDataReference::new(
                _type,
                id,
                name,
                declared_unit,
                format_version,
                source,
                comment,
                conversions,
                impacts,
                meta_data,
            )),
            &_ => panic!("Unknown impact type!"),
        }
    }
}
