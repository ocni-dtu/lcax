use crate::lcabyg::parse;
use crate::project::LCAxProject;
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
pub fn convertLCAbyg(data: String, resultData: Option<String>) -> Result<LCAxProject, JsError> {
    let project = parse::parse_lcabyg(&data, resultData.as_deref());
    match project {
        Ok(project) => Ok(project),
        Err(error) => Err(JsError::new(error.to_string().as_str())),
    }
}
