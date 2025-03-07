use lcax_convert::lcabyg::parse::LCABygResult;
use lcax_convert::{ilcd, lcabyg};
use lcax_models::epd::EPD;

#[cfg(feature = "default")]
pub fn convert_lcabyg(data: String, result_data: Option<String>) -> Result<LCABygResult, String> {
    match lcabyg::parse::parse_lcabyg(&data, result_data.as_deref()) {
        Ok(result) => Ok(result),
        Err(_) => panic!("Error parsing LCAbyg data"),
    }
}

#[cfg(feature = "default")]
pub fn convert_ilcd(data: String) -> Result<EPD, String> {
    match ilcd::parse::parse_ilcd(&data) {
        Ok(epd) => Ok(epd),
        Err(_) => panic!("Error parsing ILCD data"),
    }
}
