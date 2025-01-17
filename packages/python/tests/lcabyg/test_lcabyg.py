import pytest

import lcax


def test_lcabyg_parse_project(datafix_dir):
    lcabyg_file = datafix_dir / "lcabyg_project.json"
    project = lcax.convert_lcabyg(lcabyg_file.read_text())

    assert isinstance(project, lcax.Project)


def test_parse_empty():
    with pytest.raises(TypeError):
        lcax.convert_lcabyg("{}")
