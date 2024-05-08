use tsify::Tsify;
use wasm_bindgen::prelude::*;

use lcax_convert::{ilcd, lcabyg, slice};
use lcax_models::epd::EPD;
use lcax_models::project::Project;
use serde::{Deserialize, Serialize};
extern crate console_error_panic_hook;
use std::panic;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn convertLCAbyg(data: String, resultData: Option<String>) -> Result<Project, JsError> {
    console_error_panic_hook::set_once();
    let project = lcabyg::parse::parse_lcabyg(&data, resultData.as_deref());
    match project {
        Ok(project) => Ok(project),
        Err(error) => Err(JsError::new(error.to_string().as_str())),
    }
}

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

#[derive(Deserialize, Serialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct JSProjects(Vec<Project>);

#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn convertSLiCE(file: Vec<u8>) -> Result<JSProjects, JsError> {
    console_error_panic_hook::set_once();
    match slice::parse::parse_slice(file) {
        Ok(projects) => Ok(JSProjects(projects)),
        Err(error) => Err(JsError::new(error.to_string().as_str())),
    }
}
