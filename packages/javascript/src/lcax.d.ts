/* tslint:disable */
/* eslint-disable */
/**
* @param {string} data
* @param {string | undefined} resultData
* @returns {LCAxProject}
*/
export function convertLCAbyg(data: string, resultData?: string): LCAxProject;
export interface Classification {
    system: string;
    code: string;
    name: string;
}

export interface InternalImpactData {
    path: string;
}

export interface ExternalImpactData {
    url: string;
    format: string;
    version: string | null;
}

export type ImpactDataSource = { epd: EPD } | { externalimpactdata: ExternalImpactData } | { internalimpactdata: InternalImpactData };

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
    description: string;
    referenceServiceLife: number;
    impactData: ImpactDataSource;
    quantity: number;
    unit: Unit;
    transport: Transport | null;
    results: Results;
    metaData: Record<string, string> | null;
}

export interface Assembly {
    id: string;
    name: string;
    description: string;
    comment: string | null;
    quantity: number;
    unit: Unit;
    category: string | null;
    classification: Classification[] | null;
    products: Record<string, Product>;
    results: Results;
    metaData: Record<string, string> | null;
}

export type ImpactCategoryKey = "gwp" | "odp" | "ap" | "ep" | "pocp" | "adpe" | "adpf" | "penre" | "pere" | "perm" | "pert" | "penrt" | "penrm" | "sm" | "rsf" | "nrsf" | "fw" | "hwd" | "nhwd" | "rwd" | "cru" | "mrf" | "mer" | "eee" | "eet";

export type LifeCycleStage = "a1a3" | "a4" | "a5" | "b1" | "b2" | "b3" | "b4" | "b5" | "b6" | "b7" | "c1" | "c2" | "c3" | "c4" | "d";

export type BuildingTypology = "office" | "residential" | "public" | "commercial" | "industrial" | "infrastructure" | "agricultural" | "other";

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

export interface BuildingInfo {
    buildingType: BuildingType;
    buildingTypology: BuildingTypology;
    certifications: string;
    buildingMass: string;
    grossFloorArea: number;
    grossFloorAreaDefinition: string;
    heatedFloorArea: number;
    heatedFloorAreaDefinition: string;
    floorsAboveGround: number;
    floorsBelowGround: number;
    frameType: string;
    buildingCompletionYear: number;
    energyDemandHeating: number;
    energySupplyHeating: number;
    energyDemandElectricity: number;
    energySupplyElectricity: number;
    exportedElectricity: number;
    energyClass: string;
    buildingModelScope: BuildingModelScope | null;
}

export type ProjectInfo = { buildinginfo: BuildingInfo } | { infrastructureinfo: Record<string, string> };

export interface Location {
    country: string;
    city: string | null;
    address: string | null;
}

export type ProjectPhase = "design" | "ongoing" | "built" | "other";

export interface SoftwareInfo {
    goalAndScopeDefinition: string | null;
    lcaSoftware: string;
    calculationType: string | null;
}

export interface LCAxProject {
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

