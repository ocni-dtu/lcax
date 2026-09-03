use crate::models::CalculationOptions;
use lcax_core::value::{AnyValue, Number};
use lcax_models::assembly::Assembly as LCAxAssembly;
use lcax_models::life_cycle_base::{
    ImpactCategory, ImpactCategoryKey, Impacts, LifeCycleModule, NewResults,
};
use lcax_models::product::{ImpactData, Product as LCAxProduct, Product};
use lcax_models::project::Project as LCAxProject;
use lcax_models::shared::MetaData;
use std::collections::HashMap;

pub fn calculate_project(
    project: &mut LCAxProject,
    options: Option<CalculationOptions>,
) -> Result<&mut LCAxProject, String> {
    let mut _options = match options {
        Some(options) => options,
        None => CalculationOptions {
            reference_study_period: project.reference_study_period.clone(),
            life_cycle_modules: project.life_cycle_modules.clone(),
            impact_categories: project.impact_categories.clone(),
            overwrite_existing_results: true,
        },
    };
    if _options.reference_study_period.is_none() {
        _options.reference_study_period = project.reference_study_period.clone();
    }

    if !_options.overwrite_existing_results && project.results.is_some() {
        return Ok(project);
    }

    if let Some(project_info) = &project.project_info {
        if let Some(year) = project_info
            .building_completion_year
            .or(project_info.building_permit_year)
        {
            if year > 1000 {
                for assembly in &mut project.assemblies {
                    if let Ok(actual_assembly) = assembly.resolve_mut() {
                        for product in &mut actual_assembly.products {
                            if let Ok(actual_product) = product.resolve_mut() {
                                let meta = actual_product
                                    .meta_data
                                    .get_or_insert_with(HashMap::new);
                                if !meta.contains_key("startYear")
                                    && !meta.contains_key("start_year")
                                {
                                    meta.insert(
                                        "startYear".to_string(),
                                        Some(AnyValue::Number(Number::Int(year as i64))),
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let mut project_results =
        Impacts::new_results(&_options.impact_categories, &_options.life_cycle_modules);
    for assembly in &mut project.assemblies {
        let results = calculate_assembly(assembly.resolve_mut()?, &_options)?;
        add_results(&mut project_results, &results);
    }
    project.results = Some(project_results.clone());
    Ok(project)
}

pub fn calculate_assembly(
    assembly: &mut LCAxAssembly,
    options: &CalculationOptions,
) -> Result<Impacts, String> {
    if !options.overwrite_existing_results && assembly.results.is_some() {
        return Ok(assembly.results.clone().unwrap());
    }

    let mut assembly_results =
        Impacts::new_results(&options.impact_categories, &options.life_cycle_modules);

    let assembly_rate_based = is_rate_based(&assembly.meta_data, &assembly.description);
    let rsp = options.reference_study_period.map(|r| r as f64).unwrap_or(1.0);

    for product_ref in &mut assembly.products {
        let product = product_ref.resolve_mut()?;
        let product_rate_based = is_rate_based(&product.meta_data, &product.description);
        let mut results = calculate_product(product, options)?;

        if assembly_rate_based && !product_rate_based && rsp != 1.0 {
            scale_impacts(&mut results, rsp);
        }
        add_results(&mut assembly_results, &results);
    }

    for impact_category_key in &options.impact_categories {
        for life_cycle_module in &options.life_cycle_modules {
            let value = match assembly_results.get(impact_category_key) {
                Some(_impact) => match _impact.get(life_cycle_module) {
                    Some(value) => match value {
                        Some(value) => value,
                        None => &0.0,
                    },
                    None => &0.0,
                },
                None => &0.0,
            };
            *assembly_results
                .get_mut(impact_category_key)
                .unwrap()
                .get_mut(life_cycle_module)
                .unwrap() = Some(value * assembly.quantity)
        }
    }
    assembly.results = Some(assembly_results.clone());
    Ok(assembly_results)
}

pub fn calculate_product(
    product: &mut LCAxProduct,
    options: &CalculationOptions,
) -> Result<Impacts, String> {
    if !options.overwrite_existing_results && product.results.is_some() {
        return Ok(product.results.clone().unwrap());
    }

    let rate_based = is_rate_based(&product.meta_data, &product.description);
    let study_period = options
        .reference_study_period
        .map(|r| r as f64)
        .or_else(|| {
            if product.reference_service_life > 0 {
                Some(product.reference_service_life as f64)
            } else {
                None
            }
        })
        .unwrap_or(50.0);

    let rate_scale = if rate_based {
        options
            .reference_study_period
            .map(|r| r as f64)
            .unwrap_or_else(|| {
                if product.reference_service_life > 0 {
                    product.reference_service_life as f64
                } else {
                    1.0
                }
            })
    } else {
        1.0
    };

    let mut milestone_items: Vec<(u16, &ImpactData)> = Vec::new();
    let mut static_items: Vec<&ImpactData> = Vec::new();

    for item in &product.impact_data {
        if let Some(year) = extract_milestone_year(item) {
            milestone_items.push((year, item));
        } else {
            static_items.push(item);
        }
    }

    let is_milestone_series = {
        if milestone_items.len() >= 2 {
            let mut years: Vec<u16> = milestone_items.iter().map(|(y, _)| *y).collect();
            years.sort();
            years.dedup();
            let all_distinct = years.len() == milestone_items.len();
            all_distinct && (static_items.is_empty() || is_interpolated(product))
        } else {
            false
        }
    };

    let mut product_results = Impacts::new();

    for impact_category_key in &options.impact_categories {
        let mut impact_category = ImpactCategory::new();
        for life_cycle_module in &options.life_cycle_modules {
            if is_milestone_series {
                let mut milestones: Vec<(f64, f64)> = Vec::new();
                for (year, item) in &milestone_items {
                    let (impacts, declared_unit, conversions, id) = match item {
                        ImpactData::EPD(epd) => {
                            let e = epd.resolve()?;
                            (
                                e.impacts.clone(),
                                e.declared_unit.clone(),
                                e.conversions.clone(),
                                e.id.clone(),
                            )
                        }
                        ImpactData::GenericData(data) => {
                            let d = data.resolve()?;
                            (
                                d.impacts.clone(),
                                d.declared_unit.clone(),
                                d.conversions.clone(),
                                d.id.clone(),
                            )
                        }
                    };
                    let conversion_factor = resolve_conversion(
                        &product.unit,
                        &declared_unit,
                        &conversions,
                        &id,
                        &product.id,
                    )?;
                    let raw_val =
                        get_raw_impact(&impacts, impact_category_key, life_cycle_module)
                            .unwrap_or(0.0);
                    milestones.push((*year as f64, raw_val * conversion_factor));
                }
                milestones.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

                let has_any_value = milestones.iter().any(|(_, v)| v.abs() > 0.0);
                if has_any_value {
                    let start_year = extract_start_year(&product.meta_data)
                        .unwrap_or(milestones[0].0);
                    let end_year = start_year + study_period;
                    let integral =
                        integrate_piecewise_linear(&milestones, start_year, end_year);
                    let time_weighted_avg = integral / study_period;
                    let contribution = time_weighted_avg * product.quantity * rate_scale;
                    add_to_impact_category(
                        &mut impact_category,
                        life_cycle_module,
                        contribution,
                    );
                } else {
                    add_to_impact_category(&mut impact_category, life_cycle_module, 0.0);
                }

                for item in &static_items {
                    let (impacts, declared_unit, conversions, id) = match item {
                        ImpactData::EPD(epd) => {
                            let e = epd.resolve()?;
                            (
                                e.impacts.clone(),
                                e.declared_unit.clone(),
                                e.conversions.clone(),
                                e.id.clone(),
                            )
                        }
                        ImpactData::GenericData(data) => {
                            let d = data.resolve()?;
                            (
                                d.impacts.clone(),
                                d.declared_unit.clone(),
                                d.conversions.clone(),
                                d.id.clone(),
                            )
                        }
                    };
                    let conversion_factor = resolve_conversion(
                        &product.unit,
                        &declared_unit,
                        &conversions,
                        &id,
                        &product.id,
                    )?;
                    let raw_val =
                        get_raw_impact(&impacts, impact_category_key, life_cycle_module)
                            .unwrap_or(0.0);
                    let contribution =
                        raw_val * conversion_factor * product.quantity * rate_scale;
                    add_to_impact_category(
                        &mut impact_category,
                        life_cycle_module,
                        contribution,
                    );
                }
            } else {
                for impact_data in &product.impact_data {
                    let (impacts, declared_unit, conversions, id) = match impact_data {
                        ImpactData::EPD(epd) => {
                            let e = epd.resolve()?;
                            (
                                e.impacts.clone(),
                                e.declared_unit.clone(),
                                e.conversions.clone(),
                                e.id.clone(),
                            )
                        }
                        ImpactData::GenericData(data) => {
                            let d = data.resolve()?;
                            (
                                d.impacts.clone(),
                                d.declared_unit.clone(),
                                d.conversions.clone(),
                                d.id.clone(),
                            )
                        }
                    };
                    let conversion_factor = resolve_conversion(
                        &product.unit,
                        &declared_unit,
                        &conversions,
                        &id,
                        &product.id,
                    )?;
                    let raw_val =
                        get_raw_impact(&impacts, impact_category_key, life_cycle_module)
                            .unwrap_or(0.0);
                    let contribution =
                        raw_val * conversion_factor * product.quantity * rate_scale;
                    add_to_impact_category(
                        &mut impact_category,
                        life_cycle_module,
                        contribution,
                    );
                }
            }
        }
        product_results.insert(impact_category_key.clone(), impact_category);
    }
    product.results = Some(product_results.clone());
    Ok(product_results)
}

fn scale_impacts(impacts: &mut Impacts, scale: f64) {
    for (_, category) in impacts.iter_mut() {
        for (_, value) in category.iter_mut() {
            if let Some(v) = value {
                *v *= scale;
            }
        }
    }
}

fn is_rate_based(meta_data: &Option<MetaData>, description: &Option<String>) -> bool {
    if let Some(desc) = description {
        let desc_lower = desc.to_ascii_lowercase();
        if desc_lower.contains("linearly interpolated") || desc_lower.contains("annual") {
            return true;
        }
    }
    if let Some(meta) = meta_data {
        for key in [
            "isAnnual",
            "is_annual",
            "annual",
            "rateBased",
            "rate_based",
        ] {
            if let Some(Some(val)) = meta.get(key) {
                match val {
                    AnyValue::Bool(b) => {
                        if *b {
                            return true;
                        }
                    }
                    AnyValue::String(s) => {
                        let s_lower = s.to_ascii_lowercase();
                        if s_lower == "true" || s_lower == "annual" || s_lower == "rate_based" {
                            return true;
                        }
                    }
                    _ => {}
                }
            }
        }
        if let Some(Some(AnyValue::String(s))) =
            meta.get("quantityType").or_else(|| meta.get("quantity_type"))
        {
            let s_lower = s.to_ascii_lowercase();
            if s_lower == "annual" || s_lower == "rate_based" || s_lower == "ratebased" {
                return true;
            }
        }
    }
    false
}

fn is_interpolated(product: &Product) -> bool {
    if let Some(desc) = &product.description {
        let desc_lower = desc.to_ascii_lowercase();
        if desc_lower.contains("interpolat") {
            return true;
        }
    }
    if let Some(meta) = &product.meta_data {
        for key in [
            "interpolate",
            "interpolated",
            "linearInterpolation",
            "linear_interpolation",
            "timeSeries",
            "time_series",
            "isTimeSeries",
            "is_time_series",
        ] {
            if let Some(Some(val)) = meta.get(key) {
                match val {
                    AnyValue::Bool(b) => {
                        if *b {
                            return true;
                        }
                    }
                    AnyValue::String(s) => {
                        let s_lower = s.to_ascii_lowercase();
                        if s_lower == "true" {
                            return true;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    false
}

fn extract_milestone_year(impact_data: &ImpactData) -> Option<u16> {
    match impact_data {
        ImpactData::GenericData(data_ref) => {
            let data = data_ref.resolve().ok()?;
            if let Some(year) = extract_year_from_metadata(&data.meta_data) {
                return Some(year);
            }
            extract_year_from_text(&data.name)
                .or_else(|| data.comment.as_deref().and_then(extract_year_from_text))
        }
        ImpactData::EPD(epd_ref) => {
            let epd = epd_ref.resolve().ok()?;
            if let Some(year) = extract_year_from_metadata(&epd.meta_data) {
                return Some(year);
            }
            extract_year_from_text(&epd.name)
                .or_else(|| epd.comment.as_deref().and_then(extract_year_from_text))
        }
    }
}

fn extract_year_from_metadata(meta_data: &Option<MetaData>) -> Option<u16> {
    let meta = meta_data.as_ref()?;
    for key in [
        "year",
        "milestoneYear",
        "milestone_year",
        "milestone",
        "targetYear",
        "target_year",
    ] {
        if let Some(Some(val)) = meta.get(key) {
            match val {
                AnyValue::Number(Number::Int(y)) => {
                    if (1900..=2200).contains(y) {
                        return Some(*y as u16);
                    }
                }
                AnyValue::Number(Number::Float(y)) => {
                    if *y >= 1900.0 && *y <= 2200.0 {
                        return Some(*y as u16);
                    }
                }
                AnyValue::String(s) => {
                    if let Ok(y) = s.trim().parse::<u16>() {
                        if (1900..=2200).contains(&y) {
                            return Some(y);
                        }
                    }
                }
                _ => {}
            }
        }
    }
    None
}

fn extract_year_from_text(text: &str) -> Option<u16> {
    for word in text.split(|c: char| !c.is_alphanumeric()) {
        if word.len() == 4 && word.chars().all(|c| c.is_ascii_digit()) {
            if let Ok(year) = word.parse::<u16>() {
                if (1900..=2200).contains(&year) {
                    return Some(year);
                }
            }
        }
    }
    None
}

fn extract_start_year(meta_data: &Option<MetaData>) -> Option<f64> {
    let meta = meta_data.as_ref()?;
    for key in [
        "startYear",
        "start_year",
        "completionYear",
        "completion_year",
        "projectCompletionYear",
        "project_completion_year",
        "buildingCompletionYear",
        "building_completion_year",
    ] {
        if let Some(Some(val)) = meta.get(key) {
            match val {
                AnyValue::Number(Number::Int(y)) => {
                    if (1900..=2200).contains(y) {
                        return Some(*y as f64);
                    }
                }
                AnyValue::Number(Number::Float(y)) => {
                    if *y >= 1900.0 && *y <= 2200.0 {
                        return Some(*y);
                    }
                }
                AnyValue::String(s) => {
                    if let Ok(y) = s.trim().parse::<f64>() {
                        if (1900.0..=2200.0).contains(&y) {
                            return Some(y);
                        }
                    }
                }
                _ => {}
            }
        }
    }
    None
}

fn evaluate_piecewise_linear(milestones: &[(f64, f64)], t: f64) -> f64 {
    if milestones.is_empty() {
        return 0.0;
    }
    if milestones.len() == 1 || t <= milestones[0].0 {
        return milestones[0].1;
    }
    if t >= milestones.last().unwrap().0 {
        return milestones.last().unwrap().1;
    }
    for k in 0..(milestones.len() - 1) {
        let (t0, v0) = milestones[k];
        let (t1, v1) = milestones[k + 1];
        if t >= t0 && t <= t1 {
            if (t1 - t0).abs() < 1e-9 {
                return v0;
            }
            return v0 + (v1 - v0) * (t - t0) / (t1 - t0);
        }
    }
    milestones.last().unwrap().1
}

fn integrate_piecewise_linear(milestones: &[(f64, f64)], a: f64, b: f64) -> f64 {
    if b <= a || milestones.is_empty() {
        return 0.0;
    }
    if milestones.len() == 1 {
        return milestones[0].1 * (b - a);
    }
    let mut points = vec![a];
    for &(year, _) in milestones {
        if year > a && year < b {
            points.push(year);
        }
    }
    points.push(b);
    points.sort_by(|x, y| x.partial_cmp(y).unwrap());
    points.dedup_by(|x, y| (*x - *y).abs() < 1e-9);

    let mut total_integral = 0.0;
    for i in 0..(points.len() - 1) {
        let t_start = points[i];
        let t_end = points[i + 1];
        let dt = t_end - t_start;
        let val_start = evaluate_piecewise_linear(milestones, t_start);
        let val_end = evaluate_piecewise_linear(milestones, t_end);
        total_integral += dt * (val_start + val_end) / 2.0;
    }
    total_integral
}

fn resolve_conversion(
    product_unit: &lcax_models::shared::Unit,
    declared_unit: &lcax_models::shared::Unit,
    conversions: &Option<Vec<lcax_models::shared::Conversion>>,
    impact_data_id: &str,
    product_id: &str,
) -> Result<f64, String> {
    if product_unit == declared_unit {
        return Ok(1.0);
    }
    let conv_list = conversions.as_ref().ok_or_else(|| {
        format!(
            "Product and Impact Data do not share the same unit. Impact Data ({}) does not have any conversions.",
            impact_data_id
        )
    })?;
    conv_list
        .iter()
        .find(|c| &c.to == product_unit)
        .map(|c| c.value)
        .ok_or_else(|| {
            format!(
                "Product and Impact Data do not share the same unit. Could not resolve conversion from Impact Data ({}) to Product ({}).",
                impact_data_id, product_id
            )
        })
}

fn get_raw_impact(
    impacts: &Impacts,
    impact_category_key: &ImpactCategoryKey,
    life_cycle_module: &LifeCycleModule,
) -> Option<f64> {
    impacts
        .get(impact_category_key)
        .and_then(|cat| cat.get(life_cycle_module))
        .and_then(|val| *val)
}

fn add_to_impact_category(
    impact_category: &mut ImpactCategory,
    life_cycle_module: &LifeCycleModule,
    contribution: f64,
) {
    let existing = impact_category
        .get(life_cycle_module)
        .and_then(|value| *value)
        .unwrap_or(0.0);
    impact_category.insert(life_cycle_module.clone(), Some(existing + contribution));
}

fn add_results(existing_results: &mut Impacts, new_results: &Impacts) {
    new_results
        .iter()
        .for_each(|(impact_category_key, impact_category)| {
            impact_category
                .iter()
                .for_each(|(life_cycle_module, value)| {
                    match existing_results.get_mut(impact_category_key) {
                        Some(impact_result) => match impact_result.get_mut(life_cycle_module) {
                            Some(life_cycle_result) => {
                                *life_cycle_result =
                                    Some(life_cycle_result.unwrap() + value.unwrap());
                            }
                            None => {
                                impact_result
                                    .insert(life_cycle_module.clone(), Some(value.unwrap()));
                            }
                        },
                        None => {
                            existing_results
                                .insert(impact_category_key.clone(), impact_category.clone());
                        }
                    }
                });
        });
}
