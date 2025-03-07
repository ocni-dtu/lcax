import datetime
from enum import Enum
from typing import Any, Self


class SoftwareInfo:
    lca_software: str
    lca_software_version: str | None
    goal_and_scope_definition: str | None
    calculation_type: str | None

    def __init__(self, lca_software: str, lca_software_version=None, goal_and_scope_definition=None,
                 calculation_type=None):
        ...


class ImpactCategoryKey(Enum):
    GWP = "GWP",
    GWP_FOS = "GWP_FOS",
    GWP_BIO = "GWP_BIO",
    GWP_LUL = "GWP_LUL",
    ODP = "ODP",
    AP = "AP",
    EP = "EP",
    EP_FW = "EP_FW",
    EP_MAR = "EP_MAR",
    EP_TER = "EP_TER",
    POCP = "POCP",
    ADPE = "ADPE",
    ADPF = "ADPF",
    PENRE = "PENRE",
    PERE = "PERE",
    PERM = "PERM",
    PERT = "PERT",
    PENRT = "PENRT",
    PENRM = "PENRM",
    SM = "SM",
    PM = "PM",
    WDP = "WDP",
    IRP = "IRP",
    ETP_FW = "ETP_FW",
    HTP_C = "HTP_C",
    HTP_NC = "HTP_NC",
    SQP = "SQP",
    RSF = "RSF",
    NRSF = "NRSF",
    FW = "FW",
    HWD = "HWD",
    NHWD = "NHWD",
    RWD = "RWD",
    CRU = "CRU",
    MRF = "MRF",
    MER = "MER",
    EEE = "EEE",
    EET = "EET",


class LifeCycleStage(Enum):
    A0 = "A0",
    A1A3 = "A1A3",
    A4 = "A4",
    A5 = "A5",
    B1 = "B1",
    B2 = "B2",
    B3 = "B3",
    B4 = "B4",
    B5 = "B5",
    B6 = "B6",
    B7 = "B7",
    B8 = "B8",
    C1 = "C1",
    C2 = "C2",
    C3 = "C3",
    C4 = "C4",
    D = "D",


class ProjectPhase(Enum):
    STRATEGIC_DESIGN = "STRATEGIC_DESIGN",
    CONCEPT_DESIGN = "CONCEPT_DESIGN",
    TECHNICAL_DESIGN = "TECHNICAL_DESIGN",
    CONSTRUCTION = "CONSTRUCTION",
    POST_COMPLETION = "POST_COMPLETION",
    IN_USE = "IN_USE",
    OTHER = "OTHER",


