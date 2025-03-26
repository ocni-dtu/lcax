use lcax_models::life_cycle_base::{ImpactCategory, ImpactCategoryKey, Impacts, LifeCycleModule};

pub fn get_impact_total(
    impacts: &Impacts,
    category: &ImpactCategoryKey,
    exclude_modules: &Option<Vec<LifeCycleModule>>,
) -> f64 {
    match impacts.get(category) {
        Some(impact) => {
            let mut total = 0.0;
            for (key, value) in impact.iter() {
                match exclude_modules {
                    Some(exclude_modules) => match exclude_modules.contains(key) {
                        true => continue,
                        false => {}
                    },
                    None => {}
                }
                total = total + value.unwrap_or(0.0);
            }
            total
        }
        None => 0.0,
    }
}

pub fn normalize_result(result: &f64, normalizing_factor: &f64) -> f64 {
    result / normalizing_factor
}

pub fn get_impacts_by_life_cycle_module(
    impacts: &Impacts,
    category: &ImpactCategoryKey,
    exclude_modules: &Option<Vec<LifeCycleModule>>,
    normalizing_factor: Option<f64>,
) -> Option<ImpactCategory> {
    let new_impacts = match impacts.get(category) {
        Some(impact) => match exclude_modules {
            Some(exclude_modules) => {
                let mut _impacts = impact.clone();
                for _exclude_module in exclude_modules {
                    _impacts.remove(_exclude_module);
                }
                Some(_impacts)
            }
            None => Some(impact.clone()),
        },
        None => None,
    };
    if let (Some(new_impacts), Some(normalizing_factor)) = (&new_impacts, &normalizing_factor) {
        let mut _impacts = ImpactCategory::new();
        for (key, value) in new_impacts.iter() {
            let _value = match value {
                Some(value) => normalize_result(&value, normalizing_factor),
                None => continue,
            };
            _impacts.insert(key.clone(), Some(_value));
        }
        return Some(_impacts);
    };
    new_impacts
}
