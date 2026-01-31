/* @ts-self-types="./lcax.d.ts" */

import * as wasm from "./lcax_bg.wasm";
import { __wbg_set_wasm } from "./lcax_bg.js";
__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
export {
    buildingModelScopes, buildingTypes, buildingTypologies, calculateAssembly, calculateProduct, calculateProject, convertBRStandard, convertIlcd, convertLCAbyg, countries, generalEnergyClasses, getImpactTotal, getImpactsByLifeCycleModule, impactCategories, lifeCycleModules, normalizeResult, projectPhases, roofTypes, standards, subTypes, toLCAbyg, units, validate
} from "./lcax_bg.js";
