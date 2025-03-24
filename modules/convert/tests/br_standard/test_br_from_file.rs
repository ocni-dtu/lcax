use std::fs;
use lcax_convert::br_standard::xlsx::{br_standard_from_file, read_br_standard_from_bytes, read_br_standard_from_file};
use std::path::Path;

#[test]
fn test_br_standard_from_file() -> Result<(), String> {
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let file_path = root_dir.join("tests/br_standard/datafixtures/Traditionelt_Etagehus.xlsx");

    let project = br_standard_from_file(file_path);
    assert!(project.is_ok());

    Ok(())
}

#[test]
fn test_read_br_standard_from_file() -> Result<(), String> {
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let file_path = root_dir.join("tests/br_standard/datafixtures/Traditionelt_Etagehus.xlsx");

    read_br_standard_from_file(&file_path).unwrap();

    Ok(())
}

#[test]
fn test_read_br_standard_from_bytes() -> Result<(), String> {
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let file_path = root_dir.join("tests/br_standard/datafixtures/Traditionelt_Etagehus.xlsx");
    let file = fs::read(file_path).unwrap();

    read_br_standard_from_bytes(file).unwrap();

    Ok(())
}