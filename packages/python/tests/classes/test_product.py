from datetime import date

from lcax import Product, Unit, Standard, Country, SubType, Impacts, EPD, GenericData


def test_product_import():
    assert Product


def test_product_new():

    product = Product(
        name='Test',
        reference_service_life=50,
        impact_data=[EPD(
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

def test_product_generic_new():

    product = Product(
        name='Test',
        reference_service_life=50,
        impact_data=[GenericData(
            name="EPD",
            declared_unit=Unit.M2,
            impacts=Impacts()
        )],
        quantity=1.0,
        unit=Unit.KG,
    )

    assert product

def test_product_str(product):
    assert str(product) == f"Product: {product.id}"