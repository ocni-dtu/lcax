use crate::model::{Level, ValidationResult, ValidationSchema};
use crate::rules;
use lcax_models::assembly::AssemblyReference;
use lcax_models::epd::EPDReference;
use lcax_models::generic_impact_data::GenericDataReference;
use lcax_models::product::{ImpactData, ProductReference};
use lcax_models::project::Project as LCAxProject;
use log;
use valitron::available::{Message, Range, Required};
use valitron::*;

pub fn validate(
    project: &LCAxProject,
    validation_schema: &Vec<ValidationSchema>,
) -> Vec<ValidationResult> {
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
        Err(msg) => {
            log::info!("Validation failed for project");
            for (field_name, message) in msg {
                errors.push(ValidationResult {
                    field: field_name.as_str().to_string(),
                    message: message[0].to_string(),
                })
            }
        }
    }
    for assembly in &project.assemblies {
        match assembly {
            AssemblyReference::Assembly(_assembly) => {
                match assembly_validator.clone().validate(_assembly) {
                    Ok(()) => {
                        log::debug!(
                            "Validation successful for Assembly: {0} - all checks passed",
                            _assembly.id
                        );
                    }
                    Err(msg) => {
                        log::info!("Validation failed for Assembly: {0}", _assembly.id);
                        for (field_name, message) in msg {
                            errors.push(ValidationResult {
                                field: field_name.as_str().to_string(),
                                message: message[0].to_string(),
                            })
                        }
                    }
                }
                for product in &_assembly.products {
                    match product {
                        ProductReference::Product(_product) => {
                            match product_validator.clone().validate(_product) {
                                Ok(()) => {
                                    log::debug!(
                                        "Validation successful for Product: {0} - all checks passed",
                                        _product.id
                                    );
                                }
                                Err(msg) => {
                                    log::info!("Validation failed for Product: {0}", _product.id);
                                    for (field_name, message) in msg {
                                        errors.push(ValidationResult {
                                            field: field_name.as_str().to_string(),
                                            message: message[0].to_string(),
                                        })
                                    }
                                }
                            }
                            for impact in &_product.impact_data {
                                match impact {
                                    ImpactData::GenericData(GenericDataReference::GenericData(
                                        _data,
                                    )) => match impact_validator.clone().validate(_data) {
                                        Ok(()) => {
                                            log::debug!("Validation successful for ImpactData: {0} - all checks passed", _data.id);
                                        }
                                        Err(msg) => {
                                            log::info!(
                                                "Validation failed for ImpactData: {0}",
                                                _data.id
                                            );
                                            for (field_name, message) in msg {
                                                errors.push(ValidationResult {
                                                    field: field_name.as_str().to_string(),
                                                    message: message[0].to_string(),
                                                })
                                            }
                                        }
                                    },
                                    ImpactData::EPD(EPDReference::EPD(_data)) => {
                                        match impact_validator.clone().validate(_data) {
                                            Ok(()) => {
                                                log::debug!("Validation successful for EPD: {0} - all checks passed", _data.id);
                                            }
                                            Err(msg) => {
                                                log::info!(
                                                    "Validation failed for EPD: {0}",
                                                    _data.id
                                                );
                                                for (field_name, message) in msg {
                                                    errors.push(ValidationResult {
                                                        field: field_name.as_str().to_string(),
                                                        message: message[0].to_string(),
                                                    })
                                                }
                                            }
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
    } else {
        log::info!("Validation failed");
    }
    errors
}

fn apply_rule<'a>(
    mut validator: Validator<'a, Message>,
    schema: &ValidationSchema,
) -> Validator<'a, Message> {
    if let Some(range) = &schema.rule.range {
        log::trace!(
            "At {:?} - Applying 'range' rule for: {:?} - {:?}",
            &schema.level,
            &schema.field,
            range
        );
        validator = validator.rule(&schema.field.as_str(), Range::new(range[0]..range[1]));
    }
    if let Some(required) = &schema.rule.required {
        log::trace!(
            "At {:?} - Applying 'required' rule for: {:?} - {:?}",
            &schema.level,
            &schema.field,
            required
        );
        validator = validator.rule(&schema.field.as_str(), Required);
    }
    if let Some(includes) = &schema.rule.includes {
        log::trace!(
            "At {:?} - Applying 'includes' rule for: {:?} - {:?}",
            &schema.level,
            &schema.field,
            includes
        );
        validator = validator.rule(&schema.field.as_str(), rules::Includes(includes.clone()));
    }
    if let Some(greater) = &schema.rule.greater {
        log::trace!(
            "At {:?} - Applying 'greater' rule for: {:?} - {:?}",
            &schema.level,
            &schema.field,
            greater
        );
        validator = validator.rule(&schema.field.as_str(), rules::Greater(*greater));
    }
    if let Some(less) = &schema.rule.less {
        log::trace!(
            "At {:?} - Applying 'less' rule for: {:?} - {:?}",
            &schema.level,
            &schema.field,
            less
        );
        validator = validator.rule(&schema.field.as_str(), rules::Less(*less));
    }
    if let Some(equal) = &schema.rule.equal {
        log::trace!(
            "At {:?} - Applying 'equal' rule for: {:?} - {:?}",
            &schema.level,
            &schema.field,
            equal
        );
        validator = validator.rule(&schema.field.as_str(), rules::Equal(equal.clone()));
    }
    if let Some(one_of) = &schema.rule.one_of {
        log::trace!(
            "At {:?} - Applying 'one of' rule for: {:?} - {:?}",
            &schema.level,
            &schema.field,
            one_of
        );
        validator = validator.rule(&schema.field.as_str(), rules::OneOf(one_of.clone()));
    }
    validator
}
