from pathlib import Path
from lcax import convert_ilcd, ImpactCategoryKey, LifeCycleModule

ilcd_file = Path("ilcd.json")

epd = convert_ilcd(ilcd_file.read_text())

gwp_a1a3 = epd.impacts[ImpactCategoryKey.GWP][LifeCycleModule.A1A3]
print(gwp_a1a3)
# 1.91881