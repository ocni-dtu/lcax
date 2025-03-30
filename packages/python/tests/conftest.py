import pytest

@pytest.fixture
def project_file(datafix_dir):
    yield datafix_dir / "project.json"


@pytest.fixture
def project(project_file):
    from lcax import Project

    yield Project.loads(project_file.read_text())
