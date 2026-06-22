import { calculateProduct, CalculationOptions, LifeCycleModule, ImpactCategoryKey } from 'lcax'
import fs from 'fs'

const product_data = fs.readFileSync('product.json')
const product = JSON.parse(product_data)

const options: CalculationOptions = {
  referenceStudyPeriod: 50,
  lifeCycleModules: ['a1a3'] as LifeCycleModule[],
  impactCategories: ['gwp'] as ImpactCategoryKey[],
  overwriteExistingResults: true,
}

const impacts = calculateProduct(product, options)
