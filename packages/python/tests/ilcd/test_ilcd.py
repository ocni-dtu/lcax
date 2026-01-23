import datetime

import pytest

import lcax
from lcax import Impacts


def test_parse_ilcd(datafix_dir):
    ilcd_file = datafix_dir / "f63ac879-fa7d-4f91-813e-e816cbdf1927.json"
    epd = lcax.convert_ilcd(ilcd_file.read_text())

    assert epd
    assert isinstance(epd, lcax.EPD)
    assert epd.id == "f63ac879-fa7d-4f91-813e-e816cbdf1927"
    assert epd.name
    assert epd.declared_unit
    assert isinstance(epd.declared_unit, lcax.Unit)
    assert epd.version
    assert epd.published_date
    assert isinstance(epd.published_date, datetime.date)
    assert epd.valid_until
    assert isinstance(epd.valid_until, datetime.date)
    assert hasattr(epd, "source")
    assert hasattr(epd, "reference_service_life")
    assert epd.standard
    assert isinstance(epd.standard, lcax.Standard)
    assert hasattr(epd, "comment")
    assert epd.location
    assert isinstance(epd.location, lcax.Country)
    assert epd.conversions
    assert epd.impacts
    assert isinstance(epd.impacts, Impacts)
    assert hasattr(epd, "meta_data")


def test_parse_empty():
    with pytest.raises(TypeError):
        lcax.convert_ilcd("{}")