class Country(Enum):
    UNKNOWN = "UNKNOWN",
    AFG = "AFG",
    ALA = "ALA",
    ALB = "ALB",
    DZA = "DZA",
    ASM = "ASM",
    AND = "AND",
    AGO = "AGO",
    AIA = "AIA",
    ATA = "ATA",
    ATG = "ATG",
    ARG = "ARG",
    ARM = "ARM",
    ABW = "ABW",
    AUS = "AUS",
    AUT = "AUT",
    AZE = "AZE",
    BHS = "BHS",
    BHR = "BHR",
    BGD = "BGD",
    BRB = "BRB",
    BLR = "BLR",
    BEL = "BEL",
    BLZ = "BLZ",
    BEN = "BEN",
    BMU = "BMU",
    BTN = "BTN",
    BOL = "BOL",
    BES = "BES",
    BIH = "BIH",
    BWA = "BWA",
    BVT = "BVT",
    BRA = "BRA",
    IOT = "IOT",
    BRN = "BRN",
    BGR = "BGR",
    BFA = "BFA",
    BDI = "BDI",
    CPV = "CPV",
    KHM = "KHM",
    CMR = "CMR",
    CAN = "CAN",
    CYM = "CYM",
    CAF = "CAF",
    TCD = "TCD",
    CHL = "CHL",
    CHN = "CHN",
    CXR = "CXR",
    CCK = "CCK",
    COL = "COL",
    COM = "COM",
    COG = "COG",
    COD = "COD",
    COK = "COK",
    CRI = "CRI",
    CIV = "CIV",
    HRV = "HRV",
    CUB = "CUB",
    CUW = "CUW",
    CYP = "CYP",
    CZE = "CZE",
    DNK = "DNK",
    DJI = "DJI",
    DMA = "DMA",
    DOM = "DOM",
    ECU = "ECU",
    EGY = "EGY",
    SLV = "SLV",
    GNQ = "GNQ",
    ERI = "ERI",
    EST = "EST",
    SWZ = "SWZ",
    ETH = "ETH",
    FLK = "FLK",
    FRO = "FRO",
    FJI = "FJI",
    FIN = "FIN",
    FRA = "FRA",
    GUF = "GUF",
    PYF = "PYF",
    ATF = "ATF",
    GAB = "GAB",
    GMB = "GMB",
    GEO = "GEO",
    DEU = "DEU",
    GHA = "GHA",
    GIB = "GIB",
    GRC = "GRC",
    GRL = "GRL",
    GRD = "GRD",
    GLP = "GLP",
    GUM = "GUM",
    GTM = "GTM",
    GGY = "GGY",
    GIN = "GIN",
    GNB = "GNB",
    GUY = "GUY",
    HTI = "HTI",
    HMD = "HMD",
    VAT = "VAT",
    HND = "HND",
    HKG = "HKG",
    HUN = "HUN",
    ISL = "ISL",
    IND = "IND",
    IDN = "IDN",
    IRN = "IRN",
    IRQ = "IRQ",
    IRL = "IRL",
    IMN = "IMN",
    ISR = "ISR",
    ITA = "ITA",
    JAM = "JAM",
    JPN = "JPN",
    JEY = "JEY",
    JOR = "JOR",
    KAZ = "KAZ",
    KEN = "KEN",
    KIR = "KIR",
    PRK = "PRK",
    KOR = "KOR",
    KWT = "KWT",
    KGZ = "KGZ",
    LAO = "LAO",
    LVA = "LVA",
    LBN = "LBN",
    LSO = "LSO",
    LBR = "LBR",
    LBY = "LBY",
    LIE = "LIE",
    LTU = "LTU",
    LUX = "LUX",
    MAC = "MAC",
    MDG = "MDG",
    MWI = "MWI",
    MYS = "MYS",
    MDV = "MDV",
    MLI = "MLI",
    MLT = "MLT",
    MHL = "MHL",
    MTQ = "MTQ",
    MRT = "MRT",
    MUS = "MUS",
    MYT = "MYT",
    MEX = "MEX",
    FSM = "FSM",
    MDA = "MDA",
    MCO = "MCO",
    MNG = "MNG",
    MNE = "MNE",
    MSR = "MSR",
    MAR = "MAR",
    MOZ = "MOZ",
    MMR = "MMR",
    NAM = "NAM",
    NRU = "NRU",
    NPL = "NPL",
    NLD = "NLD",
    NCL = "NCL",
    NZL = "NZL",
    NIC = "NIC",
    NER = "NER",
    NGA = "NGA",
    NIU = "NIU",
    NFK = "NFK",
    MKD = "MKD",
    MNP = "MNP",
    NOR = "NOR",
    OMN = "OMN",
    PAK = "PAK",
    PLW = "PLW",
    PSE = "PSE",
    PAN = "PAN",
    PNG = "PNG",
    PRY = "PRY",
    PER = "PER",
    PHL = "PHL",
    PCN = "PCN",
    POL = "POL",
    PRT = "PRT",
    PRI = "PRI",
    QAT = "QAT",
    REU = "REU",
    ROU = "ROU",
    RUS = "RUS",
    RWA = "RWA",
    BLM = "BLM",
    SHN = "SHN",
    KNA = "KNA",
    LCA = "LCA",
    MAF = "MAF",
    SPM = "SPM",
    VCT = "VCT",
    WSM = "WSM",
    SMR = "SMR",
    STP = "STP",
    SAU = "SAU",
    SEN = "SEN",
    SRB = "SRB",
    SYC = "SYC",
    SLE = "SLE",
    SGP = "SGP",
    SXM = "SXM",
    SVK = "SVK",
    SVN = "SVN",
    SLB = "SLB",
    SOM = "SOM",
    ZAF = "ZAF",
    SGS = "SGS",
    SSD = "SSD",
    ESP = "ESP",
    LKA = "LKA",
    SDN = "SDN",
    SUR = "SUR",
    SJM = "SJM",
    SWE = "SWE",
    CHE = "CHE",
    SYR = "SYR",
    TWN = "TWN",
    TJK = "TJK",
    TZA = "TZA",
    THA = "THA",
    TLS = "TLS",
    TGO = "TGO",
    TKL = "TKL",
    TON = "TON",
    TTO = "TTO",
    TUN = "TUN",
    TUR = "TUR",
    TKM = "TKM",
    TCA = "TCA",
    TUV = "TUV",
    UGA = "UGA",
    UKR = "UKR",
    ARE = "ARE",
    GBR = "GBR",
    USA = "USA",
    UMI = "UMI",
    URY = "URY",
    UZB = "UZB",
    VUT = "VUT",
    VEN = "VEN",
    VNM = "VNM",
    VGB = "VGB",
    VIR = "VIR",
    WLF = "WLF",
    ESH = "ESH",
    YEM = "YEM",
    ZMB = "ZMB",
    ZWE = "ZWE",


class Location:
    country: Country
    city: str | None
    address: str | None

    def __init__(self, country: Country, city=None, address=None):
        ...


