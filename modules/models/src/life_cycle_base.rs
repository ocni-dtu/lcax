use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::{Entry, IterMut};
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;

#[cfg(feature = "pybindings")]
use pyo3::prelude::*;

#[cfg(feature = "pybindings")]
use pyo3::types::PyType;

#[cfg(feature = "jsbindings")]
use tsify_next::Tsify;

#[cfg(feature = "jsbindings")]
use wasm_bindgen::prelude::*;

#[derive(Deserialize, Serialize, JsonSchema, Hash, Eq, PartialEq, Clone, Debug)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(
    feature = "jsbindings",
    derive(Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
#[cfg_attr(feature = "pybindings", pyclass(eq, eq_int, frozen, hash))]
pub enum LifeCycleModule {
    A0,
    A1A3,
    A4,
    A5,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
    C1,
    C2,
    C3,
    C4,
    D,
}

impl fmt::Display for LifeCycleModule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LifeCycleModule::A0 => write!(f, "A0"),
            LifeCycleModule::A1A3 => write!(f, "A1A3"),
            LifeCycleModule::A4 => write!(f, "A4"),
            LifeCycleModule::A5 => write!(f, "A5"),
            LifeCycleModule::B1 => write!(f, "B1"),
            LifeCycleModule::B2 => write!(f, "B2"),
            LifeCycleModule::B3 => write!(f, "B3"),
            LifeCycleModule::B4 => write!(f, "B4"),
            LifeCycleModule::B5 => write!(f, "B5"),
            LifeCycleModule::B6 => write!(f, "B6"),
            LifeCycleModule::B7 => write!(f, "B7"),
            LifeCycleModule::B8 => write!(f, "B8"),
            LifeCycleModule::C1 => write!(f, "C1"),
            LifeCycleModule::C2 => write!(f, "C2"),
            LifeCycleModule::C3 => write!(f, "C3"),
            LifeCycleModule::C4 => write!(f, "C4"),
            LifeCycleModule::D => write!(f, "D"),
        }
    }
}

impl TryFrom<&str> for LifeCycleModule {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "a0" => Ok(LifeCycleModule::A0),
            "a1to3" => Ok(LifeCycleModule::A1A3),
            "a1a3" => Ok(LifeCycleModule::A1A3),
            "a1-3" => Ok(LifeCycleModule::A1A3),
            "a4" => Ok(LifeCycleModule::A4),
            "a5" => Ok(LifeCycleModule::A5),
            "b1" => Ok(LifeCycleModule::B1),
            "b2" => Ok(LifeCycleModule::B2),
            "b3" => Ok(LifeCycleModule::B3),
            "b4" => Ok(LifeCycleModule::B4),
            "b5" => Ok(LifeCycleModule::B5),
            "b6" => Ok(LifeCycleModule::B6),
            "b7" => Ok(LifeCycleModule::B7),
            "b8" => Ok(LifeCycleModule::B8),
            "c1" => Ok(LifeCycleModule::C1),
            "c2" => Ok(LifeCycleModule::C2),
            "c3" => Ok(LifeCycleModule::C3),
            "c4" => Ok(LifeCycleModule::C4),
            "d" => Ok(LifeCycleModule::D),
            value => Err(format!("Unknown lifecycle stage: '{}'", value)),
        }
    }
}

