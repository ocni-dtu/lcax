import {describe, expect, it} from "vitest";
import {getImpactTotal, normalizeResult, getImpactsByLifeCycleModule} from "../../src/lcax";

describe("getImpactTotal", () => {
    it("Get total from impacts", async () => {
        const result = getImpactTotal({gwp: {a1a3: 10}}, 'gwp', undefined)

        expect(result).toBe(10)
    })
})

describe("normalizeResult", () => {
    it("Normalize result", async () => {
        const result = normalizeResult(10, 2)

        expect(result).toBe(5)
    })
})

describe("getImpactsByLifeCycleModule", () => {
    it("Normalize result", async () => {
        const result = getImpactsByLifeCycleModule({gwp: {a1a3: 10}}, 'gwp', undefined, 2)

        expect(result.a1a3).toBe(5)
    })
})