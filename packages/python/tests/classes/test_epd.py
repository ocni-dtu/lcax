from datetime import date

from lcax import EPD, Unit, Standard, Country, SubType, Impacts, GenericData, LifeCycleModule, ImpactCategoryKey


def test_epd_import():
    assert EPD


def test_epd_new():
    epd = EPD(name='Test', declared_unit=Unit.M3, version="0.0.3", published_date=date.today(),
              valid_until=date.today(),
              standard=Standard.EN15804A2, location=Country.GBR, subtype=SubType.INDUSTRY, impacts=Impacts())

    assert epd
    assert str(epd) == f"EPD: {epd.id}"


def test_epd_loads(epd_file):
    epd = EPD.loads(epd_file.read_text())

    assert epd


def test_epd_dumps(epd):
    data = epd.dumps()

    assert data
    assert isinstance(data, str)


def test_generic_data_import():
    assert GenericData


def test_generic_data_new():
    data = GenericData(name='Test', declared_unit=Unit.KG, comment="Test Data", impacts=Impacts())

    assert data

