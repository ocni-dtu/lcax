from pathlib import Path
import json
import lcax.pydantic

ilcd_file = Path(__file__).parent.parent / "data" / "ilcd_epd.json"

epd_dict = lcax.convert_ilcd(ilcd_file.read_text())
print(json.dumps(epd_dict, indent=2))
