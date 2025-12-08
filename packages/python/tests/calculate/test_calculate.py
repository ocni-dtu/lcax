from lcax import calculate_project, calculate_assembly, calculate_product
import pytest

def test_calculate_project(project):
    project = calculate_project(project)

    assert project.results

@pytest.mark.skip("Not Implemented")
def test_calculate_assembly(assembly, calculation_options):
    results = calculate_assembly(assembly, calculation_options)

    assert results

@pytest.mark.skip("Not Implemented")
def test_calculate_assembly_empty(assembly, empty_calculation_options):
    results = calculate_assembly(assembly, empty_calculation_options)

    assert results

@pytest.mark.skip("Not Implemented")
def test_calculate_product(product, calculation_options):
    results = calculate_product(product, calculation_options)

    assert results

@pytest.mark.skip("Not Implemented")
def test_calculate_product_empty(product, empty_calculation_options):
    results = calculate_product(product, empty_calculation_options)

    assert results == {}
