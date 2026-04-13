use std::path::Path;
use lcax_convert::lcabyg;

fn main() {
    let file_path =  Path::new("realtimelca_export.xlsx");

    let project = br_standard_from_file(file_path);
}