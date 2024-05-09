/* tslint:disable */
/* eslint-disable */
/**
* @param {string} data
* @param {string | undefined} [resultData]
* @returns {Project}
*/
export function convertLCAbyg(data: string, resultData?: string): Project;
/**
* @param {string} data
* @returns {EPD}
*/
export function convertIlcd(data: string): EPD;
/**
* @param {Uint8Array} file
* @returns {JSProjects}
*/
export function convertSLiCE(file: Uint8Array): JSProjects;
export type JSProjects = Project[];

export type BuildingTypology = "office" | "residential" | "public" | "commercial" | "industrial" | "infrastructure" | "agricultural" | "mixeduse" | "other";

export type BuildingType = "renovation" | "new";

export interface BuildingModelScope {
    facilitating_works: boolean;
    substructure: boolean;
    superstructure_frame: boolean;
    superstructure_envelope: boolean;
    superstructure_internal_elements: boolean;
    finishes: boolean;
    building_services: boolean;
    external_works: boolean;
    ff_e: boolean;
}

export type GeneralEnergyClass = "existing" | "standard" | "advanced" | "unknown";

export type RoofType = "flat" | "pitched" | "saddle" | "pyramid" | "other";

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
    buildingTypology: BuildingTypology;
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
    buildingModelScope: BuildingModelScope | null;
}

export type ProjectInfo = { buildingInfo: BuildingInfo } | { infrastructureInfo: Record<string, string> };

export interface Location {
    country: Country;
    city: string | null;
    address: string | null;
}

export type ProjectPhase = "design" | "ongoing" | "built" | "other";

export interface SoftwareInfo {
    goalAndScopeDefinition: string | null;
    lcaSoftware: string;
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
    classificationSystem: string | null;
    referenceStudyPeriod: number | null;
    lifeCycleStages: LifeCycleStage[];
    impactCategories: ImpactCategoryKey[];
    assemblies: Record<string, Assembly>;
    results: Results;
    projectInfo: ProjectInfo | null;
    projectPhase: ProjectPhase;
    softwareInfo: SoftwareInfo;
    metaData: Record<string, string> | null;
}

export type SubType = "generic" | "specific" | "industry" | "representative";

export type Standard = "en15804a1" | "en15804a2" | "unknown";

export interface EPD {
    id: string;
    name: string;
    declaredUnit: Unit;
    version: string;
    publishedDate: NaiveDate;
    validUntil: NaiveDate;
    formatVersion: string;
    source: Source | null;
    referenceServiceLife: number | null;
    standard: Standard;
    comment: string | null;
    location: Country;
    subtype: SubType;
    conversions: Conversion[] | null;
    impacts: Record<ImpactCategoryKey, ImpactCategory>;
    metaData: Record<string, string> | null;
}

export interface InternalImpactData {
    path: string;
}

export interface ExternalImpactData {
    url: string;
    format: string;
    version: string | null;
}

export type ImpactDataSource = { EPD: EPD } | { techFlow: TechFlow } | { externalImpactData: ExternalImpactData } | { internalImpactData: InternalImpactData };

export interface Transport {
    id: string;
    name: string;
    distance: number;
    distanceUnit: Unit;
    transportEpd: ImpactDataSource;
}

export interface Product {
    id: string;
    name: string;
    description: string | null;
    referenceServiceLife: number;
    impactData: ImpactDataSource;
    quantity: number;
    unit: Unit;
    transport: Transport | null;
    results: Results;
    metaData: Record<string, string> | null;
}

export type ImpactCategoryKey = "gwp" | "gwp_fos" | "gwp_bio" | "gwp_lul" | "odp" | "ap" | "ep" | "ep_fw" | "ep_mar" | "ep_ter" | "pocp" | "adpe" | "adpf" | "penre" | "pere" | "perm" | "pert" | "penrt" | "penrm" | "sm" | "pm" | "wdp" | "irp" | "etp_fw" | "htp_c" | "htp_nc" | "sqp" | "rsf" | "nrsf" | "fw" | "hwd" | "nhwd" | "rwd" | "cru" | "mrf" | "mer" | "eee" | "eet";

export type LifeCycleStage = "a1a3" | "a4" | "a5" | "b1" | "b2" | "b3" | "b4" | "b5" | "b6" | "b7" | "c1" | "c2" | "c3" | "c4" | "d";

export interface Source {
    name: string;
    url: string | null;
}

export interface Conversion {
    value: number;
    to: Unit;
    meta_data: string;
}

export type Unit = "m" | "m2" | "m3" | "kg" | "tones" | "pcs" | "l" | "m2r1" | "km" | "tones_km" | "unknown";

export interface TechFlow {
    id: string;
    name: string;
    declaredUnit: Unit;
    formatVersion: string;
    source: Source | null;
    comment: string | null;
    location: Country;
    conversions: Conversion[] | null;
    impacts: Record<ImpactCategoryKey, ImpactCategory>;
    metaData: Record<string, string> | null;
}

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
    category: string | null;
    classification: Classification[] | null;
    products: Record<string, Product>;
    results: Results;
    metaData: Record<string, string> | null;
}

