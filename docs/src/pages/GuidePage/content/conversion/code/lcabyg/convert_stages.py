from pathlib import Path
import lcax

lcabyg_file = Path("lcabyg_project.json")

project = lcax.convert_lcabyg(lcabyg_file.read_text())
