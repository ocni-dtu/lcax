from lcax import get_impact_total, get_impacts_by_life_cycle_module, normalize_result, ImpactCategoryKey, Project
from pathlib import Path

project = Project.loads(Path("project_with_results.json").read_text())

# Get total GWP impact for the project
total_gwp = get_impact_total(project.results, ImpactCategoryKey.Gwp)

# Normalize the total GWP by gross floor area (e.g., 100 m2) and reference study period (e.g., 50 years)
# result = impact / (gfa * rsp)
gfa = project.project_info.gross_floor_area.value
rsp = project.project_info.reference_study_period
normalized_gwp = normalize_result(total_gwp, gfa * rsp)

# Get impacts by life cycle module for GWP
gwp_by_module = get_impacts_by_life_cycle_module(project.results, ImpactCategoryKey.Gwp)
