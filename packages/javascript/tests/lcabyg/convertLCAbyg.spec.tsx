import {describe, expect, expectTypeOf, it} from "vitest";
import { promises as fs } from 'fs';
import {convertLCAbyg, EPD, Project, toLCAbyg} from "../../src/lcax";

describe("convertLCAbyg", () => {
    it("should convert an LCAbyg file", async () => {
        const lcabygData = Buffer.from(await fs.readFile(`${__dirname}/datafixtures/lcabyg_project.json`)).toString()
        const project = convertLCAbyg(lcabygData) as Project
        expect(project).toBeTruthy()
        expect(project.id).toEqual("e9e6e798-390e-4419-a1fa-3b46a8ba5b8d")
        expectTypeOf(project).toEqualTypeOf<Project>()
    })

    it("should convert an LCAbyg file with results", async () => {
        const lcabygData = Buffer.from(await fs.readFile(`${__dirname}/datafixtures/lcabyg_project.json`)).toString()
        const lcabygResults = Buffer.from(await fs.readFile(`${__dirname}/datafixtures/lcabyg_project_results.json`)).toString()
        const project = convertLCAbyg(lcabygData, lcabygResults) as Project
        expect(!!project).toBeTruthy()
        expect(!!project.results).toBeTruthy()
        expectTypeOf(project).toEqualTypeOf<Project>()
    })

    it("should convert LCAbyg stages", async () => {
        const lcabygData = Buffer.from(await fs.readFile(`${__dirname}/datafixtures/stages.json`)).toString()
        const epds = convertLCAbyg(lcabygData) as EPD[]
        expect(epds).toBeTruthy()
        expectTypeOf(epds).toEqualTypeOf<EPD[]>()
    })
})

describe("toLCAbyg", () => {
    it("should convert a LCAx EPD to LCAbyg", async () => {
        const epdData = JSON.parse(Buffer.from(await fs.readFile(`${__dirname}/datafixtures/epd.json`)).toString())
        const result = toLCAbyg([epdData])
        expect(result).toBeTruthy()
    })
})