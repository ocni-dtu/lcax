/* tslint:disable */
/* eslint-disable */
/**
 * Converts a json formatted LCAByg project into a LCAx Project
 */
export function convertLCAbyg(data: string, resultData?: string | null): LCABygResult;
/**
 * Converts a BR Standard Format file into a LCAx `Project`.
 */
export function convertBRStandard(project_name: string, file: Uint8Array): Project;
/**
 * Converts a json formatted ILCD+EPD data string into a LCAx EPD
 */
export function convertIlcd(data: string): EPD;
/**
 * Calculate the impact results for a Project.
 * The impact results for the project will be added to the `results` property.
 */
export function calculateProject(project: Project): Project;
/**
 * Get the total impact
 */
export function getImpactTotal(impacts: Impacts, category: ImpactCategoryKey, exclude_modules?: LifeCycleModule[] | null): number;
/**
 * Normalize a result with e.g. the reference study period and gross floor area
 */
export function normalizeResult(result: number, normalizing_factor: number): number;
/**
 * Get the impacts by life cycle module.
 * The results can be normalized by a factor.
 */
export function getImpactsByLifeCycleModule(impacts: Impacts, category: ImpactCategoryKey, exclude_modules?: LifeCycleModule[] | null, normalizing_factor?: number | null): ImpactCategory | undefined;
/**
 * Validate a LCAx Project
 */
export function validate(project: Project, validation_schemas: ValidationSchema[]): boolean;
export interface ValidationRules {
    range: [number, number] | null;
    includes: string | null;
    required: boolean | null;
    equal: string | null;
    greater: number | null;
    less: number | null;
    oneOf: string[] | null;
}

export interface ValidationSchema {
    level: Level;
    field: string;
    rule: ValidationRules;
}

export type Level = "project" | "assembly" | "product" | "impact_data";

export type LCABygResult = { project: Project } | { assemblies: Assembly[] } | { products: Product[] } | { epds: EPD[] };

export type LifeCycleModule = "a0" | "a1a3" | "a4" | "a5" | "b1" | "b2" | "b3" | "b4" | "b5" | "b6" | "b7" | "b8" | "c1" | "c2" | "c3" | "c4" | "d";

export type ImpactCategoryKey = "gwp" | "gwp_fos" | "gwp_bio" | "gwp_lul" | "odp" | "ap" | "ep" | "ep_fw" | "ep_mar" | "ep_ter" | "pocp" | "adpe" | "adpf" | "penre" | "pere" | "perm" | "pert" | "penrt" | "penrm" | "sm" | "pm" | "wdp" | "irp" | "etp_fw" | "htp_c" | "htp_nc" | "sqp" | "rsf" | "nrsf" | "fw" | "hwd" | "nhwd" | "rwd" | "cru" | "mrf" | "mer" | "eee" | "eet";

export type ImpactCategory = Record<LifeCycleModule, number | null>;

export type Impacts = Record<ImpactCategoryKey, ImpactCategory>;

export type BuildingTypology = "office" | "residential" | "public" | "commercial" | "industrial" | "infrastructure" | "agricultural" | "educational" | "health" | "unknown" | "other";

export type BuildingType = "new_construction_works" | "demolition" | "deconstruction_and_new_construction_works" | "retrofit_works" | "extension_works" | "retrofit_and_extension_works" | "fit_out_works" | "operations" | "unknown" | "other";

export type BuildingModelScope = "facilitating_works" | "substructure" | "superstructure_frame" | "superstructure_envelope" | "superstructure_internal_elements" | "finishes" | "building_services" | "external_works" | "ff_e";

export type GeneralEnergyClass = "existing" | "standard" | "advanced" | "unknown";

export type RoofType = "flat" | "pitched" | "saddle" | "pyramid" | "unknown" | "other";

export interface ValueUnit {
    value: number;
    unit: Unit;
}

export interface AreaType {
    value: number;
    unit: Unit;
    definition: string;
}

