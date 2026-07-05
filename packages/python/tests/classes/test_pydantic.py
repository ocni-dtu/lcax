from datetime import date
from pydantic import BaseModel, ConfigDict
from lcax import EPD, Unit, Standard, Country, Impacts, Product, SubType


def test_pydantic_integration_with_epd():
    """Test basic Pydantic integration with LCAx EPD class."""
    
    # Define a Pydantic model that represents EPD data
    class EPDModel(BaseModel):
        model_config = ConfigDict(arbitrary_types_allowed=True)
        
        name: str
        declared_unit: Unit
        version: str
        published_date: date
        valid_until: date
        standard: Standard
        location: Country
        subtype: SubType
        
        def to_epd(self) -> EPD:
            """Convert Pydantic model to LCAx EPD."""
            return EPD(
                name=self.name,
                declared_unit=self.declared_unit,
                version=self.version,
                published_date=self.published_date,
                valid_until=self.valid_until,
                standard=self.standard,
                location=self.location,
                subtype=self.subtype,
                impacts=Impacts()
            )
    
    # Create and validate EPD data using Pydantic
    epd_data = EPDModel(
        name='Test EPD',
        declared_unit=Unit.KG,
        version="1.0",
        published_date=date.today(),
        valid_until=date(2025, 12, 31),
        standard=Standard.EN15804A2,
        location=Country.GBR,
        subtype=SubType.INDUSTRY
    )
    
    # Verify Pydantic validation works
    assert epd_data.name == 'Test EPD'
    assert epd_data.version == "1.0"
    assert epd_data.declared_unit == Unit.KG
    
    # Convert to LCAx EPD
    epd = epd_data.to_epd()
    
    # Verify the conversion worked
    assert epd.name == 'Test EPD'
    assert epd.version == "1.0"
    assert epd.declared_unit == Unit.KG
    assert isinstance(epd, EPD)


def test_pydantic_integration_with_product():
    """Test basic Pydantic integration with LCAx Product class."""
    
    # Define a Pydantic model that represents Product data
    class ProductModel(BaseModel):
        model_config = ConfigDict(arbitrary_types_allowed=True)
        
        name: str
        reference_service_life: int
        quantity: float
        unit: Unit
        
        def to_product(self) -> Product:
            """Convert Pydantic model to LCAx Product."""
            return Product(
                name=self.name,
                reference_service_life=self.reference_service_life,
                impact_data=[],
                quantity=self.quantity,
                unit=self.unit
            )
    
    # Create and validate Product data using Pydantic
    product_data = ProductModel(
        name='Test Product',
        reference_service_life=50,
        quantity=10.0,
        unit=Unit.KG
    )
    
    # Verify Pydantic validation works
    assert product_data.name == 'Test Product'
    assert product_data.reference_service_life == 50
    assert product_data.quantity == 10.0
    
    # Convert to LCAx Product
    product = product_data.to_product()
    
    # Verify the conversion worked
    assert product.name == 'Test Product'
    assert product.reference_service_life == 50
    assert product.quantity == 10.0
    assert isinstance(product, Product)