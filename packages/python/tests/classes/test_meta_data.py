def test_meta_data_access(meta_data):
    assert meta_data
    assert meta_data["unit_description"] == "kilograms per cubic metre"


def test_meta_data_str(meta_data):
    assert str(meta_data)
