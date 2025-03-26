from lcax import Impacts, ImpactCategoryKey, LifeCycleModule, get_impact_total, get_impacts_by_life_cycle_module, \
    normalize_result, ImpactCategory
import pytest

@pytest.fixture
def impacts():
    impacts = Impacts({ImpactCategoryKey.GWP: ImpactCategory({ LifeCycleModule.A1A3: 10})})
    yield impacts

def test_get_impact_total(impacts):
    result = get_impact_total(impacts, ImpactCategoryKey.GWP, None)

    assert result == 10


def test_normalize_result():
    assert normalize_result(10, 2) == 5


def test_get_impacts_by_life_cycle_module(impacts):
    result = get_impacts_by_life_cycle_module(impacts, ImpactCategoryKey.GWP, None, 2.0)

    assert result.dict() == { LifeCycleModule.A1A3: 5}