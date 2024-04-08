import importlib.metadata
import uuid


def test_lcax_import():
    from lcax import LCAxProject

    assert LCAxProject


def test_lcax_project():
    from lcax import LCAxProject

    project = LCAxProject(id=str(uuid.uuid4()), name='Test', description="Test Project", location={"country": "dnk"},
                          format_version=importlib.metadata.version('lcax'), life_cycle_stages=[], impact_categories=[],
                          assemblies={}, software_info={"lca_software": "LCAx"}, project_phase="other")

    assert project
