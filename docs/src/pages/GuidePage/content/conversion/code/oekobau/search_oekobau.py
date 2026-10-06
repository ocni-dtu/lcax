from urllib.parse import urlencode
from urllib.request import urlopen
import json

import lcax

query = urlencode({"search": "true", "name": "concrete", "format": "json"})
url = f"https://oekobaudat.de/OEKOBAU.DAT/resource/processes?{query}"

with urlopen(url) as response:
    results = json.loads(response.read().decode("utf-8"))

# soda4LCA search results list process UUIDs under data[].uuid
first_uuid = results["data"][0]["uuid"]
process_url = (
    f"https://oekobaudat.de/OEKOBAU.DAT/resource/processes/{first_uuid}"
    "?format=json&view=extended"
)

with urlopen(process_url) as response:
    epd = lcax.convert_ilcd(response.read().decode("utf-8"))

print(epd.id, epd.name)