export interface BuildingInfo {
    buildingType: BuildingType;
    buildingTypology: BuildingTypology[];
    certifications: string[] | null;
    buildingMass: ValueUnit | null;
    buildingHeight: ValueUnit | null;
    grossFloorArea: AreaType | null;
    heatedFloorArea: AreaType | null;
    buildingFootprint: ValueUnit | null;
    floorsAboveGround: number;
    floorsBelowGround: number | null;
    roofType: RoofType;
    frameType: string | null;
    buildingCompletionYear: number | null;
    buildingPermitYear: number | null;
    energyDemandHeating: number | null;
    energySupplyHeating: number | null;
    energyDemandElectricity: number | null;
    energySupplyElectricity: number | null;
    exportedElectricity: number | null;
    generalEnergyClass: GeneralEnergyClass;
    localEnergyClass: string | null;
    buildingUsers: number | null;
    buildingModelScope: BuildingModelScope[] | null;
}

export interface Location {
    country: Country;
    city: string | null;
    address: string | null;
}

export type ProjectPhase = "strategic_design" | "concept_design" | "technical_design" | "construction" | "post_completion" | "in_use" | "other";

export interface SoftwareInfo {
    lcaSoftware: string;
    lcaSoftwareVersion: string | null;
    goalAndScopeDefinition: string | null;
    calculationType: string | null;
}

export interface Project {
    id: string;
    name: string;
    description: string | null;
    comment: string | null;
    location: Location;
    owner: string | null;
    formatVersion: string;
    lciaMethod: string | null;
    classificationSystems: string[] | null;
    referenceStudyPeriod: number | null;
    lifeCycleModules: LifeCycleModule[];
    impactCategories: ImpactCategoryKey[];
    assemblies: AssemblyReference[];
    results: Impacts | null;
    projectInfo: BuildingInfo | null;
    projectPhase: ProjectPhase;
    softwareInfo: SoftwareInfo;
    metaData: MetaData | null;
}

export type MetaData = Record<string, AnyValue>;

export interface Reference {
    uri: string;
    format: string | null;
    version: string | null;
    overrides: Record<string, AnyValue> | null;
}

export interface Source {
    name: string;
    url: string | null;
}

export interface Conversion {
    value: number;
    to: Unit;
    metaData: MetaData | null;
}

export type Unit = "m" | "m2" | "m3" | "kg" | "tones" | "pcs" | "kwh" | "l" | "m2r1" | "km" | "tones_km" | "kgm3" | "unknown";

export type GenericDataReference = ({ type: "EPD" } & GenericData) | ({ type: "reference" } & Reference);

export interface GenericData {
    id: string;
    name: string;
    declaredUnit: Unit;
    formatVersion: string;
    source: Source | null;
    comment: string | null;
    conversions: Conversion[] | null;
    impacts: Impacts;
    metaData: MetaData | null;
}

export type EPDReference = ({ type: "EPD" } & EPD) | ({ type: "reference" } & Reference);

export type SubType = "generic" | "specific" | "industry" | "representative";

export type Standard = "en15804a1" | "en15804a2" | "unknown";

export interface EPD {
    id: string;
    name: string;
    declaredUnit: Unit;
    version: string;
    publishedDate: Date;
    validUntil: Date;
    formatVersion: string;
    source: Source | null;
    referenceServiceLife: number | null;
    standard: Standard;
    comment: string | null;
    location: Country;
    subtype: SubType;
    conversions: Conversion[] | null;
    impacts: Impacts;
    metaData: MetaData | null;
}

export type ImpactData = EPDReference | GenericDataReference;

export interface Transport {
    id: string;
    name: string;
    lifeCycleModules: LifeCycleModule[];
    distance: number;
    distanceUnit: Unit;
    impactData: ImpactData;
}

export type ProductReference = ({ type: "product" } & Product) | ({ type: "reference" } & Reference);

export interface Product {
    id: string;
    name: string;
    description: string | null;
    referenceServiceLife: number;
    impactData: ImpactData[];
    quantity: number;
    unit: Unit;
    transport: Transport[] | null;
    results: Impacts | null;
    metaData: MetaData | null;
}

