import pytest
from lcax import Unit, get_energy_assemblies


def test_get_energy_assemblies_br18():
    assemblies = get_energy_assemblies("BR18")
    assert len(assemblies) == 3

    expected = [
        ("br18-electricity", "BR18 Grid Electricity", "El", "Grid Electricity"),
        ("br18-district-heating", "BR18 District Heating", "Fjernvarme", "District Heating"),
        ("br18-natural-gas", "BR18 Natural Gas", "Ledningsgas", "Natural Gas"),
    ]

    for i, (exp_id, exp_name, exp_code, exp_prod_name) in enumerate(expected):
        assembly = assemblies[i]
        assert assembly.id == exp_id
        assert assembly.name == exp_name
        assert assembly.unit == Unit.KWH
        assert assembly.quantity == 1.0

        assert len(assembly.classification) == 1
        assert assembly.classification[0].system == "BR18"
        assert assembly.classification[0].code == exp_code

        assert len(assembly.products) == 1
        prod = assembly.products[0]
        assert prod.name == exp_prod_name
        assert prod.unit == Unit.KWH
        assert prod.quantity == 1.0
        assert prod.reference_service_life == 50
        assert prod.description == "Impact data should be linearly interpolated"
        assert prod.meta_data == {"isAnnual": False}
        assert len(prod.impact_data) == 5


def test_get_energy_assemblies_case_insensitive():
    assert len(get_energy_assemblies("br18")) == 3
    assert len(get_energy_assemblies("Br18")) == 3


def test_get_energy_assemblies_unsupported():
    with pytest.raises(ValueError, match="Unsupported standard: unknown"):
        get_energy_assemblies("unknown")
