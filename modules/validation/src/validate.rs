use lcax_models::project::Project as LCAxProject;
use log;
use valitron::available::{Message, Range, Required};
use valitron::register::ValidatorError;
use valitron::*;
use lcax_models::assembly::AssemblyReference;
use lcax_models::epd::EPDReference;
use lcax_models::generic_impact_data::GenericDataReference;
use lcax_models::product::{ImpactData, ProductReference};
use crate::model::{Level, ValidationSchema};
use crate::rules;

pub fn validate(
    project: &LCAxProject,
    validation_schema: &Vec<ValidationSchema>,
) -> Result<(), Vec<ValidatorError<Message>>> {
    let mut project_validator = Validator::new();
    let mut assembly_validator = Validator::new();
    let mut product_validator = Validator::new();
    let mut impact_validator = Validator::new();
    
    for schema in validation_schema {
        match schema.level {
            Level::Project => {
                project_validator = apply_rule(project_validator, schema);
            }
            Level::Assembly => {
                assembly_validator = apply_rule(assembly_validator, schema);
            }
            Level::Product => {
                product_validator = apply_rule(product_validator, schema);
            }
            Level::ImpactData => {
                impact_validator = apply_rule(impact_validator, schema);
            }
        }
    } 
    
    let mut errors = vec![];
    
    match project_validator.validate(project) {
        Ok(()) => {
            log::debug!("Project validation successful - all checks passed");
        }
        Err(msg) => errors.push(msg),
    }
    for assembly in &project.assemblies {
        match assembly {
            AssemblyReference::Assembly(_assembly) => {
                match assembly_validator.clone().validate(_assembly) {
                    Ok(()) => {
                        log::debug!("Assembly: {0} validation successful - all checks passed", _assembly.id);
                    }
                    Err(msg) => errors.push(msg),
                }
                for product in &_assembly.products {
                    match product {
                        ProductReference::Product(_product) => {
                            match product_validator.clone().validate(_product) {
                                Ok(()) => {
                                    log::debug!("Product: {0} validation successful - all checks passed", _product.id);
                                }
                                Err(msg) => errors.push(msg),
                            }
                            for impact in &_product.impact_data {
                                match impact { 
                                    ImpactData::GenericData(GenericDataReference::GenericData(_data)) => {
                                        match impact_validator.clone().validate(_data) {
                                            Ok(()) => {
                                                log::debug!("ImpactData: {0} validation successful - all checks passed", _data.id);
                                            }
                                            Err(msg) => errors.push(msg),
                                        }
                                    }
                                    ImpactData::EPD(EPDReference::EPD(_data)) => {
                                        match impact_validator.clone().validate(_data) {
                                            Ok(()) => {
                                                log::debug!("EPD: {0} validation successful - all checks passed", _data.id);
                                            }
                                            Err(msg) => errors.push(msg),
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        ProductReference::Reference(_reference) => {}
                    }
                }
            }
            AssemblyReference::Reference(_reference) => {}
        }

    }
    
    if errors.is_empty() {
        log::info!("Validation successful - all checks passed");
        Ok(())
    } else {
        Err(errors)
    }
}

fn apply_rule<'a>(mut validator: Validator<'a, Message>, schema: &ValidationSchema) -> Validator<'a, Message> {
    if let Some(range) = &schema.rule.range {
        log::trace!("At {:?} - Applying range rule for: {:?} - {:?}", &schema.level, &schema.field, range);
        validator = validator.rule(&schema.field.as_str(), Range::new(range.min..range.max));
    }
    if let Some(includes) = &schema.rule.includes {
        log::trace!("At {:?} - Applying includes rule for: {:?} - {:?}", &schema.level, &schema.field, includes);
        validator = validator.rule(&schema.field.as_str(), rules::Includes(includes.clone()));
    }
    if let Some(required) = &schema.rule.required {
        log::trace!("At {:?} - Applying required rule for: {:?} - {:?}", &schema.level, &schema.field, required);
        validator = validator.rule(&schema.field.as_str(), Required);
    }
    if let Some(equal) = &schema.rule.equal {
        log::trace!("At {:?} - Applying equal rule for: {:?} - {:?}", &schema.level, &schema.field, equal);
        validator = validator.rule(&schema.field.as_str(), rules::Equal(equal.clone()));
    }
    validator
}