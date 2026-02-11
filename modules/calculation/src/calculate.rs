use crate::models::CalculationOptions;
use lcax_models::assembly::Assembly as LCAxAssembly;
use lcax_models::life_cycle_base::{
    ImpactCategory, ImpactCategoryKey, Impacts, LifeCycleModule, NewResults,
};
use lcax_models::product::{ImpactData, Product as LCAxProduct, Product};
use lcax_models::project::Project as LCAxProject;

pub fn calculate_project(
    project: &mut LCAxProject,
    options: Option<CalculationOptions>,
) -> Result<&mut LCAxProject, String> {
    let _options = match options {
        Some(options) => options,
        None => CalculationOptions {
            reference_study_period: project.reference_study_period.clone(),
            life_cycle_modules: project.life_cycle_modules.clone(),
            impact_categories: project.impact_categories.clone(),
            overwrite_existing_results: true,
        },
    };

    if !_options.overwrite_existing_results && project.results.is_some() {
        return Ok(project);
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

    for product in &mut assembly.products {
        let results = calculate_product(product.resolve_mut()?, options)?;
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

    let mut product_results = Impacts::new();

    for impact_category_key in &options.impact_categories {
        let mut impact_category = ImpactCategory::new();
        for life_cycle_module in &options.life_cycle_modules {
            for impact_data in &product.impact_data {
                match impact_data {
                    ImpactData::EPD(epd) => {
                        let _epd = &epd.resolve()?;
                        let impacts = &_epd.impacts;
                        let mut conversion_factor = 1.0;

                        if product.unit != _epd.declared_unit {
                            let conversions = _epd.conversions.as_ref().ok_or_else(|| {
                                format!(
                                    "Product and EPD does not share the same unit. EPD ({}) does not have any conversions.",
                                    _epd.id
                                )
                            })?;

                            conversion_factor = conversions
                                .iter()
                                .find(|_conversion| _conversion.to == product.unit)
                                .map(|_conversion| _conversion.value)
                                .ok_or_else(|| {
                                    format!(
                                        "Product and EPD does not share the same unit. Could not resolve conversion from EPD ({}) to Product ({}).",
                                        _epd.id, product.id
                                    )
                                })?;
                        }

                        impact_category.insert(
                            life_cycle_module.clone(),
                            Some(add_impact_result(
                                &impacts,
                                impact_category_key,
                                life_cycle_module,
                                &conversion_factor,
                                product,
                            )),
                        );
                    }
                    ImpactData::GenericData(data) => {
                        let _data = &data.resolve()?;
                        let impacts = &_data.impacts;
                        let mut conversion_factor = 1.0;
                        if product.unit != _data.declared_unit {
                            match &_data.conversions {
                                Some(conversions) => {
                                    match conversions.iter().find(|_conversion| _conversion.to == product.unit) {
                                        None => panic!(
                                            "Product and Generic Data does not share the same unit. Could not resolve conversion from Generic Data ({}) to Product ({}).", _data.id, product.id
                                        ),
                                        Some(_conversion) => conversion_factor = _conversion.value
                                    }
                                }
                                None => panic!(
                                    "Product and Generic Data does not share the same unit. Generic Data ({}) does not have any conversions.", _data.id
                                ),
                            }
                        }
                        impact_category.insert(
                            life_cycle_module.clone(),
                            Some(add_impact_result(
                                &impacts,
                                impact_category_key,
                                life_cycle_module,
                                &conversion_factor,
                                product,
                            )),
                        );
                    }
                }
            }
        }
        product_results.insert(impact_category_key.clone(), impact_category);
    }
    product.results = Some(product_results.clone());
    Ok(product_results)
}

fn add_impact_result(
    impacts: &Impacts,
    impact_category_key: &ImpactCategoryKey,
    life_cycle_module: &LifeCycleModule,
    conversion_factor: &f64,
    product: &Product,
) -> f64 {
    match impacts.get(impact_category_key) {
        Some(impact) => match impact.get(life_cycle_module) {
            Some(value) => match value {
                Some(value) => value * product.quantity * conversion_factor,
                None => 0.0,
            },
            None => 0.0,
        },
        None => 0.0,
    }
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
