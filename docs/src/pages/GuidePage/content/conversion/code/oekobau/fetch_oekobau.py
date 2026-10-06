from urllib.request import urlopen

import lcax

# UUID of a process in ÖKOBAUDAT (copy it from the dataset page)
process_uuid = "f63ac879-fa7d-4f91-813e-e816cbdf1927"
url = f"https://oekobaudat.de/OEKOBAU.DAT/resource/processes/{process_uuid}?format=json&view=extended"

with urlopen(url) as response:
    ilcd_json = response.read().decode("utf-8")

epd = lcax.convert_ilcd(ilcd_json)
print(epd.name, epd.declared_unit)
