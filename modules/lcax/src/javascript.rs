extern crate console_error_panic_hook;
use wasm_bindgen::prelude::*;

use lcax_calculation::calculate::calculate_project;
use lcax_calculation::results::{
    get_impact_total, get_impacts_by_life_cycle_module, normalize_result,
};
use lcax_convert::br_standard::parse::parse_br_standard;
use lcax_convert::br_standard::xlsx::read_br_standard_from_bytes;
use lcax_convert::lcabyg::parse::LCABygResult;
use lcax_convert::{ilcd, lcabyg};
use lcax_models::epd::EPD;
use lcax_models::life_cycle_base::{ImpactCategory, ImpactCategoryKey, Impacts, LifeCycleModule};
use lcax_models::project::Project;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

/// Converts a json formatted LCAByg project into a LCAx Project
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn convertLCAbyg(data: String, resultData: Option<String>) -> Result<LCABygResult, JsError> {
    console_error_panic_hook::set_once();

    match lcabyg::parse::parse_lcabyg(&data, resultData.as_deref()) {
        Ok(result) => Ok(result),
        Err(error) => Err(JsError::new(error.to_string().as_str())),
    }
}

/// Converts a BR Standard Format file into a LCAx `Project`.
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn convertBRStandard(project_name: &str, file: Vec<u8>) -> Result<Project, JsError> {
    console_error_panic_hook::set_once();

    let (project_info, components, operations) = read_br_standard_from_bytes(file)?;
    match parse_br_standard(project_name, &project_info, &components, &operations) {
        Ok(project) => Ok(project),
        Err(error) => Err(JsError::new(error.as_str())),
    }
}

///Converts a json formatted ILCD+EPD data string into a LCAx EPD
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn convertIlcd(data: String) -> Result<EPD, JsError> {
    console_error_panic_hook::set_once();
    let epd = ilcd::parse::parse_ilcd(&data);
    match epd {
        Ok(epd) => Ok(epd),
        Err(error) => Err(JsError::new(error.to_string().as_str())),
    }
}

///Calculate the impact results for a Project.
///The impact results for the project will be added to the `results` property.
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn calculateProject(mut project: Project) -> Result<Project, JsError> {
    console_error_panic_hook::set_once();
    match calculate_project(&mut project, None) {
        Ok(project) => Ok(project.clone()),
        Err(error) => Err(JsError::new(error.to_string().as_str())),
    }
}

///Get the total impact
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn getImpactTotal(
    impacts: Impacts,
    category: ImpactCategoryKey,
    exclude_modules: Option<Vec<LifeCycleModule>>,
) -> Result<f64, JsError> {
    console_error_panic_hook::set_once();
    Ok(get_impact_total(&impacts, &category, &exclude_modules))
}

///Normalize a result with e.g. the reference study period and gross floor area
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn normalizeResult(result: f64, normalizing_factor: f64) -> Result<f64, JsError> {
    Ok(normalize_result(&result, &normalizing_factor))
}

///Get the impacts by life cycle module.
///The results can be normalized by a factor.
#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn getImpactsByLifeCycleModule(
    impacts: Impacts,
    category: ImpactCategoryKey,
    exclude_modules: Option<Vec<LifeCycleModule>>,
    normalizing_factor: Option<f64>,
) -> Result<Option<ImpactCategory>, JsError> {
    Ok(get_impacts_by_life_cycle_module(
        &impacts,
        &category,
        &exclude_modules,
        normalizing_factor,
    ))
}
