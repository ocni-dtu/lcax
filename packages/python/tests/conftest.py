import pytest

@pytest.fixture
def project_file(datafix_dir):
    yield datafix_dir / "project.json"


@pytest.fixture
def project(project_file):
    from lcax import Project

    yield Project.loads(project_file.read_text())


@pytest.fixture
def assembly(project):
    yield project.assemblies[0]


@pytest.fixture
def product(project):
    yield project.assemblies[0].products[0]