#[derive(Deserialize, Serialize, JsonSchema, Hash, Eq, PartialEq, Clone, Debug)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(
    feature = "jsbindings",
    derive(Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
#[cfg_attr(feature = "pybindings", pyclass(eq, eq_int, frozen, hash))]
pub enum ImpactCategoryKey {
    GWP,
    #[allow(non_camel_case_types)]
    GWP_FOS,
    #[allow(non_camel_case_types)]
    GWP_BIO,
    #[allow(non_camel_case_types)]
    GWP_LUL,
    ODP,
    AP,
    EP,
    #[allow(non_camel_case_types)]
    EP_FW,
    #[allow(non_camel_case_types)]
    EP_MAR,
    #[allow(non_camel_case_types)]
    EP_TER,
    POCP,
    ADPE,
    ADPF,
    PENRE,
    PERE,
    PERM,
    PERT,
    PENRT,
    PENRM,
    SM,
    PM,
    WDP,
    IRP,
    #[allow(non_camel_case_types)]
    ETP_FW,
    #[allow(non_camel_case_types)]
    HTP_C,
    #[allow(non_camel_case_types)]
    HTP_NC,
    SQP,
    RSF,
    NRSF,
    FW,
    HWD,
    NHWD,
    RWD,
    CRU,
    MRF,
    MER,
    EEE,
    EET,
}

impl fmt::Display for ImpactCategoryKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ImpactCategoryKey::GWP => write!(f, "GWP"),
            ImpactCategoryKey::ODP => write!(f, "ODP"),
            ImpactCategoryKey::AP => write!(f, "AP"),
            ImpactCategoryKey::EP => write!(f, "EP"),
            ImpactCategoryKey::POCP => write!(f, "POCP"),
            ImpactCategoryKey::ADPE => write!(f, "ADPE"),
            ImpactCategoryKey::ADPF => write!(f, "ADPF"),
            ImpactCategoryKey::PERE => write!(f, "PERE"),
            ImpactCategoryKey::PERM => write!(f, "PERM"),
            ImpactCategoryKey::PERT => write!(f, "PERT"),
            ImpactCategoryKey::PENRT => write!(f, "PENRT"),
            ImpactCategoryKey::PENRM => write!(f, "PENRM"),
            ImpactCategoryKey::SM => write!(f, "SM"),
            ImpactCategoryKey::RSF => write!(f, "RSF"),
            ImpactCategoryKey::NRSF => write!(f, "NRSF"),
            ImpactCategoryKey::FW => write!(f, "FW"),
            ImpactCategoryKey::HWD => write!(f, "HWD"),
            ImpactCategoryKey::NHWD => write!(f, "NHWD"),
            ImpactCategoryKey::RWD => write!(f, "RWD"),
            ImpactCategoryKey::CRU => write!(f, "CRU"),
            ImpactCategoryKey::MRF => write!(f, "MRF"),
            ImpactCategoryKey::MER => write!(f, "MER"),
            ImpactCategoryKey::EEE => write!(f, "EEE"),
            ImpactCategoryKey::EET => write!(f, "EET"),
            ImpactCategoryKey::PENRE => write!(f, "PENRE"),
            ImpactCategoryKey::GWP_FOS => write!(f, "GWP_FOS"),
            ImpactCategoryKey::GWP_BIO => write!(f, "GWP_BIO"),
            ImpactCategoryKey::GWP_LUL => write!(f, "GWP_LUL"),
            ImpactCategoryKey::EP_FW => write!(f, "EP_FW"),
            ImpactCategoryKey::EP_MAR => write!(f, "EP_MAR"),
            ImpactCategoryKey::EP_TER => write!(f, "EP_TER"),
            ImpactCategoryKey::PM => write!(f, "PM"),
            ImpactCategoryKey::WDP => write!(f, "WDP"),
            ImpactCategoryKey::IRP => write!(f, "IRP"),
            ImpactCategoryKey::ETP_FW => write!(f, "ETP_FW"),
            ImpactCategoryKey::HTP_C => write!(f, "HTP_C"),
            ImpactCategoryKey::HTP_NC => write!(f, "HTP_NC"),
            ImpactCategoryKey::SQP => write!(f, "SQP"),
        }
    }
}

