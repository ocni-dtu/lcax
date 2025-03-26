use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cfg(feature = "jsbindings")]
use tsify_next::Tsify;

#[cfg(feature = "pybindings")]
use pyo3::prelude::*;

use crate::life_cycle_base::Impacts;
use crate::shared::{Conversion, MetaData, Reference, Source, Unit};
use lcax_core::utils::get_version;

#[derive(Deserialize, Serialize, JsonSchema, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "jsbindings",
    derive(Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
#[cfg_attr(feature = "pybindings", pyclass(get_all, set_all))]
pub struct GenericData {
    pub id: String,
    pub name: String,
    pub declared_unit: Unit,
    pub format_version: String,
    pub source: Option<Source>,
    pub comment: Option<String>,
    pub conversions: Option<Vec<Conversion>>,
    pub impacts: Impacts,
    pub meta_data: Option<MetaData>,
}

impl Default for GenericData {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: "".to_string(),
            declared_unit: Unit::UNKNOWN,
            format_version: get_version(),
            source: None,
            comment: None,
            conversions: None,
            impacts: Impacts::new(),
            meta_data: None,
        }
    }
}

#[cfg_attr(feature = "pybindings", pymethods)]
impl GenericData {
    #[cfg(feature = "pybindings")]
    #[new]
    #[pyo3(signature=(name, declared_unit, impacts, id=None, format_version=None, source=None, comment=None, conversions=None, meta_data=None ))]
    pub fn new_py(
        name: String,
        declared_unit: Unit,
        impacts: Impacts,
        id: Option<String>,
        format_version: Option<String>,
        source: Option<Source>,
        comment: Option<String>,
        conversions: Option<Vec<Conversion>>,
        meta_data: Option<MetaData>,
    ) -> GenericData {
        GenericData::new(
            id,
            name,
            declared_unit,
            format_version,
            source,
            comment,
            conversions,
            impacts,
            meta_data,
        )
    }
}

impl GenericData {
    pub fn new(
        id: Option<String>,
        name: String,
        declared_unit: Unit,
        format_version: Option<String>,
        source: Option<Source>,
        comment: Option<String>,
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
            format_version: _format_version,
            source,
            comment,
            conversions,
            impacts,
            meta_data,
        }
    }
}
#[derive(Deserialize, Serialize, JsonSchema, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
#[cfg_attr(feature = "pybindings", pyclass(eq))]
pub enum GenericDataReference {
    #[serde(rename = "EPD")]
    GenericData(GenericData),
    Reference(Reference),
}

#[cfg_attr(feature = "pybindings", pymethods)]
impl GenericDataReference {
    #[cfg(feature = "pybindings")]
    #[new]
    #[pyo3(signature=(_type, name, declared_unit, impacts, id=None, format_version=None, source=None, comment=None, conversions=None, meta_data=None )
    )]
    pub fn new_py(
        _type: &str,
        name: String,
        declared_unit: Unit,
        impacts: Impacts,
        id: Option<String>,
        format_version: Option<String>,
        source: Option<Source>,
        comment: Option<String>,
        conversions: Option<Vec<Conversion>>,
        meta_data: Option<MetaData>,
    ) -> GenericDataReference {
        GenericDataReference::new(
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
        )
    }
}

impl GenericDataReference {
    pub fn resolve(&self) -> Result<GenericData, String> {
        match self {
            GenericDataReference::GenericData(data) => Ok(data.clone()),
            _ => Err("Handling of references not implemented yet!".to_string()),
        }
    }

    pub fn new(
        _type: &str,
        id: Option<String>,
        name: String,
        declared_unit: Unit,
        format_version: Option<String>,
        source: Option<Source>,
        comment: Option<String>,
        conversions: Option<Vec<Conversion>>,
        impacts: Impacts,
        meta_data: Option<MetaData>,
    ) -> Self {
        match _type {
            "generic_data" => GenericDataReference::GenericData(GenericData::new(
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
            // "reference" => GenericDataReference::Reference(Reference::new()),
            &_ => panic!("Unknown impact type!"),
        }
    }
}

impl Default for GenericDataReference {
    fn default() -> GenericDataReference {
        GenericDataReference::GenericData(Default::default())
    }
}
