use lcax_calculation::calculate::{
    calculate_assembly as _calculate_assembly, calculate_product as _calculate_product,
    calculate_project as _calculate_project,
};
use lcax_calculation::models::CalculationOptions;
use lcax_calculation::results;
use lcax_convert::br_standard::xlsx::br_standard_from_file;
use lcax_convert::lcabyg::parse::LCABygResult;
use lcax_convert::{ilcd, lcabyg};
use lcax_core::country::Country;
use lcax_models::assembly::{Assembly, Classification};
use lcax_models::epd::{Standard, SubType, EPD};
use lcax_models::generic_impact_data::GenericData;
use lcax_models::life_cycle_base::{ImpactCategory, ImpactCategoryKey, Impacts, LifeCycleModule};
use lcax_models::product::{Product, Transport};
use lcax_models::project::{BuildingInfo, Location, Project, ProjectPhase, SoftwareInfo};
use lcax_models::shared::{Conversion, Reference, Source, Unit};
use lcax_validation::model::{Level, ValidationResult, ValidationRule};
use lcax_validation::ValidationSchema;
use pyo3::exceptions::{PyTypeError, PyValueError};
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
pub fn calculate_assembly(
    assembly: &mut Assembly,
    options: &CalculationOptions,
) -> PyResult<Impacts> {
    match _calculate_assembly(assembly, options) {
        Ok(results) => Ok(results.clone()),
        Err(error) => Err(PyTypeError::new_err(error.to_string())),
    }
}

#[pyfunction]
pub fn calculate_product(product: &mut Product, options: &CalculationOptions) -> PyResult<Impacts> {
    match _calculate_product(product, options) {
        Ok(results) => Ok(results.clone()),
        Err(error) => Err(PyTypeError::new_err(error.to_string())),
    }
}

#[pyfunction]
#[pyo3(signature = (*, project=None, epds=None))]
pub fn to_lcabyg(project: Option<Project>, epds: Option<Vec<EPD>>) -> PyResult<String> {
    if project.is_some() {
        todo!("Project serialization is not implemented yet")
    } else if epds.is_some() {
        match lcabyg::serialize::serialize_epds(&epds.unwrap()) {
            Ok(epds) => Ok(epds),
            Err(error) => Err(PyValueError::new_err(error.to_string())),
        }
    } else {
        Err(PyValueError::new_err(
            "Either project or epds should be given".to_string(),
        ))
    }
}

#[pyfunction]
#[pyo3(signature = (impacts, category, exclude_modules=None))]
pub fn get_impact_total(
    impacts: Impacts,
    category: ImpactCategoryKey,
    exclude_modules: Option<Vec<LifeCycleModule>>,
) -> PyResult<f64> {
    Ok(results::get_impact_total(
        &impacts,
        &category,
        &exclude_modules,
    ))
}

#[pyfunction]
pub fn normalize_result(result: f64, normalizing_factor: f64) -> PyResult<f64> {
    Ok(results::normalize_result(&result, &normalizing_factor))
}

#[pyfunction]
#[pyo3(signature = (impacts, category, exclude_modules=None, normalizing_factor=None))]
pub fn get_impacts_by_life_cycle_module(
    impacts: Impacts,
    category: ImpactCategoryKey,
    exclude_modules: Option<Vec<LifeCycleModule>>,
    normalizing_factor: Option<f64>,
) -> PyResult<Option<ImpactCategory>> {
    Ok(results::get_impacts_by_life_cycle_module(
        &impacts,
        &category,
        &exclude_modules,
        normalizing_factor,
    ))
}

#[pyfunction]
pub fn validate(
    project: Project,
    validation_schema: Vec<ValidationSchema>,
) -> PyResult<Vec<ValidationResult>> {
    Ok(lcax_validation::validate(&project, &validation_schema))
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn lcax(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Classes
    m.add_class::<EPD>()?;
    m.add_class::<Conversion>()?;
    m.add_class::<Unit>()?;
    m.add_class::<Source>()?;
    m.add_class::<Standard>()?;
    m.add_class::<Country>()?;
    m.add_class::<BuildingInfo>()?;
    m.add_class::<SubType>()?;
    m.add_class::<Impacts>()?;
    m.add_class::<ImpactCategory>()?;
    m.add_class::<ImpactCategoryKey>()?;
    m.add_class::<LifeCycleModule>()?;
    m.add_class::<Project>()?;
    m.add_class::<Location>()?;
    m.add_class::<ProjectPhase>()?;
    m.add_class::<SoftwareInfo>()?;
    m.add_class::<Reference>()?;
    m.add_class::<Assembly>()?;
    m.add_class::<Classification>()?;
    m.add_class::<Product>()?;
    m.add_class::<Transport>()?;
    m.add_class::<EPD>()?;
    m.add_class::<GenericData>()?;
    m.add_class::<ValidationSchema>()?;
    m.add_class::<Level>()?;
    m.add_class::<ValidationRule>()?;
    m.add_class::<ValidationResult>()?;

    // Functions
    m.add_function(wrap_pyfunction!(convert_lcabyg, m)?)?;
    m.add_function(wrap_pyfunction!(convert_ilcd, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_project, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_assembly, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_product, m)?)?;
    m.add_function(wrap_pyfunction!(to_lcabyg, m)?)?;
    m.add_function(wrap_pyfunction!(convert_br_standard, m)?)?;
    m.add_function(wrap_pyfunction!(get_impact_total, m)?)?;
    m.add_function(wrap_pyfunction!(normalize_result, m)?)?;
    m.add_function(wrap_pyfunction!(get_impacts_by_life_cycle_module, m)?)?;
    m.add_function(wrap_pyfunction!(validate, m)?)?;
    Ok(())
}
