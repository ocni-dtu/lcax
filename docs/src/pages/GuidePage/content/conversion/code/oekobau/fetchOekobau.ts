import { convertIlcd } from 'lcax'

// UUID of a process in ÖKOBAUDAT (copy it from the dataset page)
const processUuid = 'f63ac879-fa7d-4f91-813e-e816cbdf1927'
const url = `https://oekobaudat.de/OEKOBAU.DAT/resource/processes/${processUuid}?format=json&view=extended`

const response = await fetch(url)
const ilcdJson = await response.text()

const epd = convertIlcd(ilcdJson)
console.log(epd.name, epd.declaredUnit)
