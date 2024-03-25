import {convertLCAbyg} from "lcax";
import fs from 'fs'

const lcabygData = fs.readFileSync(`${__dirname}/../data/lcabyg_project.json`)
const project = convertLCAbyg(lcabygData)

// Output the LCAx formatted project to the console
console.log('LCAx', project)