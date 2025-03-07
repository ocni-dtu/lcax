import {describe, expect, expectTypeOf, it} from "vitest";
import { promises as fs } from 'fs';
import { convertLCAbyg, Project} from "../../src/lcax";

describe("convertLCAbyg", () => {
    it("should convert an LCAbyg file", async () => {
        const lcabygData = Buffer.from(await fs.readFile(`${__dirname}/datafixtures/lcabyg_project.json`)).toString()
        const result = convertLCAbyg(lcabygData) as { project: Project}
        const project = result.project
        expect(project).toBeTruthy()
        expectTypeOf(project).toEqualTypeOf<Project>()
    })

    it("should convert an LCAbyg file with results", async () => {
        const lcabygData = Buffer.from(await fs.readFile(`${__dirname}/datafixtures/lcabyg_project.json`)).toString()
        const lcabygResults = Buffer.from(await fs.readFile(`${__dirname}/datafixtures/lcabyg_project_results.json`)).toString()
        const result = convertLCAbyg(lcabygData, lcabygResults) as { project: Project}
        const project = result.project
        expect(!!project).toBeTruthy()
        expect(!!project.results).toBeTruthy()
        expectTypeOf(project).toEqualTypeOf<Project>()
    })
})