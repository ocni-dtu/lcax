from pathlib import Path
import json
import lcax.pydantic

lcabyg_file = Path(__file__).parent.parent / "data" / "lcabyg_project.json"

print("LCAx project as dict")
epd_dict = lcax.convert_lcabyg(lcabyg_file.read_text())
print(json.dumps(epd_dict, indent=2))

print("\nLCAx project as Pydantic model")
epd_pydantic = lcax.convert_lcabyg(lcabyg_file.read_text(), as_type=lcax.LCAxProject)
print(epd_pydantic)

print("\nLCAx project as string")
epd_str = lcax.convert_lcabyg(lcabyg_file.read_text(), as_type=str)
print(epd_str)
