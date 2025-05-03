from lcax import Unit, Conversion


def test_conversion_access(conversion):
    assert conversion
    assert conversion.to == Unit.KG
    assert conversion.value == 510.45


def test_new_conversion():
    conversion = Conversion(to=Unit.KG, value=1000)
    assert conversion
    
    
def test_conversion_str(conversion):
    assert str(conversion) == "Conversion: KG"
