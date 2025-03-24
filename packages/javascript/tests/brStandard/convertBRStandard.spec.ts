import {describe, expect, expectTypeOf, it} from "vitest";
import { convertBRStandard, Project} from "../../src/lcax";
import {readFileSync} from "node:fs";

describe("convertBRStandard", () => {
    it("should convert a BR Standard Format file", async () => {
        const xlsxFile = new Uint8Array(readFileSync(`${__dirname}/datafixtures/Traditionelt_Etagehus.xlsx`))
        const project = convertBRStandard("Traditionelt_Etagehus", xlsxFile)

        expect(project).toBeTruthy()
        expectTypeOf(project).toEqualTypeOf<Project>()
    })
})