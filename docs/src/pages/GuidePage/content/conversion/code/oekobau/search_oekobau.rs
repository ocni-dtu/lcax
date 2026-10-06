use lcax_convert::ilcd;
use serde_json::Value;

fn main() {
    let url = "https://oekobaudat.de/OEKOBAU.DAT/resource/processes?search=true&name=concrete&format=json";

    let results: Value = reqwest::blocking::get(url).unwrap().json().unwrap();

    // soda4LCA search results list process UUIDs under data[].uuid
    let first_uuid = results["data"][0]["uuid"].as_str().unwrap();
    let process_url = format!(
        "https://oekobaudat.de/OEKOBAU.DAT/resource/processes/{first_uuid}?format=json&view=extended"
    );

    let ilcd_json = reqwest::blocking::get(process_url).unwrap().text().unwrap();
    let epd = ilcd::parse::parse_ilcd(&ilcd_json).unwrap();
    println!("{} {}", epd.id, epd.name);
}
