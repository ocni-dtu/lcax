import { calculateAssembly, CalculationOptions, LifeCycleModule, ImpactCategoryKey } from 'lcax'
import fs from 'fs'

const assembly_data = fs.readFileSync('assembly.json')
const assembly = JSON.parse(assembly_data)

const options: CalculationOptions = {
  referenceStudyPeriod: 50,
  lifeCycleModules: ['a1a3', 'a4'] as LifeCycleModule[],
  impactCategories: ['gwp'] as ImpactCategoryKey[],
  overwriteExistingResults: true,
}

const impacts = calculateAssembly(assembly, options)
