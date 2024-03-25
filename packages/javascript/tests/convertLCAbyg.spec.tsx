import {describe, expect, expectTypeOf, it} from "vitest";
import { promises as fs } from 'fs';
import { convertLCAbyg, LCAxProject} from "../src/lcax";

describe("convertLCAbyg", () => {
    it("should convert an LCAbyg file", async () => {
        const lcabygData = Buffer.from(await fs.readFile(`${__dirname}/datafixtures/lcabyg_project.json`)).toString()
        const project = convertLCAbyg(lcabygData)

        expect(project).toBeTruthy()
        expectTypeOf(project).toEqualTypeOf<LCAxProject>()
    })
})