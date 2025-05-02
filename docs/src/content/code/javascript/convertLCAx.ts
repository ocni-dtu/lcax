import { toLCAbyg } from 'lcax'
import fs from 'fs'

const epdData = JSON.parse(fs.readFileSync('epd.json'))
const lcabygResult = toLCAbyg([epdData])
