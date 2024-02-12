
#[cfg(test)]
mod tests {
    use lcax::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_parse_lcabyg() -> Result<(), String> {
        let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let file_path = root_dir.join("tests/datafixtures/lcabyg_project.json");
        let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

        match lcabyg::parse::parse_lcabyg(contents, None) {
            Ok(epd) => Ok(()),
            Err(error) => Err(error.to_string())
        }
    }
}