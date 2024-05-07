use std::fs::File;
use lcax_convert::{lcabyg, ilcd, slice};
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

#[cfg(feature = "pybindings")]
#[pyfunction]
pub fn _convert_lcabyg(data: String, result_data: Option<String>) -> PyResult<String> {
    let project = lcabyg::parse::parse_lcabyg(&data, result_data.as_deref());
    match project {
        Ok(project) => Ok(serde_json::to_string(&project).unwrap()),
        Err(error) => Err(PyTypeError::new_err(error.to_string())),
    }
}

#[cfg(feature = "pybindings")]
#[pyfunction]
pub fn _convert_ilcd(data: String) -> PyResult<String> {
    let epd = ilcd::parse::parse_ilcd(&data);
    match epd {
        Ok(epd) => Ok(serde_json::to_string(&epd).unwrap()),
        Err(error) => Err(PyTypeError::new_err(error.to_string())),
    }
}

#[cfg(feature = "pybindings")]
#[pyfunction]
pub fn _convert_slice(path: String) -> PyResult<Vec<String>> {
    let file = File::open(path).unwrap();
    match slice::parse::parse_slice(file) {
        Ok(projects) => Ok(
            projects.iter().map(|project| serde_json::to_string(&project).unwrap()).collect()
        ),
        Err(error) => Err(PyTypeError::new_err(error.to_string())),
    }
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[cfg(feature = "pybindings")]
#[pymodule]
fn lcax(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(_convert_lcabyg, m)?)?;
    m.add_function(wrap_pyfunction!(_convert_ilcd, m)?)?;
    m.add_function(wrap_pyfunction!(_convert_slice, m)?)?;
    Ok(())
}
