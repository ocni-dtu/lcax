use lcax_convert::{lcabyg, ilcd};

#[cfg(feature = "default")]
pub fn convert_lcabyg(data: String, result_data: Option<String>) -> String {
    let project = lcabyg::parse::parse_lcabyg(&data, result_data.as_deref());
    match project {
        Ok(project) => serde_json::to_string(&project).unwrap(),
        Err(_) => String::from(""),
    }
}

#[cfg(feature = "default")]
pub fn convert_ilcd(data: String) -> String {
    let epd = ilcd::parse::parse_ilcd(&data);
    match epd {
        Ok(epd) => serde_json::to_string(&epd).unwrap(),
        Err(_) => String::from(""),
    }
}
