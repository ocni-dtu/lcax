from lcax import (
    get_energy_assemblies,
    calculate_project,
    CalculationOptions,
    LifeCycleModule,
    ImpactCategoryKey,
    Project,
)
from pathlib import Path

# Load an existing project
project = Project.loads(Path("project.json").read_text())

# Retrieve default BR18 operational energy assemblies
# Returns assemblies for Grid Electricity, District Heating, and Natural Gas
assemblies = get_energy_assemblies("BR18")

# Select electricity assembly (quantity is in kWh)
electricity = assemblies[0]

# Set the operational energy quantity (e.g. 50,000 kWh total lifetime electricity)
electricity.products[0].quantity = 50_000.0

# Optional: define project start year in product metadata (defaults to first milestone)
electricity.products[0].meta_data["startYear"] = 2025

# Add the assembly to the project
project.assemblies.append(electricity)

# Calculate operational energy impacts (Module B6)
options = CalculationOptions(
    reference_study_period=50,
    life_cycle_modules=[LifeCycleModule.B6],
    impact_categories=[ImpactCategoryKey.Gwp],
    overwrite_existing_results=True,
)

project = calculate_project(project, options)
