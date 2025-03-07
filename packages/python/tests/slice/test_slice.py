import lcax


def test_slice_parse_project_dict(datafix_dir):
    slice_file = datafix_dir / "slice.parquet"
    projects = lcax.convert_slice(slice_file)

    assert isinstance(projects[0], dict)
