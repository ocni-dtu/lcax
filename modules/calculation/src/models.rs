use lcax_models::life_cycle_base::{ImpactCategoryKey, LifeCycleModule};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cfg(feature = "jsbindings")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "jsbindings")]
use tsify_next::Tsify;

#[cfg(feature = "pybindings")]
use pyo3::prelude::*;

// #[cfg(feature = "pybindings")]
// use pyo3::types::PyType;

#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(
    feature = "jsbindings",
    derive(Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
#[cfg_attr(feature = "pybindings", pyclass(get_all, set_all))]
pub struct CalculationOptions {
    pub reference_study_period: Option<u8>,
    pub life_cycle_modules: Vec<LifeCycleModule>,
    pub impact_categories: Vec<ImpactCategoryKey>,
    pub overwrite_existing_results: bool,
}

#[cfg_attr(feature = "pybindings", pymethods)]
impl CalculationOptions {
    // #[cfg(feature = "pybindings")]
    // #[new]
    // #[pyo3(signature = (reference_study_period, life_cycle_modules, impact_categories, overwrite_existing_results=false))]
    // pub fn new_py(
    //     reference_study_period: Option<u8>,
    //     life_cycle_modules: Vec<LifeCycleModule>,
    //     impact_categories: Vec<ImpactCategoryKey>,
    //     overwrite_existing_results: bool
    // ) -> Self {
    //     Self { reference_study_period, life_cycle_modules, impact_categories, overwrite_existing_results }
    // }

    // #[cfg(feature = "pybindings")]
    // #[classmethod]
    // fn from_dict(
    //     _cls: &Bound<'_, PyType>,
    //     value: HashMap<String, ImpactCategory>,
    // ) -> Self {
    //     Self::new_py()
    // }
    // #[cfg(feature = "pybindings")]
    // fn dict(&self) -> HashMap<ImpactCategoryKey, ImpactCategory> {
    //     self.0.clone()
    // }

    #[cfg(feature = "pybindings")]
    fn __repr__(&self) -> String {
        "CalculationOptions".to_string()
    }

    #[cfg(feature = "pybindings")]
    fn __str__(&self) -> String {
        "CalculationOptions".to_string()
    }
}
