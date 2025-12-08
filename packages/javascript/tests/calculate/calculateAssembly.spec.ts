import {describe, expect, it} from "vitest";
import { promises as fs } from 'fs';
import { calculateAssembly, CalculationOptions } from '../../src/lcax'

describe("calculateAssembly", async () => {
    const project_data = Buffer.from(await fs.readFile(`${__dirname}/datafixtures/project.json`)).toString()
    let project = JSON.parse(project_data)

    it("should calculate an assembly", async () => {
        const assembly = project.assemblies[0]
        const options = {
            referenceStudyPeriod: 50,
            lifeCycleModules: ['a1a3'], impactCategories: ['gwp'],
            overwriteExistingResults: false
        } as CalculationOptions
        const result = calculateAssembly(assembly, options)

        expect(result.gwp.a1a3).toBe(20450)
    })

    it("should calculate an assembly, but return empty", async () => {
        const assembly = project.assemblies[0]
        const options = {
            referenceStudyPeriod: 50,
            lifeCycleModules: [], impactCategories: [],
            overwriteExistingResults: false
        }
        const result = calculateAssembly(assembly, options)

        expect(result).toStrictEqual({})
    })
})