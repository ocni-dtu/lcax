
use std::fs;
use std::path::Path;
use lcax_convert::lcabyg;

#[test]
fn test_parse_lcabyg_project() -> Result<(), String> {
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let file_path = root_dir.join("tests/lcabyg/datafixtures/lcabyg_project.json");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    match lcabyg::parse::parse_lcabyg(&contents, None) {
        Ok(lca) => Ok({
            // Assert LCA Info
            assert!(!lca.id.is_empty());
            assert!(!lca.name.is_empty());

            // Assert Assembly Info
            assert!(!lca.assemblies.is_empty());
            for (_, assembly) in &lca.assemblies {
                assert!(!assembly.name.is_empty());
                assert!(!assembly.products.is_empty());

                for (_, product) in &assembly.products {
                    // Assert Product Info
                    assert!(!product.name.is_empty());
                    assert!(!product.quantity.is_nan());
                }
            }
            ()
        }),
        Err(error) => Err(error.to_string()),
    }
}

#[test]
fn test_parse_lcabyg_example() -> Result<(), String> {
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    let file_path = root_dir.join("tests/lcabyg/datafixtures/lcabyg_example_project.json");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let result_path = root_dir.join("tests/lcabyg/datafixtures/lcabyg_example_results.json");
    let result_content =
        fs::read_to_string(result_path).expect("Should have been able to read the file");

    match lcabyg::parse::parse_lcabyg(&contents, Some(&result_content)) {
        Ok(lca) => Ok({
            // Assert LCA Info
            assert!(!lca.id.is_empty());
            assert!(lca.results.is_some());

            // Assert Assembly Info
            assert!(!lca.assemblies.is_empty());
            for (_, assembly) in &lca.assemblies {
                assert!(assembly.results.is_some());
            }
            ()
        }),
        Err(error) => Err(error.to_string()),
    }
}
