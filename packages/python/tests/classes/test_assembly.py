from lcax import Assembly, Unit

def test_assembly_import():
    assert Assembly


def test_assembly_new():
    assembly = Assembly(name='Test', description="Test Assembly", quantity=4.0, unit=Unit.M, products=[])

    assert assembly

def test_assembly_str(assembly):
    assert str(assembly) == f"Assembly: {assembly.id}"