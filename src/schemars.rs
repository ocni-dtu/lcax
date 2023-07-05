use schemars::{schema_for};
extern crate lcax;

use lcax::project;

fn main() {
    let schema = schema_for!(project::LCAxProject);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}