from pathlib import Path
import lcax

ilcd_file = Path("ilcd.json")

epd = lcax.convert_ilcd(ilcd_file.read_text())
