use lcax_calculation::results::{get_impact_total, get_impacts_by_life_cycle_module};
use lcax_models::life_cycle_base::{ImpactCategory, ImpactCategoryKey, Impacts, LifeCycleModule};

#[test]
fn test_get_total() -> Result<(), String> {
    let impacts = Impacts::from([(
        ImpactCategoryKey::GWP,
        ImpactCategory::from([
            (LifeCycleModule::A1A3, Some(10.0)),
            (LifeCycleModule::A4, Some(10.0)),
            (LifeCycleModule::B1, Some(10.0)),
            (LifeCycleModule::B2, None),
            (LifeCycleModule::D, Some(-10.0)),
        ]),
    )]);

    assert_eq!(
        get_impact_total(&impacts, &ImpactCategoryKey::GWP, &None),
        20.0
    );
    Ok(())
}

#[test]
fn test_get_total_exclude() -> Result<(), String> {
    let impacts = Impacts::from([(
        ImpactCategoryKey::GWP,
        ImpactCategory::from([
            (LifeCycleModule::A1A3, Some(10.0)),
            (LifeCycleModule::A4, Some(10.0)),
            (LifeCycleModule::B1, Some(10.0)),
            (LifeCycleModule::B2, None),
            (LifeCycleModule::D, Some(-10.0)),
        ]),
    )]);

    assert_eq!(
        get_impact_total(
            &impacts,
            &ImpactCategoryKey::GWP,
            &Some(vec![LifeCycleModule::D])
        ),
        30.0
    );
    Ok(())
}

#[test]
fn test_get_impacts_by_life_cycle_module() -> Result<(), String> {
    let mut impacts = Impacts::from([(
        ImpactCategoryKey::GWP,
        ImpactCategory::from([
            (LifeCycleModule::A1A3, Some(10.0)),
            (LifeCycleModule::A4, Some(10.0)),
            (LifeCycleModule::B1, Some(10.0)),
            (LifeCycleModule::B2, None),
            (LifeCycleModule::D, Some(-10.0)),
        ]),
    )]);

    assert_eq!(
        get_impacts_by_life_cycle_module(
            &mut impacts,
            &ImpactCategoryKey::GWP,
            &&Some(vec![LifeCycleModule::D]),
            Some(2.0)
        ),
        Some(ImpactCategory::from([
            (LifeCycleModule::A1A3, Some(5.0)),
            (LifeCycleModule::A4, Some(5.0)),
            (LifeCycleModule::B1, Some(5.0)),
        ]))
    );
    Ok(())
}
