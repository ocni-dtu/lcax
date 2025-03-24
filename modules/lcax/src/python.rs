use lcax_calculation::calculate::calculate_project as _calculate_project;
use lcax_convert::br_standard::xlsx::br_standard_from_file;
use lcax_convert::lcabyg::parse::LCABygResult;
use lcax_convert::lcabyg::serialize::to_lcabyg as _to_lcabyg;
use lcax_convert::{ilcd, lcabyg};
use lcax_core::country::Country;
use lcax_models::assembly::{Assembly, AssemblyReference, Classification};
use lcax_models::epd::{EPDReference, Standard, SubType, EPD};
use lcax_models::generic_impact_data::{GenericData, GenericDataReference};
use lcax_models::life_cycle_base::{ImpactCategoryKey, LifeCycleModule};
use lcax_models::product::{ImpactData, Product, ProductReference, Transport};
use lcax_models::project::{Location, Project, ProjectInfo, ProjectPhase, SoftwareInfo};
use lcax_models::shared::{Conversion, Reference, Source, Unit};
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use std::path::PathBuf;

#[pyfunction]
#[pyo3(signature = (data, result_data=None))]
pub fn convert_lcabyg(data: String, result_data: Option<String>) -> PyResult<LCABygResult> {
    let project = lcabyg::parse::parse_lcabyg(&data, result_data.as_deref());
    match project {
        Ok(project) => Ok(project),
        Err(error) => Err(PyTypeError::new_err(error.to_string())),
    }
}

#[pyfunction]
pub fn convert_ilcd(data: String) -> PyResult<EPD> {
    let epd = ilcd::parse::parse_ilcd(&data);
    match epd {
        Ok(epd) => Ok(epd),
        Err(error) => Err(PyTypeError::new_err(error.to_string())),
    }
}

#[pyfunction]
pub fn convert_br_standard(file_path: PathBuf) -> PyResult<Project> {
    // match br_standard_from_file(PathBuf::from(file_path)) {
    match br_standard_from_file(file_path) {
        Ok(project) => Ok(project),
        Err(error) => Err(PyTypeError::new_err(error.to_string())),
    }
}

#[pyfunction]
pub fn calculate_project(project: &mut Project) -> PyResult<Project> {
    match _calculate_project(project, None) {
        Ok(project) => Ok(project.clone()),
        Err(error) => Err(PyTypeError::new_err(error.to_string())),
    }
}

#[pyfunction]
pub fn to_lcabyg(objects: &LCABygResult) -> PyResult<String> {
    match _to_lcabyg(objects) {
        Ok(result) => Ok(result),
        Err(error) => Err(PyTypeError::new_err(error.to_string())),
    }
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn lcax(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Classes
    m.add_class::<EPD>()?;
    m.add_class::<LCABygResult>()?;
    m.add_class::<Conversion>()?;
    m.add_class::<Unit>()?;
    m.add_class::<Source>()?;
    m.add_class::<Standard>()?;
    m.add_class::<Country>()?;
    m.add_class::<SubType>()?;
    m.add_class::<ImpactCategoryKey>()?;
    m.add_class::<LifeCycleModule>()?;
    m.add_class::<Project>()?;
    m.add_class::<Location>()?;
    m.add_class::<ProjectInfo>()?;
    m.add_class::<ProjectPhase>()?;
    m.add_class::<SoftwareInfo>()?;
    m.add_class::<Reference>()?;
    m.add_class::<AssemblyReference>()?;
    m.add_class::<Assembly>()?;
    m.add_class::<Classification>()?;
    m.add_class::<ProductReference>()?;
    m.add_class::<Product>()?;
    m.add_class::<Transport>()?;
    m.add_class::<ImpactData>()?;
    m.add_class::<EPDReference>()?;
    m.add_class::<EPD>()?;
    m.add_class::<GenericDataReference>()?;
    m.add_class::<GenericData>()?;

    // Functions
    m.add_function(wrap_pyfunction!(convert_lcabyg, m)?)?;
    m.add_function(wrap_pyfunction!(convert_ilcd, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_project, m)?)?;
    m.add_function(wrap_pyfunction!(to_lcabyg, m)?)?;
    m.add_function(wrap_pyfunction!(convert_br_standard, m)?)?;
    Ok(())
}
