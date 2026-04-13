use lcax_calculation::calculate::calculate_product;
use lcax_calculation::models::CalculationOptions;
use lcax_models::life_cycle_base::{ImpactCategoryKey, LifeCycleModule};
use lcax_models::product::Product;
use std::fs;

fn main() {
    let mut product = serde_json::from_str::<Product>(
        &fs::read_to_string("product.json").unwrap()
    ).unwrap();

    let options = CalculationOptions {
        reference_study_period: Some(50),
        life_cycle_modules: vec![LifeCycleModule::A1A3],
        impact_categories: vec![ImpactCategoryKey::Gwp],
        overwrite_existing_results: true,
    };

    let impacts = calculate_product(&mut product, &options).unwrap();
}
