import { validate, ValidationSchema } from 'lcax'
import YAML from 'yaml'
import fs from 'fs'

const project_data = fs.readFileSync('project.json')
const project = JSON.parse(project_data)

const validation_rules = fs.readFileSync('validation_rules.yaml')
const schemas: ValidationSchema[] = YAML.parse(validation_rules)

const result = validate(project, schemas) // result = []
