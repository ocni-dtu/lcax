import {validate, ValidationRules, ValidationSchema} from "lcax";
import {promises as fs} from "fs";


const project_data = Buffer.from(await fs.readFile(`${__dirname}/datafixtures/project.json`)).toString()
const project = JSON.parse(project_data)
const schemas: ValidationSchema[] = [{
    level: 'project',
    field: 'name',
    rule: {equal: "Test eksempel"} as ValidationRules
}]
const result = validate(project, schemas)
    