class Unit(Enum):
    M = "M",
    M2 = "M2",
    M3 = "M3",
    KG = "KG",
    TONES = "TONES",
    PCS = "PCS",
    KWH = "KWH",
    L = "L",
    M2R1 = "M2R1",
    KM = "KM",
    TONES_KM = "TONES_KM",
    KGM3 = "KGM3",
    UNKNOWN = "UNKNOWN"


class Conversion:
    value: float
    to: Unit
    meta_data: MetaData | None


class Source:
    name: str
    url: str | None


class Standard(Enum):
    EN15804A1 = "EN15804A1",
    EN15804A2 = "EN15804A2",
    UNKNOWN = "UNKNOWN"


class SubType(Enum):
    GENERIC = "GENERIC",
    SPECIFIC = "SPECIFIC",
    INDUSTRY = "INDUSTRY",
    REPRESENTATIVE = "REPRESENTATIVE"


class GenericData:
    id: str
    name: str
    declared_unit: Unit
    format_version: str
    source: Source | None
    comment: str | None
    conversions: list[Conversion] | None
    impacts: Impacts
    meta_data: MetaData | None

    def __init__(self, name: str, declared_unit: Unit, impacts: Impacts, id: str | None = None,
                 format_version: str | None = None, source: Source | None = None, comment: str | None = None,
                 conversions: list[Conversion] | None = None, meta_data: MetaData | None = None):
        ...


class EPD:
    id: str
    name: str
    declared_unit: Unit
    version: str
    published_date: datetime.date
    valid_until: datetime.date
    format_version: str
    source: Source | None
    reference_service_life: int | None
    standard: Standard
    comment: str | None
    location: Country
    subtype: SubType
    conversions: list[Conversion] | None
    impacts: Impacts
    meta_data: MetaData | None

    def __init__(self, name: str, declared_unit: Unit, version: str, published_date: datetime.date,
                 valid_until: datetime.date, standard: Standard, location: Country, subtype: SubType, impacts: Impacts,
                 id: str | None = None, format_version: str | None = None,
                 source: Source | None = None, reference_service_life: int | None = None, comment: str | None = None,
                 conversions: list[Conversion] | None = None, meta_data: MetaData | None = None):
        ...

    def dumps(self) -> str:
        ...

    @classmethod
    def loads(cls, value: str) -> Self:
        ...


class AreaType:
    value: float
    unit: Unit
    definition: str


class ValueUnit:
    value: float
    unit: Unit


class RoofType(Enum):
    FLAT = "FLAT",
    PITCHED = "PITCHED",
    SADDLE = "SADDLE",
    PYRAMID = "PYRAMID",
    UNKNOWN = "UNKNOWN",
    OTHER = "OTHER",


class GeneralEnergyClass(Enum):
    EXISTING = "EXISTING",
    STANDARD = "STANDARD",
    ADVANCED = "ADVANCED",
    UNKNOWN = "UNKNOWN",


class BuildingModelScope(Enum):
    FACILITATING_WORKS = "FACILITATING_WORKS",
    SUBSTRUCTURE = "SUBSTRUCTURE",
    SUPERSTRUCTURE_FRAME = "SUPERSTRUCTURE_FRAME",
    SUPERSTRUCTURE_ENVELOPE = "SUPERSTRUCTURE_ENVELOPE",
    SUPERSTRUCTURE_INTERNAL_ELEMENTS = "SUPERSTRUCTURE_INTERNAL_ELEMENTS",
    FINISHES = "FINISHES",
    BUILDING_SERVICES = "BUILDING_SERVICES",
    EXTERNAL_WORKS = "EXTERNAL_WORKS",
    FF_E = "FF_E",


class BuildingType(Enum):
    NEW_CONSTRUCTION_WORKS = "NEW_CONSTRUCTION_WORKS",
    DEMOLITION = "DEMOLITION",
    DECONSTRUCTION_AND_NEW_CONSTRUCTION_WORKS = "DECONSTRUCTION_AND_NEW_CONSTRUCTION_WORKS",
    RETROFIT_WORKS = "RETROFIT_WORKS",
    EXTENSION_WORKS = "EXTENSION_WORKS",
    RETROFIT_AND_EXTENSION_WORKS = "RETROFIT_AND_EXTENSION_WORKS",
    FIT_OUT_WORKS = "FIT_OUT_WORKS",
    OPERATIONS = "OPERATIONS",
    UNKNOWN = "UNKNOWN",
    OTHER = "OTHER",


