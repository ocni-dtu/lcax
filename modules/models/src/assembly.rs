use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[cfg(feature = "jsbindings")]
use tsify::Tsify;

#[cfg(feature = "pybindings")]
use pyo3::prelude::*;

use crate::life_cycle_base::Impacts;
use crate::product::ProductReference;
use crate::shared::{MetaData, Reference, Unit};

#[derive(Deserialize, Serialize, JsonSchema, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
#[cfg_attr(feature = "pybindings", pyclass(get_all, set_all))]
pub struct Assembly {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub comment: Option<String>,
    pub quantity: f64,
    pub unit: Unit,
    pub classification: Option<Vec<Classification>>,
    pub products: Vec<ProductReference>,
    pub results: Option<Impacts>,
    pub meta_data: Option<MetaData>,
}

#[cfg_attr(feature = "pybindings", pymethods)]
impl Assembly {
    #[cfg(feature = "pybindings")]
    #[new]
    #[pyo3(signature=(name, quantity, unit, products, id=None, description=None, comment=None, classification=None, results=None, meta_data=None))]
    pub fn new_py(
        name: String,
        quantity: f64,
        unit: Unit,
        products: Vec<ProductReference>,
        id: Option<String>,
        description: Option<String>,
        comment: Option<String>,
        classification: Option<Vec<Classification>>,
        results: Option<Impacts>,
        meta_data: Option<MetaData>,
    ) -> Self {
        Self::new(
            id,
            name,
            description,
            comment,
            quantity,
            unit,
            classification,
            products,
            results,
            meta_data,
        )
    }
}

impl Assembly {
    pub fn new(
        id: Option<String>,
        name: String,
        description: Option<String>,
        comment: Option<String>,
        quantity: f64,
        unit: Unit,
        classification: Option<Vec<Classification>>,
        products: Vec<ProductReference>,
        results: Option<Impacts>,
        meta_data: Option<MetaData>,
    ) -> Self {
        let _id = id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        Self {
            id: _id,
            name,
            description,
            comment,
            quantity,
            unit,
            classification,
            products,
            results,
            meta_data,
        }
    }
}

#[derive(Deserialize, Serialize, JsonSchema, Clone, PartialEq)]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
#[cfg_attr(feature = "pybindings", pyclass(get_all, set_all))]
pub struct Classification {
    pub system: String,
    pub code: String,
    pub name: String,
}

#[cfg_attr(feature = "pybindings", pymethods)]
impl Classification {
    #[cfg(feature = "pybindings")]
    #[new]
    pub fn new_py(system: String, code: String, name: String) -> Self {
        Self::new(system, code, name)
    }
}

impl Classification {
    pub fn new(system: String, code: String, name: String) -> Self {
        Self { system, code, name }
    }
}

#[derive(Deserialize, Serialize, JsonSchema, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
#[cfg_attr(feature = "jsbindings", derive(Tsify))]
#[cfg_attr(feature = "pybindings", pyclass(eq))]
pub enum AssemblyReference {
    Assembly(Assembly),
    Reference(Reference),
}

#[cfg_attr(feature = "pybindings", pymethods)]
impl AssemblyReference {
    #[cfg(feature = "pybindings")]
    #[new]
    #[pyo3(signature=(_type, name, quantity, unit, products, id=None, description=None, comment=None, classification=None, results=None, meta_data=None))]
    pub fn new_py(
        _type: &str,
        name: String,
        quantity: f64,
        unit: Unit,
        products: Vec<ProductReference>,
        id: Option<String>,
        description: Option<String>,
        comment: Option<String>,
        classification: Option<Vec<Classification>>,
        results: Option<Impacts>,
        meta_data: Option<MetaData>,
    ) -> Self {
        Self::new(
            _type,
            id,
            name,
            description,
            comment,
            quantity,
            unit,
            classification,
            products,
            results,
            meta_data,
        )
    }
}

impl AssemblyReference {
    pub fn resolve(&self) -> Result<Assembly, String> {
        match self {
            AssemblyReference::Assembly(data) => Ok(data.clone()),
            _ => Err("Handling of references not implemented yet!".to_string()),
        }
    }

    pub fn resolve_mut(&mut self) -> Result<Assembly, String> {
        match self {
            AssemblyReference::Assembly(data) => Ok(data.clone()),
            _ => Err("Handling of references not implemented yet!".to_string()),
        }
    }

    pub fn new(
        _type: &str,
        id: Option<String>,
        name: String,
        description: Option<String>,
        comment: Option<String>,
        quantity: f64,
        unit: Unit,
        classification: Option<Vec<Classification>>,
        products: Vec<ProductReference>,
        results: Option<Impacts>,
        meta_data: Option<MetaData>,
    ) -> Self {
        match _type {
            "assembly" => AssemblyReference::Assembly(Assembly::new(
                id,
                name,
                description,
                comment,
                quantity,
                unit,
                classification,
                products,
                results,
                meta_data,
            )),
            &_ => panic!("Unknown impact type!"),
        }
    }
}
