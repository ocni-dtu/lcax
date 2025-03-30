import {describe, expect, it} from "vitest";
import {validate, ValidationRule, ValidationSchema} from "../../src/lcax";
import {promises as fs} from "fs";

describe("Validate", () => {
    it("Successfully validate a project", async () => {
        const project_data = Buffer.from(await fs.readFile(`${__dirname}/datafixtures/project.json`)).toString()
        const project = JSON.parse(project_data)
        const schemas: ValidationSchema[] = [{
            level: 'project',
            field: 'name',
            rule: {equal: "Test eksempel"} as ValidationRule
        }]
        const result = validate(project, schemas)

        expect(result).toStrictEqual([])
    })

    it("It should fail validating the project", async () => {
        const project_data = Buffer.from(await fs.readFile(`${__dirname}/datafixtures/project.json`)).toString()
        const project = JSON.parse(project_data)
        const schemas: ValidationSchema[] = [{
            level: 'project', field: 'name', rule: {
                equal: "Te eksempel",
            } as ValidationRule
        }]
        const result = validate(project, schemas)

        expect(result.length).toBe(1)
        expect(result[0].field).toBe('name')
        expect(result[0].message).toBe('Field is not equal to: Te eksempel')
    })
})
