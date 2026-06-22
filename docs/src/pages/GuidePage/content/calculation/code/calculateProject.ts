import { calculateProject, CalculationOptions, LifeCycleModule, ImpactCategoryKey } from 'lcax'
import fs from 'fs'

const project_data = fs.readFileSync('project.json')
const project = JSON.parse(project_data)

const options: CalculationOptions = {
  referenceStudyPeriod: 50,
  lifeCycleModules: ['a1a3', 'a4', 'b4', 'c3', 'c4'] as LifeCycleModule[],
  impactCategories: ['gwp'] as ImpactCategoryKey[],
  overwriteExistingResults: true,
}

const calculatedProject = calculateProject(project, options)
