import {describe, expect, it} from "vitest";
import { promises as fs } from 'fs';
import { calculateProduct, CalculationOptions } from '../../src/lcax'

describe("calculateProduct", async () => {
    const project_data = Buffer.from(await fs.readFile(`${__dirname}/datafixtures/project.json`)).toString()
    let project = JSON.parse(project_data)

    it("should calculate a product", async () => {
        const product = project.assemblies[0].products[0]
        const options = {
            referenceStudyPeriod: 50,
            lifeCycleModules: ['a1a3'], impactCategories: ['gwp'],
            overwriteExistingResults: false
        } as CalculationOptions
        const result = calculateProduct(product, options)

        expect(result.gwp.a1a3).toBe(409)
    })

    it("should calculate a product, but return empty", async () => {
        const product = project.assemblies[0].products[0]
        const options = {
            referenceStudyPeriod: 50,
            lifeCycleModules: [], impactCategories: [],
            overwriteExistingResults: false
        }
        const result = calculateProduct(product, options)

        expect(result).toStrictEqual({})
    })
})