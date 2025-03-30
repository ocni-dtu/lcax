from datetime import date

from lcax import Impacts


def test_product_import():
    from lcax import Product

    assert Product


def test_product_new():
    from lcax import Product, Unit, ImpactData, Standard, Country, SubType

    product = Product(
        name='Test',
        reference_service_life=50,
        impact_data=[ImpactData(
            _type="epd",
            name="EPD",
            declared_unit=Unit.M2,
            version="0.0.3",
            published_date=date.today(),
            valid_until=date.today(),
            standard=Standard.EN15804A2,
            location=Country.FRA,
            subtype=SubType.INDUSTRY,
            impacts=Impacts()
        )],
        quantity=1.0,
        unit=Unit.KG,
    )

    assert product
