from lcax import ValidationSchema, validate, Level, ValidationRule


def test_validate_project(project):
    schemas = [ValidationSchema(
        **{ "level": Level.Project, "field": "name", "rule": ValidationRule(**{ "equal": "Test eksempel"}) }
    )]
    result = validate(project, schemas)

    assert len(result) == 0


def test_validate_project_fail(project):
    schemas = [ValidationSchema(
        **{ "level": Level.Project, "field": "name", "rule": ValidationRule(**{ "equal": "Te eksempel"}) }
    )]

    result = validate(project, schemas)
    assert len(result) == 1
    assert result[0].field == "name"
    assert result[0].message == "Field is not equal to: Te eksempel"
