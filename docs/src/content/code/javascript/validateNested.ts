import {validate, ValidationSchema} from "lcax";
import fs from 'fs'


const project_data =  fs.readFileSync('project.json')
const project = JSON.parse(project_data)
const schemas: ValidationSchema[] = [
    {
        level: 'project',
        field: 'location.country',
        rule: {equal: "dnk"}
    },
    {
        level: 'project',
        field: 'projectInfo?.grossFloorArea?.value',
        rule: {greater: 50.0}
    }
]

const result = validate(project, schemas) // result = []