class BuildingTypology(Enum):
    OFFICE = "OFFICE",
    RESIDENTIAL = "RESIDENTIAL",
    PUBLIC = "PUBLIC",
    COMMERCIAL = "COMMERCIAL",
    INDUSTRIAL = "INDUSTRIAL",
    INFRASTRUCTURE = "INFRASTRUCTURE",
    AGRICULTURAL = "AGRICULTURAL",
    EDUCATIONAL = "EDUCATIONAL",
    HEALTH = "HEALTH",
    UNKNOWN = "UNKNOWN",
    OTHER = "OTHER",


class BuildingInfo:
    building_type: BuildingType
    building_typology: list[BuildingTypology]
    certifications: list[str] | None
    building_mass: ValueUnit | None
    building_height: ValueUnit | None
    gross_floor_area: AreaType | None
    heated_floor_area: AreaType | None
    building_footprint: ValueUnit | None
    floors_above_ground: int
    floors_below_ground: int | None
    roof_type: RoofType
    frame_type: str | None
    building_completion_year: int | None
    building_permit_year: int | None
    energy_demand_heating: float | None
    energy_supply_heating: float | None
    energy_demand_electricity: float | None
    energy_supply_electricity: float | None
    exported_electricity: float | None
    general_energy_class: GeneralEnergyClass
    local_energy_class: str | None
    building_users: int | None
    building_model_scope: list[BuildingModelScope] | None


class Reference:
    uri: str
    format: str | None
    version: str | None
    overrides: dict[str, Any] | None


class Classification:
    system: str
    code: str
    name: str

    def __init__(self, system: str, code: str, name: str):
        ...


class Product:
    id: str
    name: str
    description: str | None
    reference_service_life: int
    impact_data: list[ImpactData]
    quantity: float
    unit: Unit
    transport: list[Transport] | None
    results: Impacts | None
    meta_data: MetaData | None

    def __init__(self, name: str, reference_service_life: int, impact_data: ImpactData, quantity: float, unit: Unit,
                 id: str | None = None,
                 description: str | None = None, transport: list[Transport] | None = None,
                 results: Impacts | None = None, meta_data: MetaData | None = None):
        ...


class Assembly:
    id: str
    name: str
    description: str | None
    comment: str | None
    quantity: float
    unit: Unit
    classification: list[Classification] | None
    products: list[Product | Reference]
    results: Impacts | None
    meta_data: MetaData | None

    def __init__(self, name: str, quantity: float, unit: Unit, products: dict[str, Product | Reference],
                 id: str | None = None,
                 description: str | None = None, comment: str | None = None,
                 classification: str | None = None, results: Impacts | None = None, meta_data: MetaData | None = None):
        ...


class Transport:
    id: str
    name: str
    life_cycle_stages: list[LifeCycleStage]
    distance: float
    distance_unit: Unit
    impact_data: ImpactData


class ImpactData:
    def __init__(self, _type: str, name: str, declared_unit: Unit, version: str, published_date: datetime.date,
                 valid_until: datetime.date, standard: Standard,
                 location: Country, subtype: SubType, impacts: Impacts, id: str | None = None,
                 format_version: str | None = None, source: Source | None = None,
                 reference_service_life: int | None = None,
                 comment: str | None = None, conversions: list[Conversion] | None = None,
                 meta_data: MetaData | None = None) -> EPD | GenericData:
        ...


type ImpactCategory = dict[LifeCycleStage, float | None]
# type ImpactData = EPD | GenericData | Reference
type ProjectInfo = BuildingInfo | dict[str, str]
type Impacts = dict[ImpactCategoryKey, ImpactCategory]
type MetaData = dict[str, Any]


class Project:
    id: str
    name: str
    description: str | None
    comment: str | None
    location: Location
    owner: str | None
    format_version: str
    lcia_method: str | None
    classification_systems: list[str] | None
    reference_study_period: int | None
    life_cycle_stages: list[LifeCycleStage]
    impact_categories: list[ImpactCategoryKey]
    assemblies: list[Assembly | Reference]
    results: Impacts | None
    project_info: ProjectInfo | None
    project_phase: ProjectPhase
    software_info: SoftwareInfo
    meta_data: MetaData | None

    def __init__(self, id: str, name: str, location: Location, project_phase: ProjectPhase, software_info,
                 life_cycle_stages, impact_categories, assemblies, description=None, comment=None, owner=None,
                 format_version=None, lcia_method=None, classification_system=None, reference_study_period=None,
                 results=None, project_info=None, meta_data=None):
        ...

    def dumps(self) -> str:
        ...

    @classmethod
    def loads(cls, value: str) -> Self:
        ...


def convert_lcabyg(data: str, result_data: str | None = None) -> Project:
    """Converts a json formatted LCAByg project into a LCAx Project"""


def convert_ilcd(data: str) -> EPD:
    """Converts a json formatted ILCD+EPD data string into a LCAx EPD"""


def calculate_project(project: Project) -> Project:
    """
    Calculate the impact results for a Project.
    The impact results for the project will be added to the `results` property.
    """
