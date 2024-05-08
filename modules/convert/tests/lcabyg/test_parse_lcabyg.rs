use std::fs;
use std::path::Path;

use lcax_convert::lcabyg::nodes::{epd_from_lcabyg_stages, Stage, Node};
use lcax_convert::lcabyg::parse::NodesAndEdges;
use lcax_models::epd::Standard;
use lcax_models::life_cycle_base::ImpactCategoryKey;

macro_rules! parse_lcabyg_tests {
    ($($name:ident: $value:expr)*) => {
    $(
        #[test]
        fn $name() -> Result<(), String> {
            let input = $value;

            let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
            let file_path = root_dir
                .join("tests/lcabyg/datafixtures")
                .join(input);
            let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
            let nodes: Vec<NodesAndEdges> = serde_json::from_str(&contents).unwrap();

            let mut stages: Vec<Stage> = vec![];
            for node in nodes {
                match node {
                    NodesAndEdges::Node(Node::Stage(_node)) => stages.append(&mut vec![_node]),
                    _ => {}
                }
            }
            let _epd = epd_from_lcabyg_stages(&stages);

            // Assert EPD Info
            assert!(!_epd.id.is_empty());
            assert!(!_epd.name.is_empty());
            assert!(!matches!(_epd.standard, Standard::UNKNOWN));

            // Assert Impact Category Values
            assert!(!_epd.impacts.get(&ImpactCategoryKey::GWP).unwrap().is_empty());
            assert!(!_epd.impacts.get(&ImpactCategoryKey::ODP).unwrap().is_empty());
            assert!(!_epd.impacts.get(&ImpactCategoryKey::AP).unwrap().is_empty());
            assert!(!_epd.impacts.get(&ImpactCategoryKey::POCP).unwrap().is_empty());
            assert!(!_epd.impacts.get(&ImpactCategoryKey::ADPE).unwrap().is_empty());
            assert!(!_epd.impacts.get(&ImpactCategoryKey::ADPF).unwrap().is_empty());
            Ok(())
        }
    )*
    }
}
parse_lcabyg_tests! {
    lcabyg_5aa09d72: "5aa09d72.json"
}
