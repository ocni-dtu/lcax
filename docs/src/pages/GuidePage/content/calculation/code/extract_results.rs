use lcax_calculation::results::{get_impact_total, get_impacts_by_life_cycle_module, normalize_result};
use lcax_models::life_cycle_base::ImpactCategoryKey;
use lcax_models::project::Project;
use std::fs;

fn main() {
    let project = serde_json::from_str::<Project>(
        &fs::read_to_string("project_with_results.json").unwrap()
    ).unwrap();

    let impacts = project.results.as_ref().unwrap();

    // Get total GWP impact for the project
    let total_gwp = get_impact_total(impacts, &ImpactCategoryKey::Gwp, &None);

    // Normalize the total GWP by gross floor area (e.g., 100 m2) and reference study period (e.g., 50 years)
    // result = impact / (gfa * rsp)
    let gfa = project.project_info.gross_floor_area.value;
    let rsp = project.project_info.reference_study_period;
    let normalized_gwp = normalize_result(&total_gwp, &(gfa * rsp as f64));

    // Get impacts by life cycle module for GWP
    let gwp_by_module = get_impacts_by_life_cycle_module(impacts, &ImpactCategoryKey::Gwp, &None, None);
}
