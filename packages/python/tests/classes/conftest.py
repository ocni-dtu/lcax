import pytest

from lcax import Product, Unit, Assembly


@pytest.fixture
def epd_file(datafix_dir):
    yield datafix_dir / "epd.json"


@pytest.fixture
def epd(epd_file):
    from lcax import EPD

    yield EPD.loads(epd_file.read_text())


@pytest.fixture
def impacts(epd):
    yield epd.impacts


@pytest.fixture
def conversion(epd):
    yield epd.conversions[0]


@pytest.fixture
def source(epd):
    yield epd.source


@pytest.fixture
def meta_data(epd):
    yield epd.conversions[0].meta_data


@pytest.fixture
def product(epd):
    product = Product(
        name='Test',
        reference_service_life=50,
        impact_data=[epd],
        quantity=1.0,
        unit=Unit.KG,
    )

    yield product

@pytest.fixture
def assembly(product):
    assembly = Assembly(name='Test', description="Test Assembly", quantity=4.0, unit=Unit.M, products=[product])

    yield assembly