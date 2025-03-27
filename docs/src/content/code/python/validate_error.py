from lcax import ValidationSchema, validate, Level, ValidationRule, Project
from pathlib import Path

project = Project.loads(Path("project.json").read_text())

schemas = [
    ValidationSchema(**{
        "level": Level.Project,
        "field": "name",
        "rule": ValidationRule(**{"equal": "Te eksempel"})
    })
]

result = validate(project, schemas)
# raises ValueError:
# [{"name":[{"kind":"Field is not equal to: Te eksempel"}]}]
