use crate::lcabyg::parse;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

#[cfg(feature = "pybindings")]
#[pyfunction]
pub fn _convert_lcabyg(data: String, result_data: Option<String>) -> PyResult<String> {
    let project = parse::parse_lcabyg(&data, result_data.as_deref());
    match project {
        Ok(project) => Ok(serde_json::to_string(&project).unwrap()),
        Err(error) => Err(PyTypeError::new_err(error.to_string())),
    }
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[cfg(feature = "pybindings")]
#[pymodule]
fn lcax(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(_convert_lcabyg, m)?)?;

    Ok(())
}
