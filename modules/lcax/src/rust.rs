use lcax_convert::{ilcd, lcabyg, slice};
use lcax_models::epd::EPD;
use lcax_models::project::Project;

#[cfg(feature = "default")]
pub fn convert_lcabyg(data: String, result_data: Option<String>) -> Result<Project, String> {
    match lcabyg::parse::parse_lcabyg(&data, result_data.as_deref()) {
        Ok(project) => Ok(project),
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

#[cfg(feature = "default")]
pub fn convert_slice(file: Vec<u8>) -> Result<Vec<Project>, String> {
    match slice::parse::parse_slice(file) {
        Ok(projects) => Ok(projects),
        Err(_) => panic!("Error parsing SLiCE data"),
    }
}
