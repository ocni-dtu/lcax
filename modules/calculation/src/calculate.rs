use lcax_models::assembly::Assembly as LCAxAssembly;
use lcax_models::life_cycle_base::{
    ImpactCategory, ImpactCategoryKey, Impacts, LifeCycleStage, NewResults,
};
use lcax_models::product::{ImpactData, Product as LCAxProduct, Product};
use lcax_models::project::Project as LCAxProject;

pub struct CalculationOptions {
    pub reference_study_period: Option<u8>,
    pub life_cycle_stages: Vec<LifeCycleStage>,
    pub impact_categories: Vec<ImpactCategoryKey>,
}

pub fn calculate_project(
    project: &mut LCAxProject,
    options: Option<CalculationOptions>,
) -> Result<&mut LCAxProject, String> {
    let _options = match options {
        Some(options) => options,
        None => CalculationOptions {
            reference_study_period: project.reference_study_period.clone(),
            life_cycle_stages: project.life_cycle_stages.clone(),
            impact_categories: project.impact_categories.clone(),
        },
    };

    let mut project_results =
        Impacts::new_results(&_options.impact_categories, &_options.life_cycle_stages);
    for assembly in &mut project.assemblies {
        let results = calculate_assembly(&mut assembly.resolve_mut()?, &_options)?;
        add_results(&mut project_results, &results);
    }
    project.results = Some(project_results.clone());
    Ok(project)
}

pub fn calculate_assembly(
    assembly: &mut LCAxAssembly,
    options: &CalculationOptions,
) -> Result<Impacts, String> {
    let mut assembly_results =
        Impacts::new_results(&options.impact_categories, &options.life_cycle_stages);

    for product in &mut assembly.products {
        let results = calculate_product(&mut product.resolve()?, options)?;
        add_results(&mut assembly_results, &results);
    }

    for impact_category_key in &options.impact_categories {
        for life_cycle_stage in &options.life_cycle_stages {
            let value = match assembly_results.get(impact_category_key) {
                Some(_impact) => match _impact.get(life_cycle_stage) {
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
                .get_mut(life_cycle_stage)
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
    let mut product_results = Impacts::new();

    for impact_category_key in &options.impact_categories {
        let mut impact_category = ImpactCategory::new();
        for life_cycle_stage in &options.life_cycle_stages {
            match &product.impact_data {
                ImpactData::EPD(epd) => {
                    let impacts = epd.resolve()?.impacts;
                    impact_category.insert(
                        life_cycle_stage.clone(),
                        Some(add_impact_result(
                            &impacts,
                            impact_category_key,
                            life_cycle_stage,
                            product,
                        )),
                    );
                }
                ImpactData::GenericData(data) => {
                    let impacts = data.resolve()?.impacts;
                    impact_category.insert(
                        life_cycle_stage.clone(),
                        Some(add_impact_result(
                            &impacts,
                            impact_category_key,
                            life_cycle_stage,
                            product,
                        )),
                    );
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
    life_cycle_stage: &LifeCycleStage,
    product: &Product,
) -> f64 {
    match impacts.get(impact_category_key) {
        Some(impact) => match impact.get(life_cycle_stage) {
            Some(value) => value.unwrap() * product.quantity,
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
                .for_each(|(life_cycle_stage, value)| {
                    *existing_results
                        .get_mut(impact_category_key)
                        .unwrap()
                        .get_mut(life_cycle_stage)
                        .unwrap() = Some(
                        existing_results[impact_category_key][life_cycle_stage].unwrap()
                            + value.unwrap(),
                    );
                });
        });
}
