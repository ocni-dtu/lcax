use lcax_models::assembly::Assembly as LCAxAssembly;
use lcax_models::life_cycle_base::{
    ImpactCategory, ImpactCategoryKey, LifeCycleStage, Results as LCAxResults,
};
use lcax_models::product::{ImpactDataSource, Product as LCAxProduct};
use lcax_models::project::Project as LCAxProject;
use lcax_models::shared::ReferenceSource;

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

    let mut project_results = LCAxResults::new();
    project.assemblies.iter_mut().for_each(|(_, assembly)| {
        let results =
            calculate_assembly(resolve_reference_mut(assembly).unwrap(), &_options).unwrap();
        add_results(&mut project_results, &results)
    });
    project.results = Some(project_results.clone());
    Ok(project)
}

pub fn calculate_assembly(
    assembly: &mut LCAxAssembly,
    options: &CalculationOptions,
) -> Result<LCAxResults, String> {
    let mut assembly_results = LCAxResults::new();
    assembly.products.iter_mut().for_each(|(_, product)| {
        let results = calculate_product(resolve_reference_mut(product).unwrap(), options).unwrap();
        add_results(&mut assembly_results, &results)
    });

    options
        .impact_categories
        .iter()
        .for_each(|impact_category_key| {
            options
                .life_cycle_stages
                .iter()
                .for_each(|life_cycle_stage| {
                    let value = assembly_results
                        .get(impact_category_key)
                        .unwrap()
                        .get(life_cycle_stage)
                        .unwrap()
                        .unwrap();

                    *assembly_results
                        .get_mut(impact_category_key)
                        .unwrap()
                        .get_mut(life_cycle_stage)
                        .unwrap() = Some(value * assembly.quantity);
                });
        });
    assembly.results = Some(assembly_results.clone());
    Ok(assembly_results)
}

pub fn calculate_product(
    product: &mut LCAxProduct,
    options: &CalculationOptions,
) -> Result<LCAxResults, String> {
    let mut product_results = LCAxResults::new();

    options
        .impact_categories
        .iter()
        .for_each(|impact_category_key| {
            let mut impact_category = ImpactCategory::new();
            options
                .life_cycle_stages
                .iter()
                .for_each(
                    |life_cycle_stage| match resolve_reference(&product.impact_data) {
                        Ok(ImpactDataSource::EPD(epd)) => {
                            impact_category.insert(
                                life_cycle_stage.clone(),
                                Some(
                                    epd.impacts[impact_category_key][life_cycle_stage].unwrap()
                                        * product.quantity,
                                ),
                            );
                        }
                        Ok(ImpactDataSource::TechFlow(techflow)) => {
                            impact_category.insert(
                                life_cycle_stage.clone(),
                                Some(
                                    techflow.impacts[impact_category_key][life_cycle_stage]
                                        .unwrap()
                                        * product.quantity,
                                ),
                            );
                        }
                        Err(_) => panic!("Handling reference not implemented yet!"),
                    },
                );
            product_results.insert(impact_category_key.clone(), impact_category);
        });

    product.results = Some(product_results.clone());
    Ok(product_results)
}

fn resolve_reference<T>(reference: &ReferenceSource<T>) -> Result<&T, String> {
    match reference {
        ReferenceSource::Actual(reference) => Ok(reference),
        ReferenceSource::Reference(_) => panic!("Handling reference not implemented yet!"),
    }
}

fn resolve_reference_mut<T>(reference: &mut ReferenceSource<T>) -> Result<&mut T, String> {
    match reference {
        ReferenceSource::Actual(reference) => Ok(reference),
        ReferenceSource::Reference(_) => panic!("Handling reference not implemented yet!"),
    }
}

fn add_results(existing_results: &mut LCAxResults, new_results: &LCAxResults) {
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
