from pathlib import Path
import lcax

epd = lcax.EPD.loads(Path("epd.json").read_text())

lcabyg_string = lcax.to_lcabyg(epds=[epd])
