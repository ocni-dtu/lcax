use lcax_convert::lcabyg;

fn main() {
    let file_path = Path::new("lcabyg_stages.json");
    let contents = fs::read_to_string(file_path).unwrap();

    let epds = lcabyg::parse::parse_lcabyg(&contents, None).unwrap();
}