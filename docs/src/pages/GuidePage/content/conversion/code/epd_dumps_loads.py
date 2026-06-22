from pathlib import Path
from lcax import convert_ilcd, EPD

ilcd_file = Path("ilcd.json")

epd = convert_ilcd(ilcd_file.read_text())

epd_file = Path("epd.lcax.json")
epd_file.write_text(epd.dumps())

new_epd = EPD.loads(epd_file.read_text())
