from lcax import ImpactCategoryKey, LifeCycleModule


def test_impacts_access(impacts):
    assert impacts
    assert impacts[ImpactCategoryKey.GWP][LifeCycleModule.A1A3] == -647.4201396839651



def test_impacts_str(impacts):
    assert str(impacts) == "Impacts"


def test_category_str(impacts):
    assert str(impacts[ImpactCategoryKey.GWP]) == "Impact Category"