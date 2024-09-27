import {describe, expect, it} from "vitest";
import { promises as fs } from 'fs';
import { calculateProject } from "../../src/lcax";

describe("calculateProject", () => {
    it("should calculate a project", async () => {
        const project_data = Buffer.from(await fs.readFile(`${__dirname}/datafixtures/project.json`)).toString()
        let project = JSON.parse(project_data)
        project = calculateProject(project)

        expect(project.results).toBeTruthy()
    })
})