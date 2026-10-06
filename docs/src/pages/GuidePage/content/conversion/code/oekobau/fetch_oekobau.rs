use lcax_convert::ilcd;

fn main() {
    // UUID of a process in ÖKOBAUDAT (copy it from the dataset page)
    let process_uuid = "f63ac879-fa7d-4f91-813e-e816cbdf1927";
    let url = format!(
        "https://oekobaudat.de/OEKOBAU.DAT/resource/processes/{process_uuid}?format=json&view=extended"
    );

    let ilcd_json = reqwest::blocking::get(url).unwrap().text().unwrap();
    let epd = ilcd::parse::parse_ilcd(&ilcd_json).unwrap();
    println!("{} {:?}", epd.name, epd.declared_unit);
}
