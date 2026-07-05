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


def test_product_subclass():
    """Test that Product can be subclassed in Python."""
    
    class CustomProduct(Product):
        """A custom subclass of Product."""
        
        def custom_method(self):
            return f"Custom Product: {self.name}"
    
    # Create an instance of the subclass using the parent class constructor
    custom_product = CustomProduct(
        name='Custom Product', 
        reference_service_life=50, 
        impact_data=[], 
        quantity=10.0, 
        unit=Unit.KG
    )
    
    # Add custom attributes after creation
    custom_product.custom_field = "product_special"
    
    # Verify it's an instance of both CustomProduct and Product
    assert isinstance(custom_product, CustomProduct)
    assert isinstance(custom_product, Product)
    
    # Verify the custom field and method work
    assert custom_product.custom_field == "product_special"
    assert custom_product.custom_method() == "Custom Product: Custom Product"
    
    # Verify inherited functionality still works
    assert custom_product.name == "Custom Product"
    assert custom_product.quantity == 10.0