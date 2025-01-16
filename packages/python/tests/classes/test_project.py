import json
import uuid


def test_project_import():
    from lcax import Project

    assert Project


def test_project_new():
    from lcax import Project, Location, Country, ProjectPhase, SoftwareInfo

    project = Project(id=str(uuid.uuid4()), name='Test', description="Test Project", location=Location(country=Country.DNK),
                      life_cycle_stages=[], impact_categories=[],
                      assemblies=[], software_info=SoftwareInfo(lca_software="LCAx"), project_phase=ProjectPhase.OTHER)

    assert project


def test_project_loads(project_file):
    from lcax import Project

    project = Project.loads(project_file.read_text())

    assert project
    assert isinstance(project, Project)


def test_project_dumps(project):
    json_data = project.dumps()

    assert json_data
    assert isinstance(json_data, str)