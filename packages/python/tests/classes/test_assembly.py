import uuid


def test_assembly_import():
    from lcax import Assembly

    assert Assembly


def test_assembly_new():
    from lcax import Assembly, Unit

    assembly = Assembly(name='Test', description="Test Assembly", quantity=4.0, unit=Unit.M, products={})

    assert assembly