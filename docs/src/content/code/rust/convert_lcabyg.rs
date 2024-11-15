use lcax_convert::lcabyg;

fn main() {
    let file_path = Path::new("../data/lcabyg_project.json");
    let contents = fs::read_to_string(file_path).unwrap();

    let project = lcabyg::parse::parse_lcabyg(&contents, None).unwrap();
}