use std::fs;
use std::path::Path;

use lcax_convert::slice::parse::parse_slice;

#[test]
fn test_parse_slice() -> Result<(), String> {
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let file_path = root_dir.join("tests/slice/datafixtures/results_slice_WLCR.parquet");
    let file = fs::read(file_path).unwrap();

    let projects = parse_slice(file)?;

    for project in projects {
        assert!(!project.id.is_empty());
        assert!(!project.name.is_empty());

        // Assert Assembly Info
        assert!(!project.assemblies.is_empty());
        for (_, assembly) in &project.assemblies {
            assert!(!assembly.name.is_empty());
            assert!(!assembly.products.is_empty());

            for (_, product) in &assembly.products {
                // Assert Product Info
                assert!(!product.quantity.is_nan());
            }
        }
    }
    Ok(())
}
