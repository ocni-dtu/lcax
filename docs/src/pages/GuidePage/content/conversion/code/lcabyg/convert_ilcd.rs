use lcax_convert::ilcd;

fn main() {
    let file_path = Path::new("../data/ilcd_epd.json");
    let contents = fs::read_to_string(file_path).unwrap();

    let epd = ilcd::parse::parse_ilcd(&contents).unwrap();
}