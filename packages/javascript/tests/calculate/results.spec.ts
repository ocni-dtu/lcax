import {describe, expect, it} from "vitest";
import {getImpactTotal, normalizeResult, getImpactsByLifeCycleModule, Impacts} from "../../src/lcax";

describe("getImpactTotal", () => {
    it("Get total from impacts", async () => {
        const result = getImpactTotal({gwp: {a1a3: 10, d: -2}} as Impacts, 'gwp', ['d'])

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
    it("Get value", async () => {
        const result = getImpactsByLifeCycleModule({gwp: {a1a3: 10}} as Impacts, 'gwp', undefined, 2)

        expect(result.a1a3).toBe(5)
    })
})