#[derive(Deserialize, Serialize, JsonSchema, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "jsbindings",
    derive(Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
#[cfg_attr(feature = "pybindings", pyclass)]
pub struct ImpactCategory(HashMap<LifeCycleModule, Option<f64>>);

impl ImpactCategory {
    pub fn new() -> Self {
        ImpactCategory(HashMap::new())
    }
    pub fn insert(&mut self, key: LifeCycleModule, value: Option<f64>) -> Option<Option<f64>> {
        self.0.insert(key, value)
    }
    pub fn iter(&self) -> impl Iterator<Item = (&LifeCycleModule, &Option<f64>)> {
        self.0.iter()
    }
    pub fn iter_mut(&mut self) -> IterMut<LifeCycleModule, Option<f64>> {
        self.0.iter_mut()
    }

    pub fn get(&self, key: &LifeCycleModule) -> Option<&Option<f64>> {
        self.0.get(key)
    }

    pub fn get_mut(&mut self, key: &LifeCycleModule) -> Option<&mut Option<f64>> {
        self.0.get_mut(key)
    }
    pub fn remove(&mut self, key: &LifeCycleModule) -> Option<Option<f64>> {
        self.0.remove(key)
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<const N: usize> From<[(LifeCycleModule, Option<f64>); N]> for ImpactCategory {
    fn from(value: [(LifeCycleModule, Option<f64>); N]) -> Self {
        ImpactCategory(HashMap::from(value))
    }
}

#[cfg_attr(feature = "pybindings", pymethods)]
impl ImpactCategory {
    #[cfg(feature = "pybindings")]
    #[new]
    #[pyo3(signature = (value=None))]
    pub fn new_py(value: Option<HashMap<LifeCycleModule, Option<f64>>>) -> Self {
        let mut category = Self::new();
        if value.is_none() {
            return category;
        }
        category.0 = value.unwrap();
        category
    }

    #[cfg(feature = "pybindings")]
    #[classmethod]
    fn from_dict(_cls: &Bound<'_, PyType>, value: HashMap<LifeCycleModule, Option<f64>>) -> Self {
        let mut category = ImpactCategory::new();
        category.0 = value;
        category
    }

    #[cfg(feature = "pybindings")]
    fn dict(&self) -> HashMap<LifeCycleModule, Option<f64>> {
        self.0.clone()
    }
}

#[derive(Deserialize, Serialize, JsonSchema, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "jsbindings",
    derive(Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
#[cfg_attr(feature = "pybindings", pyclass)]
pub struct Impacts(HashMap<ImpactCategoryKey, ImpactCategory>);

impl Impacts {
    pub fn new() -> Self {
        Impacts(HashMap::new())
    }
    pub fn insert(
        &mut self,
        key: ImpactCategoryKey,
        value: ImpactCategory,
    ) -> Option<ImpactCategory> {
        self.0.insert(key, value)
    }
    pub fn iter(&self) -> impl Iterator<Item = (&ImpactCategoryKey, &ImpactCategory)> {
        self.0.iter()
    }
    pub fn iter_mut(&mut self) -> IterMut<ImpactCategoryKey, ImpactCategory> {
        self.0.iter_mut()
    }
    pub fn get(&self, key: &ImpactCategoryKey) -> Option<&ImpactCategory> {
        self.0.get(key)
    }
    pub fn entry(&mut self, key: ImpactCategoryKey) -> Entry<ImpactCategoryKey, ImpactCategory> {
        self.0.entry(key)
    }

    pub fn get_mut(&mut self, key: &ImpactCategoryKey) -> Option<&mut ImpactCategory> {
        self.0.get_mut(key)
    }
    pub fn remove(&mut self, key: &ImpactCategoryKey) -> Option<ImpactCategory> {
        self.0.remove(key)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<const N: usize> From<[(ImpactCategoryKey, ImpactCategory); N]> for Impacts {
    fn from(value: [(ImpactCategoryKey, ImpactCategory); N]) -> Self {
        Impacts(HashMap::from(value))
    }
}

#[cfg_attr(feature = "pybindings", pymethods)]
impl Impacts {
    #[cfg(feature = "pybindings")]
    #[new]
    #[pyo3(signature = (value=None))]
    pub fn new_py(value: Option<HashMap<ImpactCategoryKey, ImpactCategory>>) -> Self {
        let mut impacts = Self::new();
        if value.is_none() {
            return impacts;
        }
        impacts.0 = value.unwrap();
        impacts
    }

    #[cfg(feature = "pybindings")]
    #[classmethod]
    fn from_dict(
        _cls: &Bound<'_, PyType>,
        value: HashMap<ImpactCategoryKey, ImpactCategory>,
    ) -> Self {
        let mut impacts = Self::new();
        impacts.0 = value;
        impacts
    }
    #[cfg(feature = "pybindings")]
    fn dict(&self) -> HashMap<ImpactCategoryKey, ImpactCategory> {
        self.0.clone()
    }
}

pub trait NewResults {
    fn new_results(
        impact_categories: &Vec<ImpactCategoryKey>,
        life_cycle_stage: &Vec<LifeCycleModule>,
    ) -> Self;
}
impl NewResults for Impacts {
    fn new_results(
        impact_categories: &Vec<ImpactCategoryKey>,
        life_cycle_stage: &Vec<LifeCycleModule>,
    ) -> Self {
        let mut results = Impacts::new();
        impact_categories.iter().for_each(|impact_category_key| {
            let mut impact_category = ImpactCategory::new();
            life_cycle_stage.iter().for_each(|life_cycle_stage| {
                impact_category.insert(life_cycle_stage.clone(), Some(0.0));
            });
            results.insert(impact_category_key.clone(), impact_category);
        });
        results
    }
}
