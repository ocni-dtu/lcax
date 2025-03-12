use std::path::Path;
use lcax_convert::br_standard::xlsx::read_br_standard_from_file;

// #[test]
// fn test_br_standard_from_file() -> Result<(), String> {
//     let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
//     let file_path = root_dir.join("tests/br_standard/datafixtures/Traditionelt_Etagehus.xlsx");
//
//     let project = br_standard::br_standard_from_file(file_path);
//     assert!(project);
//
//     Ok(())
// }

#[test]
fn test_read_br_standard_from_file() -> Result<(), String> {
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let file_path = root_dir.join("tests/br_standard/datafixtures/Traditionelt_Etagehus.xlsx");

    read_br_standard_from_file(file_path).unwrap();

    Ok(())
}
