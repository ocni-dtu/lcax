import {
  getEnergyAssemblies,
  calculateProject,
  CalculationOptions,
  LifeCycleModule,
  ImpactCategoryKey,
  Project,
} from 'lcax'
import fs from 'fs'

const projectData = fs.readFileSync('project.json', 'utf8')
const project: Project = JSON.parse(projectData)

// Retrieve default BR18 operational energy assemblies
// Returns assemblies for Grid Electricity, District Heating, and Natural Gas
const assemblies = getEnergyAssemblies('BR18')

// Select electricity assembly (quantity is in kWh)
const electricity = assemblies[0]

// Set the operational energy quantity (e.g. 50,000 kWh total lifetime electricity)
electricity.products[0].quantity = 50000

// Optional: define project start year in product metadata (defaults to first milestone)
electricity.products[0].metaData = {
  ...electricity.products[0].metaData,
  startYear: 2025,
}

// Add the assembly to the project
project.assemblies.push(electricity)

// Calculate operational energy impacts (Module B6)
const options: CalculationOptions = {
  referenceStudyPeriod: 50,
  lifeCycleModules: ['b6'] as LifeCycleModule[],
  impactCategories: ['gwp'] as ImpactCategoryKey[],
  overwriteExistingResults: true,
}

const calculatedProject = calculateProject(project, options)
