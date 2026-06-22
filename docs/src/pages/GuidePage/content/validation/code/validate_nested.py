from lcax import ValidationSchema, validate, Level, ValidationRule, Project
from pathlib import Path

project = Project.loads(Path("project.json").read_text())

schemas = [
    ValidationSchema(**{
        "level": Level.Project,
        "field": "location.country",
        "rule": ValidationRule(**{"equal": "dnk"})
    }),
    ValidationSchema(**{
        "level": Level.Project,
        "field": "projectInfo?.grossFloorArea?.value",
        "rule": ValidationRule(**{"greater": 50.0})
    })
]

result = validate(project, schemas) # result = []

