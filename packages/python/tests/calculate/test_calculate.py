from lcax import calculate_project


def test_calculate_project(project):
    project = calculate_project(project)

    assert project.results
