from pathlib import Path
import lcax

br_file = Path("realtimelca_export.xlsx")
project = lcax.convert_br_standard(br_file)

