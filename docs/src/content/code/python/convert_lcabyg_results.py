from pathlib import Path
import lcax

lcabyg_file = Path("lcabyg_project.json")
results_file = Path("lcabyg_project_results.json")

project = lcax.convert_lcabyg(lcabyg_file.read_text(), results_file.read_text())

