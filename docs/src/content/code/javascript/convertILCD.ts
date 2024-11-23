import {convertIlcd} from "lcax";
import fs from 'fs'

const epdData = fs.readFileSync(`./ilcd_epd.json`)
const epd = convertIlcd(epdData)

// Output the LCAx formatted EPD to the console
console.log('EPD', epd)
