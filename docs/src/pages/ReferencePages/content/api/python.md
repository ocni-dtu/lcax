---
title: Python API Reference
description: LCAx Python API Reference
---

## `lcax`

### `lcax.lcax`

A Python module implemented in Rust. The name of this function must match
the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
import the module.

#### `lcax.lcax.Assembly`

```python
Assembly(name, quantity, unit, products, id=None, description=None, comment=None, classification=None, results=None, meta_data=None)
```

##### `lcax.lcax.Assembly.classification`

```python
classification: list[Classification] | None
```

##### `lcax.lcax.Assembly.comment`

```python
comment: str | None
```

##### `lcax.lcax.Assembly.description`

```python
description: str | None
```

##### `lcax.lcax.Assembly.id`

```python
id: str
```

##### `lcax.lcax.Assembly.meta_data`

```python
meta_data: MetaData | None
```

##### `lcax.lcax.Assembly.name`

```python
name: str
```

##### `lcax.lcax.Assembly.products`

```python
products: list[Product | Reference]
```

##### `lcax.lcax.Assembly.quantity`

```python
quantity: float
```

##### `lcax.lcax.Assembly.results`

```python
results: Impacts | None
```

##### `lcax.lcax.Assembly.unit`

```python
unit: Unit
```

#### `lcax.lcax.BuildingInfo`

##### `lcax.lcax.BuildingInfo.building_completion_year`

```python
building_completion_year: int | None
```

##### `lcax.lcax.BuildingInfo.building_footprint`

```python
building_footprint: ValueUnit | None
```

##### `lcax.lcax.BuildingInfo.building_height`

```python
building_height: ValueUnit | None
```

##### `lcax.lcax.BuildingInfo.building_mass`

```python
building_mass: ValueUnit | None
```

##### `lcax.lcax.BuildingInfo.building_model_scope`

```python
building_model_scope: list[BuildingModelScope] | None
```

##### `lcax.lcax.BuildingInfo.building_permit_year`

```python
building_permit_year: int | None
```

##### `lcax.lcax.BuildingInfo.building_type`

```python
building_type: BuildingType
```

##### `lcax.lcax.BuildingInfo.building_typology`

```python
building_typology: list[BuildingTypology]
```

##### `lcax.lcax.BuildingInfo.building_users`

```python
building_users: int | None
```

##### `lcax.lcax.BuildingInfo.certifications`

```python
certifications: list[str] | None
```

##### `lcax.lcax.BuildingInfo.energy_demand_electricity`

```python
energy_demand_electricity: float | None
```

##### `lcax.lcax.BuildingInfo.energy_demand_heating`

```python
energy_demand_heating: float | None
```

##### `lcax.lcax.BuildingInfo.energy_supply_electricity`

```python
energy_supply_electricity: float | None
```

##### `lcax.lcax.BuildingInfo.energy_supply_heating`

```python
energy_supply_heating: float | None
```

##### `lcax.lcax.BuildingInfo.exported_electricity`

```python
exported_electricity: float | None
```

##### `lcax.lcax.BuildingInfo.floors_above_ground`

```python
floors_above_ground: int
```

##### `lcax.lcax.BuildingInfo.floors_below_ground`

```python
floors_below_ground: int | None
```

##### `lcax.lcax.BuildingInfo.frame_type`

```python
frame_type: str | None
```

##### `lcax.lcax.BuildingInfo.general_energy_class`

```python
general_energy_class: GeneralEnergyClass
```

##### `lcax.lcax.BuildingInfo.gross_floor_area`

```python
gross_floor_area: AreaType | None
```

##### `lcax.lcax.BuildingInfo.heated_floor_area`

```python
heated_floor_area: AreaType | None
```

##### `lcax.lcax.BuildingInfo.local_energy_class`

```python
local_energy_class: str | None
```

##### `lcax.lcax.BuildingInfo.roof_type`

```python
roof_type: RoofType
```

#### `lcax.lcax.Classification`

```python
Classification(system, code, name)
```

##### `lcax.lcax.Classification.code`

```python
code: str
```

##### `lcax.lcax.Classification.name`

```python
name: str
```

##### `lcax.lcax.Classification.system`

```python
system: str
```

#### `lcax.lcax.Conversion`

```python
Conversion(value, to, meta_data=None)
```

##### `lcax.lcax.Conversion.meta_data`

```python
meta_data: MetaData | None
```

##### `lcax.lcax.Conversion.to`

```python
to: Unit
```

##### `lcax.lcax.Conversion.value`

```python
value: float
```

#### `lcax.lcax.Country`

