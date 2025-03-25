use lcax_validation::model::{ValidationSchema};
use schemars::schema_for;

/// Generate a JSON Schema for validation schema.
fn main() {
    let validation_schema = serde_json::to_string_pretty(&schema_for!(Vec<ValidationSchema>)).unwrap();

    println!("{}", validation_schema);
}
