from datetime import date

from lcax import Impacts


def test_epd_import():
    from lcax import EPD

    assert EPD


def test_epd_new():
    from lcax import EPD, Unit, Standard, Country, SubType

    epd = EPD(name='Test', declared_unit=Unit.M3, version="0.0.3", published_date=date.today(), valid_until=date.today(),
              standard=Standard.EN15804A2, location=Country.GBR, subtype=SubType.INDUSTRY, impacts=Impacts())

    assert epd


def test_epd_loads(epd_file):
    from lcax import EPD

    epd = EPD.loads(epd_file.read_text())

    assert epd


def test_epd_dumps(epd):
    data = epd.dumps()

    assert data
    assert isinstance(data, str)


def test_generic_data_import():
    from lcax import GenericData

    assert GenericData


def test_generic_data_new():
    from lcax import GenericData, Unit

    data = GenericData(name='Test', declared_unit=Unit.KG, comment="Test Data", impacts=Impacts())

    assert data
