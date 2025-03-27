from lcax import ValidationSchema, validate, Level, ValidationRule, Project
from pathlib import Path

project = Project.loads(Path("project.json").read_text())

schemas = [
    ValidationSchema(**{
        "level": Level.Project,
        "field": "name",
        "rule": ValidationRule(**{"equal": "Test eksempel"})
    })
]

result = validate(project, schemas) # result = true

