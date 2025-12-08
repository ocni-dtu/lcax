import pytest

@pytest.fixture
def empty_calculation_options():
    yield {}


@pytest.fixture
def calculation_options():
    yield {
        "reference_study_period": 50,
        "life_cycle_modules": ["a1a3"], "impact_categories": ["gwp"],
        "overwrite_existing_results": False
    }