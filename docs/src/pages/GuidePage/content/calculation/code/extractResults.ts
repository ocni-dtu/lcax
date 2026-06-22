import { getImpactTotal, getImpactsByLifeCycleModule, normalizeResult, ImpactCategoryKey } from 'lcax'
import fs from 'fs'

const project_data = fs.readFileSync('project_with_results.json')
const project = JSON.parse(project_data)

// Get total GWP impact for the project
const totalGwp = getImpactTotal(project.results, 'gwp' as ImpactCategoryKey)

// Normalize the total GWP by gross floor area (e.g., 100 m2) and reference study period (e.g., 50 years)
// result = impact / (gfa * rsp)
const gfa = project.projectInfo.grossFloorArea.value
const rsp = project.projectInfo.referenceStudyPeriod
const normalizedGwp = normalizeResult(totalGwp, gfa * rsp)

// Get impacts by life cycle module for GWP
const gwpByModule = getImpactsByLifeCycleModule(project.results, 'gwp' as ImpactCategoryKey)
