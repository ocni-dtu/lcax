import {convertBRStandard} from "lcax";
import fs from 'fs'

const xlsxFile = new Uint8Array(fs.readFileSync('realtimelca_export.xlsx'))
const project = convertBRStandard("Real-Time LCA Project", xlsxFile)

// Output the LCAx formatted project to the console
console.log('LCAx', project)