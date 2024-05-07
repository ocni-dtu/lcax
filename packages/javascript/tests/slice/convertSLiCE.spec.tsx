import {describe, expect, expectTypeOf, it} from "vitest";
import { convertSLiCE, Project} from "../../src/lcax";
import {readFileSync} from "node:fs";
import { webcrypto } from 'node:crypto'
// @ts-ignore
globalThis.crypto = webcrypto

describe("convertSLiCE", () => {
    it("should convert a SLiCE file", async () => {
        const sliceFile = new Uint8Array(readFileSync(`${__dirname}/datafixtures/results_slice_WLCR.parquet`))
        const project = convertSLiCE(sliceFile)

        expect(project).toBeTruthy()
        expectTypeOf(project).toEqualTypeOf<Project[]>()
    })
})