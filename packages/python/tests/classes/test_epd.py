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


def test_epd_subclass():
    """Test that EPD can be subclassed in Python."""
    
    class CustomEPD(EPD):
        """A custom subclass of EPD."""
        
        def custom_method(self):
            return f"Custom EPD: {self.name}"
    
    # Create an instance of the subclass using the parent class constructor
    custom_epd = CustomEPD(
        name='Custom Test', 
        declared_unit=Unit.M3, 
        version="0.0.3", 
        published_date=date.today(),
        valid_until=date.today(),
        standard=Standard.EN15804A2, 
        location=Country.GBR, 
        subtype=SubType.INDUSTRY, 
        impacts=Impacts()
    )
    
    # Add custom attributes after creation
    custom_epd.custom_field = "special_value"
    
    # Verify it's an instance of both CustomEPD and EPD
    assert isinstance(custom_epd, CustomEPD)
    assert isinstance(custom_epd, EPD)
    
    # Verify the custom field and method work
    assert custom_epd.custom_field == "special_value"
    assert custom_epd.custom_method() == "Custom EPD: Custom Test"
    
    # Verify inherited functionality still works
    assert custom_epd.name == "Custom Test"
    assert custom_epd.declared_unit == Unit.M3

