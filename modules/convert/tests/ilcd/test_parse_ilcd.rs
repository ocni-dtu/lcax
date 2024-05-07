
use std::fs;
use std::path::Path;
use lcax_convert::ilcd;
use lcax_models::epd::Standard;
use lcax_models::life_cycle_base::ImpactCategoryKey;

macro_rules! parse_ilcd_a1_tests {
    ($($name:ident: $value:expr)*) => {
    $(
        #[test]
        fn $name() -> Result<(), String> {
            let (file_name, standard) = $value;

            let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
            let file_path = root_dir.join("tests/ilcd/datafixtures").join(file_name);
            let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

            match ilcd::parse::parse_ilcd(&contents) {
                Ok(epd) => {
                    // Assert EPD Info
                    assert!(!epd.id.is_empty());
                    assert!(!epd.name.is_empty());
                    if standard == "a1" {
                        assert!(matches!(epd.standard, Standard::EN15804A1));
                    } else {
                        assert!(matches!(epd.standard, Standard::EN15804A2));
                    }

                    // Assert Impact Category Values
                    assert!(!epd.impacts.get(&ImpactCategoryKey::GWP).unwrap().is_empty());
                    assert!(!epd.impacts.get(&ImpactCategoryKey::ODP).unwrap().is_empty());
                    assert!(!epd.impacts.get(&ImpactCategoryKey::AP).unwrap().is_empty());
                    assert!(!epd.impacts.get(&ImpactCategoryKey::POCP).unwrap().is_empty());
                    assert!(!epd.impacts.get(&ImpactCategoryKey::ADPE).unwrap().is_empty());
                    assert!(!epd.impacts.get(&ImpactCategoryKey::ADPF).unwrap().is_empty());

                    if standard == "a2" {
                        assert!(!epd.impacts.get(&ImpactCategoryKey::EP_FW).unwrap().is_empty());
                        assert!(!epd.impacts.get(&ImpactCategoryKey::EP_MAR).unwrap().is_empty());
                        assert!(!epd.impacts.get(&ImpactCategoryKey::EP_TER).unwrap().is_empty());
                        assert!(!epd.impacts.get(&ImpactCategoryKey::GWP_FOS).unwrap().is_empty());
                        assert!(!epd.impacts.get(&ImpactCategoryKey::GWP_BIO).unwrap().is_empty());
                        assert!(!epd.impacts.get(&ImpactCategoryKey::GWP_LUL).unwrap().is_empty());
                        assert!(!epd.impacts.get(&ImpactCategoryKey::PM).unwrap().is_empty());
                        assert!(!epd.impacts.get(&ImpactCategoryKey::WDP).unwrap().is_empty());
                        assert!(!epd.impacts.get(&ImpactCategoryKey::IRP).unwrap().is_empty());
                        assert!(!epd.impacts.get(&ImpactCategoryKey::ETP_FW).unwrap().is_empty());
                        assert!(!epd.impacts.get(&ImpactCategoryKey::HTP_C).unwrap().is_empty());
                        assert!(!epd.impacts.get(&ImpactCategoryKey::HTP_NC).unwrap().is_empty());
                        assert!(!epd.impacts.get(&ImpactCategoryKey::SQP).unwrap().is_empty());
                    }

                    Ok(())
                }
                Err(error) => Err(error.to_string())
            }
        }
    )*
    }
}
parse_ilcd_a1_tests! {
    // A1
    ilcd_00c28f1f: ("00c28f1f-1d49-4c81-9208-138922a1dd6c.json", "a1")
    ilcd_0e80e6e7: ("0e80e6e7-6882-47be-8bd8-5cd869a746d9.json", "a1")
    ilcd_f63ac879: ("f63ac879-fa7d-4f91-813e-e816cbdf1927.json", "a1")
    ilcd_0b488798: ("0b488798-140f-4efa-96e2-55aa46ed129a.json", "a1")
    ilcd_0d1e4a59: ("0d1e4a59-4901-4973-a26f-1698f65a780f.json", "a1")
    ilcd_0b4c397d: ("0b4c397d-c7a1-4ceb-9718-184334f6364e.json", "a1")
    ilcd_0e0c4e37: ("0e0c4e37-b7e6-4a4f-b1c9-d36da0aa16f5.json", "a1")
    ilcd_023f3b97: ("023f3b97-976a-41c4-b0f1-5357b9dc5b3e.json", "a1")
    ilcd_c23b2987: ("c23b2987-776d-4d55-91c7-5f2a0f2c50f1.json", "a1")
    // A2
    ilcd_0e9fd868: ("0e9fd868-9656-489e-be6c-8251b3d43283.json", "a2")
    ilcd_0cb92770: ("0cb92770-9007-48c6-bc03-466af8894419.json", "a2")
    ilcd_0aa8b645: ("0aa8b645-02d9-41b4-8aa3-70335af2a9e7.json", "a2")
    ilcd_335241f9: ("335241f9-db84-486c-9a19-cd5ebb791903.json", "a2")
    ilcd_503dfca1: ("503dfca1-7c65-4179-9ffa-ebc6d8b48b7d.json", "a2")
}


#[test]
fn test_parse_ilcd_short() -> Result<(), String> {
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let file_path = root_dir.join("tests/ilcd/datafixtures/f63ac879_test.json");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    match ilcd::parse::parse_ilcd(&contents) {
        Ok(_) => Err(String::from("Did not fail")),
        Err(_) => Ok(()),
    }
}
