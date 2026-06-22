from lcax import calculate_project, CalculationOptions, LifeCycleModule, ImpactCategoryKey, Project
from pathlib import Path

project = Project.loads(Path("project.json").read_text())

options = CalculationOptions(
    reference_study_period=50,
    life_cycle_modules=[LifeCycleModule.A1A3, LifeCycleModule.A4, LifeCycleModule.B4, LifeCycleModule.C3, LifeCycleModule.C4],
    impact_categories=[ImpactCategoryKey.Gwp],
    overwrite_existing_results=True
)

# The results are added to the project object
project = calculate_project(project, options)
