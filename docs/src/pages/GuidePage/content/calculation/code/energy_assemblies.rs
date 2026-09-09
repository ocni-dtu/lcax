use lcax::get_energy_assemblies;
use lcax_calculation::calculate::calculate_project;
use lcax_calculation::models::CalculationOptions;
use lcax_core::value::AnyValue;
use lcax_models::assembly::AssemblyReference;
use lcax_models::life_cycle_base::{ImpactCategoryKey, LifeCycleModule};
use lcax_models::product::ProductReference;
use lcax_models::project::Project;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut project = serde_json::from_str::<Project>(
        &fs::read_to_string("project.json")?
    )?;

    // Retrieve default BR18 operational energy assemblies
    // Returns assemblies for Grid Electricity, District Heating, and Natural Gas
    let assemblies = get_energy_assemblies("BR18")
        .map_err(|e| e)?;

    let mut electricity = assemblies[0].clone();

    // Set the operational energy quantity (e.g. 50,000 kWh total lifetime electricity)
    if let ProductReference::Product(product) = &mut electricity.products[0] {
        product.quantity = 50_000.0;

        // Optional: define project start year in product metadata (defaults to first milestone)
        let meta = product.meta_data.get_or_insert_default();
        meta.insert(
            "startYear".to_string(),
            Some(AnyValue::Number(lcax_core::value::Number::Int(2025))),
        );
    }

    // Add the assembly to the project
    project.assemblies.push(AssemblyReference::Assembly(electricity));

    // Calculate operational energy impacts (Module B6)
    let options = CalculationOptions {
        reference_study_period: Some(50),
        life_cycle_modules: vec![LifeCycleModule::B6],
        impact_categories: vec![ImpactCategoryKey::Gwp],
        overwrite_existing_results: true,
    };

    calculate_project(&mut project, Some(&options))?;

    Ok(())
}
