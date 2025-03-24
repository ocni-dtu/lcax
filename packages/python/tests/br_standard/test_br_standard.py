import lcax


def test_br_standard(datafix_dir):
    br_file = datafix_dir / "Traditionelt_Etagehus.xlsx"
    project = lcax.convert_br_standard(br_file)

    assert isinstance(project, lcax.Project)