import { convertLCAbyg } from 'lcax'
import fs from 'fs'

const lcabygData = fs.readFileSync('lcabyg_project.json')
const resultData = fs.readFileSync('lcabyg_project_results.json')
const project = convertLCAbyg(lcabygData, resultData)

// Output the LCAx formatted project to the console
console.log('LCAx', project)
