pub mod project;
pub mod lcabyg;
mod utils;
#[cfg(feature = "pybindings")]
mod python;

#[cfg(feature = "jsbindings")]
mod javascript;

#[cfg(feature = "default")]
pub fn convert_lcabyg(data: String, result_data: Option<String>) -> String {
    let project = lcabyg::parse::parse_lcabyg(&data, result_data.as_deref());
    match project {
        Ok(project) => serde_json::to_string(&project).unwrap(),
        Err(_) => String::from(""),
    }
}