Bases: <code>[Enum](#enum.Enum)</code>

##### `lcax.lcax.Country.ABW`

```python
ABW = ('ABW',)
```

##### `lcax.lcax.Country.AFG`

```python
AFG = ('AFG',)
```

##### `lcax.lcax.Country.AGO`

```python
AGO = ('AGO',)
```

##### `lcax.lcax.Country.AIA`

```python
AIA = ('AIA',)
```

##### `lcax.lcax.Country.ALA`

```python
ALA = ('ALA',)
```

##### `lcax.lcax.Country.ALB`

```python
ALB = ('ALB',)
```

##### `lcax.lcax.Country.AND`

```python
AND = ('AND',)
```

##### `lcax.lcax.Country.ARE`

```python
ARE = ('ARE',)
```

##### `lcax.lcax.Country.ARG`

```python
ARG = ('ARG',)
```

##### `lcax.lcax.Country.ARM`

```python
ARM = ('ARM',)
```

##### `lcax.lcax.Country.ASM`

```python
ASM = ('ASM',)
```

##### `lcax.lcax.Country.ATA`

```python
ATA = ('ATA',)
```

##### `lcax.lcax.Country.ATF`

```python
ATF = ('ATF',)
```

##### `lcax.lcax.Country.ATG`

```python
ATG = ('ATG',)
```

##### `lcax.lcax.Country.AUS`

```python
AUS = ('AUS',)
```

##### `lcax.lcax.Country.AUT`

```python
AUT = ('AUT',)
```

##### `lcax.lcax.Country.AZE`

```python
AZE = ('AZE',)
```

##### `lcax.lcax.Country.BDI`

```python
BDI = ('BDI',)
```

##### `lcax.lcax.Country.BEL`

```python
BEL = ('BEL',)
```

##### `lcax.lcax.Country.BEN`

```python
BEN = ('BEN',)
```

##### `lcax.lcax.Country.BES`

```python
BES = ('BES',)
```

##### `lcax.lcax.Country.BFA`

```python
BFA = ('BFA',)
```

##### `lcax.lcax.Country.BGD`

```python
BGD = ('BGD',)
```

##### `lcax.lcax.Country.BGR`

```python
BGR = ('BGR',)
```

##### `lcax.lcax.Country.BHR`

```python
BHR = ('BHR',)
```

##### `lcax.lcax.Country.BHS`

```python
BHS = ('BHS',)
```

##### `lcax.lcax.Country.BIH`

```python
BIH = ('BIH',)
```

##### `lcax.lcax.Country.BLM`

```python
BLM = ('BLM',)
```

##### `lcax.lcax.Country.BLR`

```python
BLR = ('BLR',)
```

##### `lcax.lcax.Country.BLZ`

```python
BLZ = ('BLZ',)
```

##### `lcax.lcax.Country.BMU`

```python
BMU = ('BMU',)
```

##### `lcax.lcax.Country.BOL`

```python
BOL = ('BOL',)
```

##### `lcax.lcax.Country.BRA`

```python
BRA = ('BRA',)
```

##### `lcax.lcax.Country.BRB`

```python
BRB = ('BRB',)
```

##### `lcax.lcax.Country.BRN`

```python
BRN = ('BRN',)
```

##### `lcax.lcax.Country.BTN`

```python
BTN = ('BTN',)
```

##### `lcax.lcax.Country.BVT`

```python
BVT = ('BVT',)
```

##### `lcax.lcax.Country.BWA`

```python
BWA = ('BWA',)
```

##### `lcax.lcax.Country.CAF`

```python
CAF = ('CAF',)
```

##### `lcax.lcax.Country.CAN`

```python
CAN = ('CAN',)
```

##### `lcax.lcax.Country.CCK`

```python
CCK = ('CCK',)
```

##### `lcax.lcax.Country.CHE`

```python
CHE = ('CHE',)
```

##### `lcax.lcax.Country.CHL`

```python
CHL = ('CHL',)
```

##### `lcax.lcax.Country.CHN`

```python
CHN = ('CHN',)
```

##### `lcax.lcax.Country.CIV`

```python
CIV = ('CIV',)
```

##### `lcax.lcax.Country.CMR`

```python
CMR = ('CMR',)
```

##### `lcax.lcax.Country.COD`

```python
COD = ('COD',)
```

##### `lcax.lcax.Country.COG`

```python
COG = ('COG',)
```

##### `lcax.lcax.Country.COK`

```python
COK = ('COK',)
```

##### `lcax.lcax.Country.COL`

```python
COL = ('COL',)
```

##### `lcax.lcax.Country.COM`

```python
COM = ('COM',)
```

##### `lcax.lcax.Country.CPV`

```python
CPV = ('CPV',)
```

##### `lcax.lcax.Country.CRI`

```python
CRI = ('CRI',)
```

##### `lcax.lcax.Country.CUB`

```python
CUB = ('CUB',)
```

##### `lcax.lcax.Country.CUW`

```python
CUW = ('CUW',)
```

##### `lcax.lcax.Country.CXR`

```python
CXR = ('CXR',)
```

##### `lcax.lcax.Country.CYM`

```python
CYM = ('CYM',)
```

##### `lcax.lcax.Country.CYP`

```python
CYP = ('CYP',)
```

##### `lcax.lcax.Country.CZE`

```python
CZE = ('CZE',)
```

##### `lcax.lcax.Country.DEU`

```python
DEU = ('DEU',)
```

##### `lcax.lcax.Country.DJI`

```python
DJI = ('DJI',)
```

##### `lcax.lcax.Country.DMA`

```python
DMA = ('DMA',)
```

##### `lcax.lcax.Country.DNK`

```python
DNK = ('DNK',)
```

##### `lcax.lcax.Country.DOM`

```python
DOM = ('DOM',)
```

##### `lcax.lcax.Country.DZA`

```python
DZA = ('DZA',)
```

##### `lcax.lcax.Country.ECU`

```python
ECU = ('ECU',)
```

##### `lcax.lcax.Country.EGY`

```python
EGY = ('EGY',)
```

##### `lcax.lcax.Country.ERI`

```python
ERI = ('ERI',)
```

##### `lcax.lcax.Country.ESH`

```python
ESH = ('ESH',)
```

##### `lcax.lcax.Country.ESP`

```python
ESP = ('ESP',)
```

##### `lcax.lcax.Country.EST`

```python
EST = ('EST',)
```

##### `lcax.lcax.Country.ETH`

```python
ETH = ('ETH',)
```

##### `lcax.lcax.Country.FIN`

```python
FIN = ('FIN',)
```

##### `lcax.lcax.Country.FJI`

```python
FJI = ('FJI',)
```

##### `lcax.lcax.Country.FLK`

```python
FLK = ('FLK',)
```

##### `lcax.lcax.Country.FRA`

```python
FRA = ('FRA',)
```

##### `lcax.lcax.Country.FRO`

```python
FRO = ('FRO',)
```

##### `lcax.lcax.Country.FSM`

```python
FSM = ('FSM',)
```

##### `lcax.lcax.Country.GAB`

```python
GAB = ('GAB',)
```

##### `lcax.lcax.Country.GBR`

```python
GBR = ('GBR',)
```

##### `lcax.lcax.Country.GEO`

```python
GEO = ('GEO',)
```

##### `lcax.lcax.Country.GGY`

```python
GGY = ('GGY',)
```

##### `lcax.lcax.Country.GHA`

```python
GHA = ('GHA',)
```

##### `lcax.lcax.Country.GIB`

```python
GIB = ('GIB',)
```

##### `lcax.lcax.Country.GIN`

```python
GIN = ('GIN',)
```

##### `lcax.lcax.Country.GLP`

```python
GLP = ('GLP',)
```

##### `lcax.lcax.Country.GMB`

```python
GMB = ('GMB',)
```

##### `lcax.lcax.Country.GNB`

```python
GNB = ('GNB',)
```

##### `lcax.lcax.Country.GNQ`

```python
GNQ = ('GNQ',)
```

##### `lcax.lcax.Country.GRC`

```python
GRC = ('GRC',)
```

##### `lcax.lcax.Country.GRD`

```python
GRD = ('GRD',)
```

##### `lcax.lcax.Country.GRL`

```python
GRL = ('GRL',)
```

##### `lcax.lcax.Country.GTM`

```python
GTM = ('GTM',)
```

##### `lcax.lcax.Country.GUF`

```python
GUF = ('GUF',)
```

##### `lcax.lcax.Country.GUM`

```python
GUM = ('GUM',)
```

##### `lcax.lcax.Country.GUY`

```python
GUY = ('GUY',)
```

##### `lcax.lcax.Country.HKG`

```python
HKG = ('HKG',)
```

##### `lcax.lcax.Country.HMD`

```python
HMD = ('HMD',)
```

##### `lcax.lcax.Country.HND`

```python
HND = ('HND',)
```

##### `lcax.lcax.Country.HRV`

```python
HRV = ('HRV',)
```

##### `lcax.lcax.Country.HTI`

```python
HTI = ('HTI',)
```

##### `lcax.lcax.Country.HUN`

```python
HUN = ('HUN',)
```

##### `lcax.lcax.Country.IDN`

```python
IDN = ('IDN',)
```

##### `lcax.lcax.Country.IMN`

```python
IMN = ('IMN',)
```

##### `lcax.lcax.Country.IND`

```python
IND = ('IND',)
```

##### `lcax.lcax.Country.IOT`

```python
IOT = ('IOT',)
```

##### `lcax.lcax.Country.IRL`

```python
IRL = ('IRL',)
```

##### `lcax.lcax.Country.IRN`

```python
IRN = ('IRN',)
```

##### `lcax.lcax.Country.IRQ`

```python
IRQ = ('IRQ',)
```

##### `lcax.lcax.Country.ISL`

```python
ISL = ('ISL',)
```

##### `lcax.lcax.Country.ISR`

```python
ISR = ('ISR',)
```

##### `lcax.lcax.Country.ITA`

```python
ITA = ('ITA',)
```

##### `lcax.lcax.Country.JAM`

```python
JAM = ('JAM',)
```

##### `lcax.lcax.Country.JEY`

```python
JEY = ('JEY',)
```

##### `lcax.lcax.Country.JOR`

```python
JOR = ('JOR',)
```

##### `lcax.lcax.Country.JPN`

```python
JPN = ('JPN',)
```

##### `lcax.lcax.Country.KAZ`

```python
KAZ = ('KAZ',)
```

##### `lcax.lcax.Country.KEN`

```python
KEN = ('KEN',)
```

##### `lcax.lcax.Country.KGZ`

```python
KGZ = ('KGZ',)
```

##### `lcax.lcax.Country.KHM`

```python
KHM = ('KHM',)
```

##### `lcax.lcax.Country.KIR`

```python
KIR = ('KIR',)
```

##### `lcax.lcax.Country.KNA`

```python
KNA = ('KNA',)
```

##### `lcax.lcax.Country.KOR`

```python
KOR = ('KOR',)
```

##### `lcax.lcax.Country.KWT`

```python
KWT = ('KWT',)
```

##### `lcax.lcax.Country.LAO`

```python
LAO = ('LAO',)
```

##### `lcax.lcax.Country.LBN`

```python
LBN = ('LBN',)
```

##### `lcax.lcax.Country.LBR`

```python
LBR = ('LBR',)
```

##### `lcax.lcax.Country.LBY`

```python
LBY = ('LBY',)
```

##### `lcax.lcax.Country.LCA`

```python
LCA = ('LCA',)
```

##### `lcax.lcax.Country.LIE`

```python
LIE = ('LIE',)
```

##### `lcax.lcax.Country.LKA`

```python
LKA = ('LKA',)
```

##### `lcax.lcax.Country.LSO`

```python
LSO = ('LSO',)
```

##### `lcax.lcax.Country.LTU`

```python
LTU = ('LTU',)
```

##### `lcax.lcax.Country.LUX`

```python
LUX = ('LUX',)
```

##### `lcax.lcax.Country.LVA`

```python
LVA = ('LVA',)
```

##### `lcax.lcax.Country.MAC`

```python
MAC = ('MAC',)
```

##### `lcax.lcax.Country.MAF`

```python
MAF = ('MAF',)
```

##### `lcax.lcax.Country.MAR`

```python
MAR = ('MAR',)
```

##### `lcax.lcax.Country.MCO`

```python
MCO = ('MCO',)
```

##### `lcax.lcax.Country.MDA`

```python
MDA = ('MDA',)
```

##### `lcax.lcax.Country.MDG`

```python
MDG = ('MDG',)
```

##### `lcax.lcax.Country.MDV`

```python
MDV = ('MDV',)
```

##### `lcax.lcax.Country.MEX`

```python
MEX = ('MEX',)
```

##### `lcax.lcax.Country.MHL`

```python
MHL = ('MHL',)
```

##### `lcax.lcax.Country.MKD`

```python
MKD = ('MKD',)
```

##### `lcax.lcax.Country.MLI`

```python
MLI = ('MLI',)
```

##### `lcax.lcax.Country.MLT`

```python
MLT = ('MLT',)
```

##### `lcax.lcax.Country.MMR`

```python
MMR = ('MMR',)
```

##### `lcax.lcax.Country.MNE`

```python
MNE = ('MNE',)
```

##### `lcax.lcax.Country.MNG`

```python
MNG = ('MNG',)
```

##### `lcax.lcax.Country.MNP`

```python
MNP = ('MNP',)
```

##### `lcax.lcax.Country.MOZ`

```python
MOZ = ('MOZ',)
```

##### `lcax.lcax.Country.MRT`

```python
MRT = ('MRT',)
```

##### `lcax.lcax.Country.MSR`

```python
MSR = ('MSR',)
```

##### `lcax.lcax.Country.MTQ`

```python
MTQ = ('MTQ',)
```

##### `lcax.lcax.Country.MUS`

```python
MUS = ('MUS',)
```

##### `lcax.lcax.Country.MWI`

```python
MWI = ('MWI',)
```

##### `lcax.lcax.Country.MYS`

```python
MYS = ('MYS',)
```

##### `lcax.lcax.Country.MYT`

```python
MYT = ('MYT',)
```

##### `lcax.lcax.Country.NAM`

```python
NAM = ('NAM',)
```

##### `lcax.lcax.Country.NCL`

```python
NCL = ('NCL',)
```

##### `lcax.lcax.Country.NER`

```python
NER = ('NER',)
```

##### `lcax.lcax.Country.NFK`

```python
NFK = ('NFK',)
```

##### `lcax.lcax.Country.NGA`

```python
NGA = ('NGA',)
```

##### `lcax.lcax.Country.NIC`

```python
NIC = ('NIC',)
```

##### `lcax.lcax.Country.NIU`

```python
NIU = ('NIU',)
```

##### `lcax.lcax.Country.NLD`

```python
NLD = ('NLD',)
```

##### `lcax.lcax.Country.NOR`

```python
NOR = ('NOR',)
```

##### `lcax.lcax.Country.NPL`

```python
NPL = ('NPL',)
```

##### `lcax.lcax.Country.NRU`

```python
NRU = ('NRU',)
```

##### `lcax.lcax.Country.NZL`

```python
NZL = ('NZL',)
```

##### `lcax.lcax.Country.OMN`

```python
OMN = ('OMN',)
```

##### `lcax.lcax.Country.PAK`

```python
PAK = ('PAK',)
```

##### `lcax.lcax.Country.PAN`

```python
PAN = ('PAN',)
```

##### `lcax.lcax.Country.PCN`

```python
PCN = ('PCN',)
```

##### `lcax.lcax.Country.PER`

```python
PER = ('PER',)
```

##### `lcax.lcax.Country.PHL`

```python
PHL = ('PHL',)
```

##### `lcax.lcax.Country.PLW`

```python
PLW = ('PLW',)
```

##### `lcax.lcax.Country.PNG`

```python
PNG = ('PNG',)
```

##### `lcax.lcax.Country.POL`

```python
POL = ('POL',)
```

##### `lcax.lcax.Country.PRI`

```python
PRI = ('PRI',)
```

##### `lcax.lcax.Country.PRK`

```python
PRK = ('PRK',)
```

##### `lcax.lcax.Country.PRT`

```python
PRT = ('PRT',)
```

##### `lcax.lcax.Country.PRY`

```python
PRY = ('PRY',)
```

##### `lcax.lcax.Country.PSE`

```python
PSE = ('PSE',)
```

##### `lcax.lcax.Country.PYF`

```python
PYF = ('PYF',)
```

##### `lcax.lcax.Country.QAT`

```python
QAT = ('QAT',)
```

##### `lcax.lcax.Country.REU`

```python
REU = ('REU',)
```

##### `lcax.lcax.Country.ROU`

```python
ROU = ('ROU',)
```

##### `lcax.lcax.Country.RUS`

```python
RUS = ('RUS',)
```

##### `lcax.lcax.Country.RWA`

```python
RWA = ('RWA',)
```

##### `lcax.lcax.Country.SAU`

```python
SAU = ('SAU',)
```

##### `lcax.lcax.Country.SDN`

```python
SDN = ('SDN',)
```

##### `lcax.lcax.Country.SEN`

```python
SEN = ('SEN',)
```

##### `lcax.lcax.Country.SGP`

```python
SGP = ('SGP',)
```

##### `lcax.lcax.Country.SGS`

```python
SGS = ('SGS',)
```

##### `lcax.lcax.Country.SHN`

```python
SHN = ('SHN',)
```

##### `lcax.lcax.Country.SJM`

```python
SJM = ('SJM',)
```

##### `lcax.lcax.Country.SLB`

```python
SLB = ('SLB',)
```

##### `lcax.lcax.Country.SLE`

```python
SLE = ('SLE',)
```

##### `lcax.lcax.Country.SLV`

```python
SLV = ('SLV',)
```

##### `lcax.lcax.Country.SMR`

```python
SMR = ('SMR',)
```

##### `lcax.lcax.Country.SOM`

```python
SOM = ('SOM',)
```

##### `lcax.lcax.Country.SPM`

```python
SPM = ('SPM',)
```

##### `lcax.lcax.Country.SRB`

```python
SRB = ('SRB',)
```

##### `lcax.lcax.Country.SSD`

```python
SSD = ('SSD',)
```

##### `lcax.lcax.Country.STP`

```python
STP = ('STP',)
```

##### `lcax.lcax.Country.SUR`

```python
SUR = ('SUR',)
```

##### `lcax.lcax.Country.SVK`

```python
SVK = ('SVK',)
```

##### `lcax.lcax.Country.SVN`

```python
SVN = ('SVN',)
```

##### `lcax.lcax.Country.SWE`

```python
SWE = ('SWE',)
```

##### `lcax.lcax.Country.SWZ`

```python
SWZ = ('SWZ',)
```

##### `lcax.lcax.Country.SXM`

```python
SXM = ('SXM',)
```

##### `lcax.lcax.Country.SYC`

```python
SYC = ('SYC',)
```

##### `lcax.lcax.Country.SYR`

```python
SYR = ('SYR',)
```

##### `lcax.lcax.Country.TCA`

```python
TCA = ('TCA',)
```

##### `lcax.lcax.Country.TCD`

```python
TCD = ('TCD',)
```

##### `lcax.lcax.Country.TGO`

```python
TGO = ('TGO',)
```

##### `lcax.lcax.Country.THA`

```python
THA = ('THA',)
```

##### `lcax.lcax.Country.TJK`

```python
TJK = ('TJK',)
```

##### `lcax.lcax.Country.TKL`

```python
TKL = ('TKL',)
```

##### `lcax.lcax.Country.TKM`

```python
TKM = ('TKM',)
```

##### `lcax.lcax.Country.TLS`

```python
TLS = ('TLS',)
```

##### `lcax.lcax.Country.TON`

```python
TON = ('TON',)
```

##### `lcax.lcax.Country.TTO`

```python
TTO = ('TTO',)
```

##### `lcax.lcax.Country.TUN`

```python
TUN = ('TUN',)
```

##### `lcax.lcax.Country.TUR`

```python
TUR = ('TUR',)
```

##### `lcax.lcax.Country.TUV`

```python
TUV = ('TUV',)
```

##### `lcax.lcax.Country.TWN`

```python
TWN = ('TWN',)
```

##### `lcax.lcax.Country.TZA`

```python
TZA = ('TZA',)
```

##### `lcax.lcax.Country.UGA`

```python
UGA = ('UGA',)
```

##### `lcax.lcax.Country.UKR`

```python
UKR = ('UKR',)
```

##### `lcax.lcax.Country.UMI`

```python
UMI = ('UMI',)
```

##### `lcax.lcax.Country.UNKNOWN`

```python
UNKNOWN = ('UNKNOWN',)
```

##### `lcax.lcax.Country.URY`

```python
URY = ('URY',)
```

##### `lcax.lcax.Country.USA`

```python
USA = ('USA',)
```

##### `lcax.lcax.Country.UZB`

```python
UZB = ('UZB',)
```

##### `lcax.lcax.Country.VAT`

```python
VAT = ('VAT',)
```

##### `lcax.lcax.Country.VCT`

```python
VCT = ('VCT',)
```

##### `lcax.lcax.Country.VEN`

```python
VEN = ('VEN',)
```

##### `lcax.lcax.Country.VGB`

```python
VGB = ('VGB',)
```

##### `lcax.lcax.Country.VIR`

```python
VIR = ('VIR',)
```

##### `lcax.lcax.Country.VNM`

```python
VNM = ('VNM',)
```

##### `lcax.lcax.Country.VUT`

```python
VUT = ('VUT',)
```

##### `lcax.lcax.Country.WLF`

```python
WLF = ('WLF',)
```

##### `lcax.lcax.Country.WSM`

```python
WSM = ('WSM',)
```

##### `lcax.lcax.Country.YEM`

```python
YEM = ('YEM',)
```

##### `lcax.lcax.Country.ZAF`

```python
ZAF = ('ZAF',)
```

##### `lcax.lcax.Country.ZMB`

```python
ZMB = ('ZMB',)
```

##### `lcax.lcax.Country.ZWE`

```python
ZWE = ('ZWE',)
```

##### `lcax.lcax.Country.from_string`

```python
from_string(value)
```

##### `lcax.lcax.Country.name`

```python
name()
```

#### `lcax.lcax.EPD`

```python
EPD(name, declared_unit, version, published_date, valid_until, standard, location, subtype, impacts, id=None, source=None, reference_service_life=None, comment=None, conversions=None, meta_data=None)
```

##### `lcax.lcax.EPD.comment`

```python
comment: str | None
```

##### `lcax.lcax.EPD.conversions`

```python
conversions: list[Conversion] | None
```

##### `lcax.lcax.EPD.declared_unit`

```python
declared_unit: Unit
```

##### `lcax.lcax.EPD.dumps`

```python
dumps()
```

Serializes the EPD into a JSON string

##### `lcax.lcax.EPD.id`

```python
id: str
```

##### `lcax.lcax.EPD.impacts`

```python
impacts: Impacts
```

##### `lcax.lcax.EPD.loads`

```python
loads(value)
```

Deserializes a JSON string into a LCAx EPD

##### `lcax.lcax.EPD.location`

```python
location: Country
```

##### `lcax.lcax.EPD.meta_data`

```python
meta_data: MetaData | None
```

##### `lcax.lcax.EPD.name`

```python
name: str
```

##### `lcax.lcax.EPD.published_date`

```python
published_date: datetime.date
```

##### `lcax.lcax.EPD.reference_service_life`

```python
reference_service_life: int | None
```

##### `lcax.lcax.EPD.source`

```python
source: Source | None
```

##### `lcax.lcax.EPD.standard`

```python
standard: Standard
```

##### `lcax.lcax.EPD.subtype`

```python
subtype: SubType
```

##### `lcax.lcax.EPD.valid_until`

```python
valid_until: datetime.date
```

##### `lcax.lcax.EPD.version`

```python
version: str
```

#### `lcax.lcax.GenericData`

```python
GenericData(name, declared_unit, impacts, id=None, source=None, comment=None, conversions=None, meta_data=None)
```

##### `lcax.lcax.GenericData.comment`

```python
comment: str | None
```

##### `lcax.lcax.GenericData.conversions`

```python
conversions: list[Conversion] | None
```

##### `lcax.lcax.GenericData.declared_unit`

```python
declared_unit: Unit
```

##### `lcax.lcax.GenericData.id`

```python
id: str
```

##### `lcax.lcax.GenericData.impacts`

```python
impacts: Impacts
```

##### `lcax.lcax.GenericData.meta_data`

```python
meta_data: MetaData | None
```

##### `lcax.lcax.GenericData.name`

```python
name: str
```

##### `lcax.lcax.GenericData.source`

```python
source: Source | None
```

#### `lcax.lcax.ImpactCategory`

```python
ImpactCategory(value=None)
```

##### `lcax.lcax.ImpactCategory.dict`

```python
dict()
```

##### `lcax.lcax.ImpactCategory.from_dict`

```python
from_dict(value)
```

#### `lcax.lcax.ImpactCategoryKey`

Bases: <code>[Enum](#enum.Enum)</code>

##### `lcax.lcax.ImpactCategoryKey.ADPE`

```python
ADPE = ('ADPE',)
```

##### `lcax.lcax.ImpactCategoryKey.ADPF`

```python
ADPF = ('ADPF',)
```

##### `lcax.lcax.ImpactCategoryKey.AP`

```python
AP = ('AP',)
```

##### `lcax.lcax.ImpactCategoryKey.CRU`

```python
CRU = ('CRU',)
```

##### `lcax.lcax.ImpactCategoryKey.EEE`

```python
EEE = ('EEE',)
```

##### `lcax.lcax.ImpactCategoryKey.EET`

```python
EET = ('EET',)
```

##### `lcax.lcax.ImpactCategoryKey.EP`

```python
EP = ('EP',)
```

##### `lcax.lcax.ImpactCategoryKey.EP_FW`

```python
EP_FW = ('EP_FW',)
```

##### `lcax.lcax.ImpactCategoryKey.EP_MAR`

```python
EP_MAR = ('EP_MAR',)
```

##### `lcax.lcax.ImpactCategoryKey.EP_TER`

```python
EP_TER = ('EP_TER',)
```

##### `lcax.lcax.ImpactCategoryKey.ETP_FW`

```python
ETP_FW = ('ETP_FW',)
```

##### `lcax.lcax.ImpactCategoryKey.FW`

```python
FW = ('FW',)
```

##### `lcax.lcax.ImpactCategoryKey.GWP`

```python
GWP = ('GWP',)
```

##### `lcax.lcax.ImpactCategoryKey.GWP_BIO`

```python
GWP_BIO = ('GWP_BIO',)
```

##### `lcax.lcax.ImpactCategoryKey.GWP_FOS`

```python
GWP_FOS = ('GWP_FOS',)
```

##### `lcax.lcax.ImpactCategoryKey.GWP_LUL`

```python
GWP_LUL = ('GWP_LUL',)
```

##### `lcax.lcax.ImpactCategoryKey.HTP_C`

```python
HTP_C = ('HTP_C',)
```

##### `lcax.lcax.ImpactCategoryKey.HTP_NC`

```python
HTP_NC = ('HTP_NC',)
```

##### `lcax.lcax.ImpactCategoryKey.HWD`

```python
HWD = ('HWD',)
```

##### `lcax.lcax.ImpactCategoryKey.IRP`

```python
IRP = ('IRP',)
```

##### `lcax.lcax.ImpactCategoryKey.MER`

```python
MER = ('MER',)
```

##### `lcax.lcax.ImpactCategoryKey.MRF`

```python
MRF = ('MRF',)
```

##### `lcax.lcax.ImpactCategoryKey.NHWD`

```python
NHWD = ('NHWD',)
```

##### `lcax.lcax.ImpactCategoryKey.NRSF`

```python
NRSF = ('NRSF',)
```

##### `lcax.lcax.ImpactCategoryKey.ODP`

```python
ODP = ('ODP',)
```

##### `lcax.lcax.ImpactCategoryKey.PENRE`

```python
PENRE = ('PENRE',)
```

##### `lcax.lcax.ImpactCategoryKey.PENRM`

```python
PENRM = ('PENRM',)
```

##### `lcax.lcax.ImpactCategoryKey.PENRT`

```python
PENRT = ('PENRT',)
```

##### `lcax.lcax.ImpactCategoryKey.PERE`

```python
PERE = ('PERE',)
```

##### `lcax.lcax.ImpactCategoryKey.PERM`

```python
PERM = ('PERM',)
```

##### `lcax.lcax.ImpactCategoryKey.PERT`

```python
PERT = ('PERT',)
```

##### `lcax.lcax.ImpactCategoryKey.PM`

```python
PM = ('PM',)
```

##### `lcax.lcax.ImpactCategoryKey.POCP`

```python
POCP = ('POCP',)
```

##### `lcax.lcax.ImpactCategoryKey.RSF`

```python
RSF = ('RSF',)
```

##### `lcax.lcax.ImpactCategoryKey.RWD`

```python
RWD = ('RWD',)
```

##### `lcax.lcax.ImpactCategoryKey.SM`

```python
SM = ('SM',)
```

##### `lcax.lcax.ImpactCategoryKey.SQP`

```python
SQP = ('SQP',)
```

##### `lcax.lcax.ImpactCategoryKey.WDP`

```python
WDP = ('WDP',)
```

#### `lcax.lcax.Impacts`

```python
Impacts(value=None)
```

##### `lcax.lcax.Impacts.dict`

```python
dict()
```

##### `lcax.lcax.Impacts.from_dict`

```python
from_dict(value)
```

#### `lcax.lcax.Level`

Bases: <code>[Enum](#enum.Enum)</code>

##### `lcax.lcax.Level.Assembly`

```python
Assembly = ('ASSEMBLY',)
```

##### `lcax.lcax.Level.ImpactData`

```python
ImpactData = ('IMPACT_DATA',)
```

##### `lcax.lcax.Level.Product`

```python
Product = ('PRODUCT',)
```

##### `lcax.lcax.Level.Project`

```python
Project = ('PROJECT',)
```

#### `lcax.lcax.LifeCycleModule`

Bases: <code>[Enum](#enum.Enum)</code>

##### `lcax.lcax.LifeCycleModule.A0`

```python
A0 = ('A0',)
```

##### `lcax.lcax.LifeCycleModule.A1A3`

```python
A1A3 = ('A1A3',)
```

##### `lcax.lcax.LifeCycleModule.A4`

```python
A4 = ('A4',)
```

##### `lcax.lcax.LifeCycleModule.A5`

```python
A5 = ('A5',)
```

##### `lcax.lcax.LifeCycleModule.B1`

```python
B1 = ('B1',)
```

##### `lcax.lcax.LifeCycleModule.B2`

```python
B2 = ('B2',)
```

##### `lcax.lcax.LifeCycleModule.B3`

```python
B3 = ('B3',)
```

##### `lcax.lcax.LifeCycleModule.B4`

```python
B4 = ('B4',)
```

##### `lcax.lcax.LifeCycleModule.B5`

```python
B5 = ('B5',)
```

##### `lcax.lcax.LifeCycleModule.B6`

```python
B6 = ('B6',)
```

##### `lcax.lcax.LifeCycleModule.B7`

```python
B7 = ('B7',)
```

##### `lcax.lcax.LifeCycleModule.B8`

```python
B8 = ('B8',)
```

##### `lcax.lcax.LifeCycleModule.C1`

```python
C1 = ('C1',)
```

##### `lcax.lcax.LifeCycleModule.C2`

```python
C2 = ('C2',)
```

##### `lcax.lcax.LifeCycleModule.C3`

```python
C3 = ('C3',)
```

##### `lcax.lcax.LifeCycleModule.C4`

```python
C4 = ('C4',)
```

##### `lcax.lcax.LifeCycleModule.D`

```python
D = ('D',)
```

#### `lcax.lcax.Location`

```python
Location(country, city=None, address=None)
```

##### `lcax.lcax.Location.address`

```python
address: str | None
```

##### `lcax.lcax.Location.city`

```python
city: str | None
```

##### `lcax.lcax.Location.country`

```python
country: Country
```

#### `lcax.lcax.Product`

```python
Product(name, reference_service_life, impact_data, quantity, unit, id=None, description=None, transport=None, results=None, meta_data=None)
```

##### `lcax.lcax.Product.description`

```python
description: str | None
```

##### `lcax.lcax.Product.id`

```python
id: str
```

##### `lcax.lcax.Product.impact_data`

```python
impact_data: list[EPD | GenericData | Reference]
```

##### `lcax.lcax.Product.meta_data`

```python
meta_data: MetaData | None
```

##### `lcax.lcax.Product.name`

```python
name: str
```

##### `lcax.lcax.Product.quantity`

```python
quantity: float
```

##### `lcax.lcax.Product.reference_service_life`

```python
reference_service_life: int
```

##### `lcax.lcax.Product.results`

```python
results: Impacts | None
```

##### `lcax.lcax.Product.transport`

```python
transport: list[Transport] | None
```

##### `lcax.lcax.Product.unit`

```python
unit: Unit
```

#### `lcax.lcax.Project`

```python
Project(id, name, location, project_phase, software_info, life_cycle_modules, impact_categories, assemblies, description=None, comment=None, owner=None, format_version=None, lcia_method=None, classification_systems=None, reference_study_period=None, results=None, project_info=None, meta_data=None)
```

##### `lcax.lcax.Project.assemblies`

```python
assemblies: list[Assembly | Reference]
```

##### `lcax.lcax.Project.classification_systems`

```python
classification_systems: list[str] | None
```

##### `lcax.lcax.Project.comment`

```python
comment: str | None
```

##### `lcax.lcax.Project.description`

```python
description: str | None
```

##### `lcax.lcax.Project.dumps`

```python
dumps()
```

##### `lcax.lcax.Project.format_version`

```python
format_version: str
```

##### `lcax.lcax.Project.id`

```python
id: str
```

##### `lcax.lcax.Project.impact_categories`

```python
impact_categories: list[ImpactCategoryKey]
```

##### `lcax.lcax.Project.lcia_method`

```python
lcia_method: str | None
```

##### `lcax.lcax.Project.life_cycle_modules`

```python
life_cycle_modules: list[LifeCycleModule]
```

##### `lcax.lcax.Project.loads`

```python
loads(value)
```

##### `lcax.lcax.Project.location`

```python
location: Location
```

##### `lcax.lcax.Project.meta_data`

```python
meta_data: MetaData | None
```

##### `lcax.lcax.Project.name`

```python
name: str
```

##### `lcax.lcax.Project.owner`

```python
owner: str | None
```

##### `lcax.lcax.Project.project_info`

```python
project_info: BuildingInfo | None
```

##### `lcax.lcax.Project.project_phase`

```python
project_phase: ProjectPhase
```

##### `lcax.lcax.Project.reference_study_period`

```python
reference_study_period: int | None
```

##### `lcax.lcax.Project.results`

```python
results: Impacts | None
```

##### `lcax.lcax.Project.software_info`

```python
software_info: SoftwareInfo
```

#### `lcax.lcax.ProjectPhase`

Bases: <code>[Enum](#enum.Enum)</code>

##### `lcax.lcax.ProjectPhase.CONCEPT_DESIGN`

```python
CONCEPT_DESIGN = ('CONCEPT_DESIGN',)
```

##### `lcax.lcax.ProjectPhase.CONSTRUCTION`

```python
CONSTRUCTION = ('CONSTRUCTION',)
```

##### `lcax.lcax.ProjectPhase.IN_USE`

```python
IN_USE = ('IN_USE',)
```

##### `lcax.lcax.ProjectPhase.OTHER`

```python
OTHER = ('OTHER',)
```

##### `lcax.lcax.ProjectPhase.POST_COMPLETION`

```python
POST_COMPLETION = ('POST_COMPLETION',)
```

##### `lcax.lcax.ProjectPhase.STRATEGIC_DESIGN`

```python
STRATEGIC_DESIGN = ('STRATEGIC_DESIGN',)
```

##### `lcax.lcax.ProjectPhase.TECHNICAL_DESIGN`

```python
TECHNICAL_DESIGN = ('TECHNICAL_DESIGN',)
```

#### `lcax.lcax.Reference`

##### `lcax.lcax.Reference.format`

```python
format: str | None
```

##### `lcax.lcax.Reference.overrides`

```python
overrides: dict[str, Any] | None
```

##### `lcax.lcax.Reference.uri`

```python
uri: str
```

##### `lcax.lcax.Reference.version`

```python
version: str | None
```

#### `lcax.lcax.SoftwareInfo`

```python
SoftwareInfo(lca_software, lca_software_version=None, goal_and_scope_definition=None, calculation_type=None)
```

##### `lcax.lcax.SoftwareInfo.calculation_type`

```python
calculation_type: str | None
```

##### `lcax.lcax.SoftwareInfo.goal_and_scope_definition`

```python
goal_and_scope_definition: str | None
```

##### `lcax.lcax.SoftwareInfo.lca_software`

```python
lca_software: str
```

##### `lcax.lcax.SoftwareInfo.lca_software_version`

```python
lca_software_version: str | None
```

#### `lcax.lcax.Source`

```python
Source(name, url=None)
```

##### `lcax.lcax.Source.name`

```python
name: str
```

##### `lcax.lcax.Source.url`

```python
url: str | None
```

#### `lcax.lcax.Standard`

Bases: <code>[Enum](#enum.Enum)</code>

##### `lcax.lcax.Standard.EN15804A1`

```python
EN15804A1 = ('EN15804A1',)
```

##### `lcax.lcax.Standard.EN15804A2`

```python
EN15804A2 = ('EN15804A2',)
```

##### `lcax.lcax.Standard.UNKNOWN`

```python
UNKNOWN = 'UNKNOWN'
```

#### `lcax.lcax.SubType`

Bases: <code>[Enum](#enum.Enum)</code>

##### `lcax.lcax.SubType.GENERIC`

```python
GENERIC = ('GENERIC',)
```

##### `lcax.lcax.SubType.INDUSTRY`

```python
INDUSTRY = ('INDUSTRY',)
```

##### `lcax.lcax.SubType.REPRESENTATIVE`

```python
REPRESENTATIVE = 'REPRESENTATIVE'
```

##### `lcax.lcax.SubType.SPECIFIC`

```python
SPECIFIC = ('SPECIFIC',)
```

#### `lcax.lcax.Transport`

##### `lcax.lcax.Transport.distance`

```python
distance: float
```

##### `lcax.lcax.Transport.distance_unit`

```python
distance_unit: Unit
```

##### `lcax.lcax.Transport.id`

```python
id: str
```

##### `lcax.lcax.Transport.impact_data`

```python
impact_data: EPD | GenericData | Reference
```

##### `lcax.lcax.Transport.life_cycle_modules`

```python
life_cycle_modules: list[LifeCycleModule]
```

##### `lcax.lcax.Transport.name`

```python
name: str
```

#### `lcax.lcax.Unit`

Bases: <code>[Enum](#enum.Enum)</code>

##### `lcax.lcax.Unit.KG`

```python
KG = ('KG',)
```

##### `lcax.lcax.Unit.KGM3`

```python
KGM3 = ('KGM3',)
```

##### `lcax.lcax.Unit.KM`

```python
KM = ('KM',)
```

##### `lcax.lcax.Unit.KWH`

```python
KWH = ('KWH',)
```

##### `lcax.lcax.Unit.L`

```python
L = ('L',)
```

##### `lcax.lcax.Unit.M`

```python
M = ('M',)
```

##### `lcax.lcax.Unit.M2`

```python
M2 = ('M2',)
```

##### `lcax.lcax.Unit.M2R1`

```python
M2R1 = ('M2R1',)
```

##### `lcax.lcax.Unit.M3`

```python
M3 = ('M3',)
```

##### `lcax.lcax.Unit.PCS`

```python
PCS = ('PCS',)
```

##### `lcax.lcax.Unit.TONES`

```python
TONES = ('TONES',)
```

##### `lcax.lcax.Unit.TONES_KM`

```python
TONES_KM = ('TONES_KM',)
```

##### `lcax.lcax.Unit.UNKNOWN`

```python
UNKNOWN = 'UNKNOWN'
```

#### `lcax.lcax.ValidationResult`

##### `lcax.lcax.ValidationResult.field`

```python
field: str
```

##### `lcax.lcax.ValidationResult.message`

```python
message: str
```

#### `lcax.lcax.ValidationRule`

```python
ValidationRule(range=None, includes=None, required=None, equal=None, greater=None, less=None, one_of=None)
```

##### `lcax.lcax.ValidationRule.equal`

```python
equal: str | None
```

##### `lcax.lcax.ValidationRule.greater`

```python
greater: float | None
```

##### `lcax.lcax.ValidationRule.includes`

```python
includes: str | None
```

##### `lcax.lcax.ValidationRule.less`

```python
less: float | None
```

##### `lcax.lcax.ValidationRule.one_of`

```python
one_of: list[str] | None
```

##### `lcax.lcax.ValidationRule.range`

```python
range: list[float] | None
```

##### `lcax.lcax.ValidationRule.required`

```python
required: bool | None
```

#### `lcax.lcax.ValidationSchema`

```python
ValidationSchema(level, field, rule)
```

##### `lcax.lcax.ValidationSchema.field`

```python
field: str
```

##### `lcax.lcax.ValidationSchema.level`

```python
level: Level
```

##### `lcax.lcax.ValidationSchema.rule`

```python
rule: ValidationRule
```

#### `lcax.lcax.calculate_assembly`

```python
calculate_assembly(assembly, options)
```

Calculate the impact results for an Assembly.
The impact results for the assembly will be returned.

#### `lcax.lcax.calculate_product`

```python
calculate_product(product, options)
```

Calculate the impact results for a Product.
The impact results for the product will be returned.

#### `lcax.lcax.calculate_project`

```python
calculate_project(project)
```

Calculate the impact results for a Project.
The impact results for the project will be added to the `results` property.

#### `lcax.lcax.convert_br_standard`

```python
convert_br_standard(file_path)
```

Converts a BR Standard Format .xlsx file into a LCAx Project

#### `lcax.lcax.convert_ilcd`

```python
convert_ilcd(data)
```

Converts a json formatted ILCD+EPD data string into a LCAx EPD

#### `lcax.lcax.convert_lcabyg`

```python
convert_lcabyg(data, result_data=None)
```

Converts a json formatted LCAByg project into a LCAx Project

#### `lcax.lcax.get_impact_total`

```python
get_impact_total(impacts, category, exclude_modules=None)
```

Get the total impact

#### `lcax.lcax.get_impacts_by_life_cycle_module`

```python
get_impacts_by_life_cycle_module(impacts, category, exclude_modules=None, normalizing_factor=None)
```

Get the impacts by life cycle module.
The results can be normalized by a factor.

#### `lcax.lcax.normalize_result`

```python
normalize_result(result, normalizing_factor)
```

Normalize a result with e.g. the reference study period and gross floor area

#### `lcax.lcax.to_lcabyg`

```python
to_lcabyg(*, project=None, epds=None)
```

Converts LCAx objects into LCAbyg JSON

#### `lcax.lcax.validate`

```python
validate(project, validation_schema)
```

Validate a LCAx Project
