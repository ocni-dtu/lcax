use schemars::schema_for;
use lcax_validation::model::ValidationSchema;

/// Generate a JSON Schema for validation schema.
fn main() {
    let validation_schema = serde_json::to_string_pretty(&schema_for!(ValidationSchema)).unwrap();

    println!("{}", validation_schema);
}
