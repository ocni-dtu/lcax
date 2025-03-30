import {convertLCAbyg} from "lcax";
import fs from 'fs'

const lcabygData = fs.readFileSync('lcabyg_project.json')
const project = convertLCAbyg(lcabygData).project

// Output the LCAx formatted project to the console
console.log('LCAx', project)