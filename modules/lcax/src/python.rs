use lcax_calculation::calculate::{calculate_project};
use lcax_convert::{ilcd, lcabyg};
use lcax_models::project::Project;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
use std::fs;

#[pyfunction]
pub fn _convert_lcabyg(data: String, result_data: Option<String>) -> PyResult<String> {
    let project = lcabyg::parse::parse_lcabyg(&data, result_data.as_deref());
    match project {
        Ok(project) => Ok(serde_json::to_string(&project).unwrap()),
        Err(error) => Err(PyTypeError::new_err(error.to_string())),
    }
}

#[pyfunction]
pub fn _convert_ilcd(data: String) -> PyResult<String> {
    let epd = ilcd::parse::parse_ilcd(&data);
    match epd {
        Ok(epd) => Ok(serde_json::to_string(&epd).unwrap()),
        Err(error) => Err(PyTypeError::new_err(error.to_string())),
    }
}

#[pyfunction]
pub fn _calculate_project(project: String) -> PyResult<String> {
    let mut _project: Project = serde_json::from_str(&project).unwrap();

    match calculate_project(&mut _project, None) {
        Ok(project) => Ok(serde_json::to_string(project).unwrap()),
        Err(error) => Err(PyTypeError::new_err(error.to_string())),
    }
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn lcax(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(_convert_lcabyg, m)?)?;
    m.add_function(wrap_pyfunction!(_convert_ilcd, m)?)?;
    m.add_function(wrap_pyfunction!(_calculate_project, m)?)?;
    Ok(())
}
