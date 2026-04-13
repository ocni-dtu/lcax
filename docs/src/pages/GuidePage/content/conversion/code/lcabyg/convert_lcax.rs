use lcax_convert::lcabyg;

fn main() {
    let lcax_epd = serde_json::from_str::<LCAbygResult>(
        &fs::read_to_string("epd.json").unwrap()
    ).unwrap();

    let lcabyg_result = lcabyg::serialize::to_lcabyg(&lcax_epd);
}