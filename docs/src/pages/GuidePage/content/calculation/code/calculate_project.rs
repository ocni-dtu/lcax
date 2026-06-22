use lcax_calculation::calculate::calculate_project;
use lcax_calculation::models::CalculationOptions;
use lcax_models::life_cycle_base::{ImpactCategoryKey, LifeCycleModule};
use lcax_models::project::Project;
use std::fs;

fn main() {
    let mut project = serde_json::from_str::<Project>(
        &fs::read_to_string("project.json").unwrap()
    ).unwrap();

    let options = CalculationOptions {
        reference_study_period: Some(50),
        life_cycle_modules: vec![LifeCycleModule::A1A3, LifeCycleModule::A4, LifeCycleModule::B4, LifeCycleModule::C3, LifeCycleModule::C4],
        impact_categories: vec![ImpactCategoryKey::Gwp],
        overwrite_existing_results: true,
    };

    let result = calculate_project(&mut project, &options).unwrap();
}
