import json

import pytest

import lcax


def test_parse_ilcd(datafix_dir):
    ilcd_file = datafix_dir / "f63ac879-fa7d-4f91-813e-e816cbdf1927.json"
    epd = lcax.convert_ilcd(ilcd_file.read_text(), as_type=dict)

    assert isinstance(epd, dict)


def test_parse_ilcd_dict_input(datafix_dir):
    ilcd_file = datafix_dir / "f63ac879-fa7d-4f91-813e-e816cbdf1927.json"
    epd = lcax.convert_ilcd(json.loads(ilcd_file.read_text()))

    assert isinstance(epd, dict)


def test_parse_empty():
    with pytest.raises(lcax.ParsingException):
        lcax.convert_ilcd("{}")


def test_parse_ilcd_str(datafix_dir):
    ilcd_file = datafix_dir / "f63ac879-fa7d-4f91-813e-e816cbdf1927.json"
    epd = lcax.convert_ilcd(ilcd_file.read_text(), as_type=str)

    assert isinstance(epd, str)


def test_parse_ilcd_pydantic(datafix_dir):
    ilcd_file = datafix_dir / "f63ac879-fa7d-4f91-813e-e816cbdf1927.json"
    epd = lcax.convert_ilcd(ilcd_file.read_text(), as_type=lcax.EPD)

    assert isinstance(epd, lcax.EPD)
