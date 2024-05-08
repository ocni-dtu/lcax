use schemars::schema_for;

use lcax_models::project::Project;

/// Generate a JSON Schema for LCAx structs.
fn main() {
    let project_schema = serde_json::to_string_pretty(&schema_for!(Project)).unwrap();

    println!("{}", project_schema);
}
