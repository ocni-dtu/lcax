use lcax_convert::lcabyg;
use lcax_convert::lcabyg::parse::LCABygResult;
use std::fs;
use std::path::Path;

#[test]
fn test_parse_lcabyg_product() -> Result<(), String> {
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    let file_path = root_dir.join("tests/lcabyg/datafixtures/stages.json");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    match lcabyg::parse::parse_lcabyg(&contents, None) {
        Ok(product) => match product {
            LCABygResult::EPDs(epds) => Ok(assert!(!epds.is_empty())),
            _ => Err("Parse failed".to_string()),
        },
        Err(error) => Err(error.to_string()),
    }
}

#[test]
fn test_serialize_to_lcabyg_product() -> Result<(), String> {
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    let file_path = root_dir.join("tests/lcabyg/datafixtures/stages.json");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let product = lcabyg::parse::parse_lcabyg(&contents, None).unwrap();

    let product_string = lcabyg::serialize::to_lcabyg(&product).unwrap();
    assert!(!product_string.is_empty());

    Ok(())
}
