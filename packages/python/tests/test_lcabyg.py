import lcax


def test_lcabyg_parse_project_dict(datafix_dir):
    lcabyg_file = datafix_dir / "lcabyg_project.json"
    project = lcax.convert_lcabyg(lcabyg_file.read_text())

    assert isinstance(project, dict)


def test_lcabyg_parse_project_str(datafix_dir):
    lcabyg_file = datafix_dir / "lcabyg_project.json"
    project = lcax.convert_lcabyg(lcabyg_file.read_text(), as_type=str)

    assert isinstance(project, str)


def test_lcabyg_parse_project_pydantic(datafix_dir):
    lcabyg_file = datafix_dir / "lcabyg_project.json"
    project = lcax.convert_lcabyg(lcabyg_file.read_text(), as_type=lcax.LCAxProject)

    assert isinstance(project, lcax.LCAxProject)
