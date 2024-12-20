import json

from lcax import calculate_project, Project


def test_calculate_project(datafix_dir):
    project_file = datafix_dir / "project.json"
    project = calculate_project(
        Project(**json.loads(project_file.read_text())))

    assert project.results
