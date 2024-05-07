use lcax_convert::{lcabyg, ilcd};
use lcax_models::project::Project;
use lcax_models::epd::EPD;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn convertLCAbyg(data: String, resultData: Option<String>) -> Result<Project, JsError> {
    let project = lcabyg::parse::parse_lcabyg(&data, resultData.as_deref());
    match project {
        Ok(project) => Ok(project),
        Err(error) => Err(JsError::new(error.to_string().as_str())),
    }
}

#[allow(non_snake_case)]
#[wasm_bindgen]
pub fn convertIlcd(data: String) -> Result<EPD, JsError> {
    let epd = ilcd::parse::parse_ilcd(&data);
    match epd {
        Ok(epd) => Ok(epd),
        Err(error) => Err(JsError::new(error.to_string().as_str())),
    }
}
