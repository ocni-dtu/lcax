from lcax import calculate_assembly, CalculationOptions, LifeCycleModule, ImpactCategoryKey, Assembly
import json

with open("assembly.json") as f:
    assembly = Assembly(**json.load(f))

options = CalculationOptions(
    reference_study_period=50,
    life_cycle_modules=[LifeCycleModule.A1A3, LifeCycleModule.A4],
    impact_categories=[ImpactCategoryKey.Gwp],
    overwrite_existing_results=True
)

impacts = calculate_assembly(assembly, options)
