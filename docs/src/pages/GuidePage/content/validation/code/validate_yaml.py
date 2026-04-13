from lcax import ValidationSchema, validate, Project
from pathlib import Path
import yaml

project = Project.loads(Path("project.json").read_text())
schemas = [
    ValidationSchema(**schema) for schema
    in yaml.load(Path("validation_rules.yaml").read_text())
]

result = validate(project, schemas) # result = []
