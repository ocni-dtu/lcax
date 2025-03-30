use lcax_convert::lcabyg;

fn main() {
    let file_path = Path::new("lcabyg_project.json");
    let project_content = fs::read_to_string(file_path).unwrap();
    let _file_path = Path::new("lcabyg_project_results.json");
    let result_content = fs::read_to_string(_file_path).unwrap();

    let project = lcabyg::parse::parse_lcabyg(
        &project_content,
        &result_content
    ).unwrap();
}