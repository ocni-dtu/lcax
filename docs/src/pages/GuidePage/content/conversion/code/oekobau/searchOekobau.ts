import { convertIlcd } from 'lcax'

const query = new URLSearchParams({
  search: 'true',
  name: 'concrete',
  format: 'json',
})
const url = `https://oekobaudat.de/OEKOBAU.DAT/resource/processes?${query}`

const results = await (await fetch(url)).json()

// soda4LCA search results list process UUIDs under data[].uuid
const firstUuid = results.data[0].uuid
const processUrl =
  `https://oekobaudat.de/OEKOBAU.DAT/resource/processes/${firstUuid}` +
  '?format=json&view=extended'

const epd = convertIlcd(await (await fetch(processUrl)).text())
console.log(epd.id, epd.name)
