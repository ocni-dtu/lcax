from lcax import ValidationSchema, validate, Level, ValidationRules
import pytest


def test_validate_project(project):
    schemas = [ValidationSchema(
        **{ "level": Level.PROJECT, "field": "name", "rule": ValidationRules(**{ "equal": "Test eksempel"}) }
    )]
    result = validate(project, schemas)

    assert result


def test_validate_project_fail(project):
    schemas = [ValidationSchema(
        **{ "level": Level.PROJECT, "field": "name", "rule": ValidationRules(**{ "equal": "Te eksempel"}) }
    )]

    with pytest.raises(ValueError):
        validate(project, schemas)
