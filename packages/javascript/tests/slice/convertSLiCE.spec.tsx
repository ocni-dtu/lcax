import {describe, expect, expectTypeOf, it} from "vitest";
import { convertSLiCE, Project} from "../../src/lcax";
import {readFileSync} from "node:fs";

describe("convertSLiCE", () => {
    it("should convert a SLiCE file", async () => {
        const sliceFile = new Uint8Array(readFileSync(`${__dirname}/datafixtures/slice.parquet`))
        const project = convertSLiCE(sliceFile)

        expect(project).toBeTruthy()
        expectTypeOf(project).toEqualTypeOf<Project[]>()
    })
})