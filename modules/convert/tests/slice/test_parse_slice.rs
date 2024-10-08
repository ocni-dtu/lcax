use std::fs;
use std::path::Path;

use lcax_convert::slice::parse::parse_slice;
use lcax_models::shared::ReferenceSource;

#[test]
fn test_parse_slice() -> Result<(), String> {
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let file_path = root_dir.join("tests/slice/datafixtures/slice.parquet");
    let file = fs::read(file_path).unwrap();

    let projects = parse_slice(file)?;

    for project in projects {
        assert!(!project.id.is_empty());
        assert!(!project.name.is_empty());
        assert!(project.results.is_some());

        // Assert Assembly Info
        assert!(!project.assemblies.is_empty());
        for (_, assembly) in &project.assemblies {
            match assembly {
                ReferenceSource::Actual(assembly) => {
                    assert!(!assembly.name.is_empty());
                    assert!(!assembly.products.is_empty());

                    for (_, product) in &assembly.products {
                        // Assert Product Info
                        match product {
                            ReferenceSource::Actual(product) => {
                                assert!(!product.quantity.is_nan());
                            }
                            ReferenceSource::Reference(_) => {
                                assert!(false);
                            }
                        }
                    }
                }
                ReferenceSource::Reference(_) => {
                    assert!(false);
                }
            }
        }
    }
    Ok(())
}