export type AssemblyReference = ({ type: "assembly" } & Assembly) | ({ type: "reference" } & Reference);

export interface Classification {
    system: string;
    code: string;
    name: string;
}

export interface Assembly {
    id: string;
    name: string;
    description: string | null;
    comment: string | null;
    quantity: number;
    unit: Unit;
    classification: Classification[] | null;
    products: ProductReference[];
    results: Impacts | null;
    metaData: MetaData | null;
}

export type Country = "unknown" | "afg" | "ala" | "alb" | "dza" | "asm" | "and" | "ago" | "aia" | "ata" | "atg" | "arg" | "arm" | "abw" | "aus" | "aut" | "aze" | "bhs" | "bhr" | "bgd" | "brb" | "blr" | "bel" | "blz" | "ben" | "bmu" | "btn" | "bol" | "bes" | "bih" | "bwa" | "bvt" | "bra" | "iot" | "brn" | "bgr" | "bfa" | "bdi" | "cpv" | "khm" | "cmr" | "can" | "cym" | "caf" | "tcd" | "chl" | "chn" | "cxr" | "cck" | "col" | "com" | "cog" | "cod" | "cok" | "cri" | "civ" | "hrv" | "cub" | "cuw" | "cyp" | "cze" | "dnk" | "dji" | "dma" | "dom" | "ecu" | "egy" | "slv" | "gnq" | "eri" | "est" | "swz" | "eth" | "flk" | "fro" | "fji" | "fin" | "fra" | "guf" | "pyf" | "atf" | "gab" | "gmb" | "geo" | "deu" | "gha" | "gib" | "grc" | "grl" | "grd" | "glp" | "gum" | "gtm" | "ggy" | "gin" | "gnb" | "guy" | "hti" | "hmd" | "vat" | "hnd" | "hkg" | "hun" | "isl" | "ind" | "idn" | "irn" | "irq" | "irl" | "imn" | "isr" | "ita" | "jam" | "jpn" | "jey" | "jor" | "kaz" | "ken" | "kir" | "prk" | "kor" | "kwt" | "kgz" | "lao" | "lva" | "lbn" | "lso" | "lbr" | "lby" | "lie" | "ltu" | "lux" | "mac" | "mdg" | "mwi" | "mys" | "mdv" | "mli" | "mlt" | "mhl" | "mtq" | "mrt" | "mus" | "myt" | "mex" | "fsm" | "mda" | "mco" | "mng" | "mne" | "msr" | "mar" | "moz" | "mmr" | "nam" | "nru" | "npl" | "nld" | "ncl" | "nzl" | "nic" | "ner" | "nga" | "niu" | "nfk" | "mkd" | "mnp" | "nor" | "omn" | "pak" | "plw" | "pse" | "pan" | "png" | "pry" | "per" | "phl" | "pcn" | "pol" | "prt" | "pri" | "qat" | "reu" | "rou" | "rus" | "rwa" | "blm" | "shn" | "kna" | "lca" | "maf" | "spm" | "vct" | "wsm" | "smr" | "stp" | "sau" | "sen" | "srb" | "syc" | "sle" | "sgp" | "sxm" | "svk" | "svn" | "slb" | "som" | "zaf" | "sgs" | "ssd" | "esp" | "lka" | "sdn" | "sur" | "sjm" | "swe" | "che" | "syr" | "twn" | "tjk" | "tza" | "tha" | "tls" | "tgo" | "tkl" | "ton" | "tto" | "tun" | "tur" | "tkm" | "tca" | "tuv" | "uga" | "ukr" | "are" | "gbr" | "usa" | "umi" | "ury" | "uzb" | "vut" | "ven" | "vnm" | "vgb" | "vir" | "wlf" | "esh" | "yem" | "zmb" | "zwe";

export type Number = { int: number } | { float: number };

export type AnyValue = { null: [] } | { bool: boolean } | { number: Number } | { string: string } | { array: AnyValue[] } | { object: Record<string, AnyValue> };

