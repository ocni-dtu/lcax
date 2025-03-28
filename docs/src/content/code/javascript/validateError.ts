import {validate, ValidationRule, ValidationSchema} from "lcax";
import fs from 'fs'


const project_data =  fs.readFileSync('project.json')
const project = JSON.parse(project_data)
const schemas: ValidationSchema[] = [{
    level: 'project',
    field: 'name',
    rule: {equal: "Te eksempel"} as ValidationRule
}]

const result = validate(project, schemas)
// result = [{ field: "name", message: "Field is not equal to: Te eksempel" }]
