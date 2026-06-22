from lcax import calculate_product, CalculationOptions, LifeCycleModule, ImpactCategoryKey, Product
import json

with open("product.json") as f:
    product = Product(**json.load(f))

options = CalculationOptions(
    reference_study_period=50,
    life_cycle_modules=[LifeCycleModule.A1A3],
    impact_categories=[ImpactCategoryKey.Gwp],
    overwrite_existing_results=True
)

impacts = calculate_product(product, options)
