from lcax import Country
import pytest

@pytest.mark.parametrize("value, expected", [
    ("AT", Country.AUT),
    ("AUT", Country.AUT),
    ("at", Country.AUT),
    ("aut", Country.AUT),
    ("Invalid", Country.UNKNOWN),
    ("XX", Country.UNKNOWN),
])
def test_country_from_string(value, expected):
    # Alpha-2
    assert Country.from_string(value) == expected


def test_country_name():
    assert Country.AUT.name() == "Austria"