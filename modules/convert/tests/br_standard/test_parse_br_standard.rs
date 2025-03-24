use lcax_convert::br_standard::parse::parse_br_standard;
use lcax_convert::br_standard::xlsx::read_br_standard_from_file;
use lcax_models::assembly::AssemblyReference;
use lcax_models::product::ProductReference;
use std::path::Path;

#[test]
fn test_parse_br_standard() -> Result<(), String> {
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let file_path =
        root_dir.join("tests/br_standard/datafixtures/914a0b0b-4b7f-498f-9349-16cf0d644766.xlsx");
    //let file_path = root_dir.join("tests/br_standard/datafixtures/Traditionelt_Etagehus.xlsx");

    let (project_info, components, operations) = read_br_standard_from_file(&file_path).unwrap();
    let project = parse_br_standard(
        "Traditionelt_Etagehus",
        &project_info,
        &components,
        &operations,
    )?;

    assert!(!project.id.is_empty());
    assert!(!project.name.is_empty());
    assert!(!project.results.unwrap().is_empty());

    // Assert Assembly Info
    assert!(!project.assemblies.is_empty());
    for assembly in &project.assemblies {
        match assembly {
            AssemblyReference::Assembly(assembly) => {
                assert!(!assembly.name.is_empty());
                assert!(!assembly.products.is_empty());
                assert!(assembly.classification.is_some());

                for product in &assembly.products {
                    // Assert Product Info
                    match product {
                        ProductReference::Product(product) => {
                            assert!(!product.name.is_empty());
                            assert!(!product.quantity.is_nan());
                        }
                        ProductReference::Reference(_) => {
                            assert!(false);
                        }
                    }
                }
            }
            AssemblyReference::Reference(_) => {
                assert!(false);
            }
        }
    }
    Ok(())
}