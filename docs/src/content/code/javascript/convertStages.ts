import {convertLCAbyg} from "lcax";
import fs from 'fs'

const lcabygData = fs.readFileSync('lcabyg_stages.json')
const epds = convertLCAbyg(lcabygData).epds

// Output the LCAx formatted EPDs to the console
console.log('LCAx', epds)