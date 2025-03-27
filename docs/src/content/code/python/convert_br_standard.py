import lcax
from pathlib import Path

br_file = Path("Traditionelt_Etagehus.xlsx")
project = lcax.convert_br_standard(br_file)
