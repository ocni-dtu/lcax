import {describe, expect, it} from "vitest";
import {validate, ValidationRules, ValidationSchema} from "../../src/lcax";
import {promises as fs} from "fs";

describe("Validate", () => {
    it("Successfully validate a project", async () => {
        const project_data = Buffer.from(await fs.readFile(`${__dirname}/datafixtures/project.json`)).toString()
        const project = JSON.parse(project_data)
        const schemas: ValidationSchema[] = [{
            level: 'project',
            field: 'name',
            rule: {equal: "Test eksempel"} as ValidationRules
        }]
        const result = validate(project, schemas)

        expect(result).toBe(true)
    })

    it("It should fail validating the project", async () => {
        const project_data = Buffer.from(await fs.readFile(`${__dirname}/datafixtures/project.json`)).toString()
        const project = JSON.parse(project_data)
        const schemas: ValidationSchema[] = [{
            level: 'project', field: 'name', rule: {
                equal: "Te eksempel",
            } as ValidationRules
        }]

        expect(() => validate(project, schemas)).toThrowError()
    })
})
