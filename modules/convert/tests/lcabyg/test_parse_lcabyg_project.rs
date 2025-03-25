use std::fs;
use std::path::Path;

use lcax_convert::lcabyg;
use lcax_convert::lcabyg::parse::LCABygResult;
use lcax_models::assembly::AssemblyReference;
use lcax_models::life_cycle_base::ImpactCategoryKey;
use lcax_models::product::ProductReference;

macro_rules! parse_lcabyg_project_tests {
    ($($name:ident: $value:expr)*) => {
    $(
        #[test]
        fn $name() -> Result<(), String> {
            let input = $value;

            let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
            let file_path = root_dir.join(format!("tests/lcabyg/datafixtures/{input}.json"));
            let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
            match lcabyg::parse::parse_lcabyg(&contents, None) {
                Ok(lca) => match lca {
                    LCABygResult::Project(project) => Ok({
                        // Assert LCA Info
                        assert!(!project.id.is_empty());
                        assert!(!project.name.is_empty());

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
                        ()
                    }),
                    _ => Err("Expected LCA Project".to_string()),
                },
                Err(error) => Err(error.to_string()),
            }
        }
    )*
    }
}
parse_lcabyg_project_tests! {
    project_test: "lcabyg_project"
}

macro_rules! parse_lcabyg_project_result_tests {
    ($($name:ident: $value:expr)*) => {
    $(
        #[test]
        fn $name() -> Result<(), String> {
            let input = $value;

            let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

            let file_path = root_dir.join(format!("tests/lcabyg/datafixtures/{input}.json"));
            let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

            let result_path = root_dir.join(format!("tests/lcabyg/datafixtures/{input}_results.json"));
            let result_content =
                fs::read_to_string(result_path).expect("Should have been able to read the file");
            match lcabyg::parse::parse_lcabyg(&contents, Some(&result_content)) {
                Ok(lca) => match lca {
                    LCABygResult::Project(project) => Ok({
                        // Assert LCA Info
                        assert!(!project.id.is_empty());
                        assert!(project.results.is_some());
                        assert!(project
                            .results
                            .unwrap()
                            .get(&ImpactCategoryKey::GWP)
                            .is_some());

                        // Assert Assembly Info
                        assert!(!project.assemblies.is_empty());
                        for assembly in &project.assemblies {
                            match assembly {
                                AssemblyReference::Assembly(assembly) => {
                                    assert!(assembly.results.is_some());
                                    assert!(assembly
                                        .results
                                        .clone()
                                        .unwrap()
                                        .get(&ImpactCategoryKey::GWP)
                                        .is_some());
                                }
                                AssemblyReference::Reference(_) => {
                                    assert!(false);
                                }
                            }
                        }
                        ()
                    }),
                    _ => Err("Expected LCA Project".to_string()),
                },
                Err(error) => Err(error.to_string()),
            }
        }
    )*
    }
}
parse_lcabyg_project_result_tests! {
    project_e9e6e798: "e9e6e798"
    project_2f95c41e: "2f95c41e"
    dtu_science_park: "dtu-science-park-kt-02"
    project_erlev_skole: "erlev-skole-sk-01"
    project_jysk_soedalsparken: "jysk-soedalsparken-kt-03"
    project_lisbjerg_bakke: "lisbjerg-bakke-eb-02"
}
