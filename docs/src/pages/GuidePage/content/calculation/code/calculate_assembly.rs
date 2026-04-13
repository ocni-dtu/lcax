use lcax_calculation::calculate::calculate_assembly;
use lcax_calculation::models::CalculationOptions;
use lcax_models::assembly::Assembly;
use lcax_models::life_cycle_base::{ImpactCategoryKey, LifeCycleModule};
use std::fs;

fn main() {
    let mut assembly = serde_json::from_str::<Assembly>(
        &fs::read_to_string("assembly.json").unwrap()
    ).unwrap();

    let options = CalculationOptions {
        reference_study_period: Some(50),
        life_cycle_modules: vec![LifeCycleModule::A1A3, LifeCycleModule::A4],
        impact_categories: vec![ImpactCategoryKey::Gwp],
        overwrite_existing_results: true,
    };

    let impacts = calculate_assembly(&mut assembly, &options).unwrap();
}
