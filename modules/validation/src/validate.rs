use crate::model::{Location, ValidationRules, ValidationSchema};
use convert_case::{Case, Casing};
use field_access::FieldAccess;
use lcax_models::project::Project as LCAxProject;
use log;
use std::any::{Any, TypeId};
use std::fmt::Debug;
use valitron::available::{Contains, Length, Message, Range, Required, StartWith};
use valitron::register::ValidatorError;
use valitron::*;

pub fn validate(
    project: &LCAxProject,
    validation_schema: &ValidationSchema,
) -> Result<(), ValidatorError<Message>> {
    let mut validator = Validator::new();
    validator = apply_rules(validator, validation_schema);

    validator.validate(project)
}

fn apply_rules<'a, T: FieldAccess>(
    mut validator: Validator<'a, Message>,
    schema: &'a T,
) -> Validator<'a, Message> {
    for (key, field) in schema.fields() {
        let converted_key = key.to_case(Case::Camel);

        if let Some(Some(rule)) = field.get::<Option<ValidationRules>>() {
            log::trace!("Adding rule for: {:?} {:?}", converted_key, rule);
            match rule {
                ValidationRules::Range(range) => {
                    validator =
                        validator.rule(converted_key.as_str(), Range::new(range.min..range.max));
                }
                ValidationRules::Required => {
                    validator = validator.rule(converted_key.as_str(), Required);
                }
                ValidationRules::Contains(contains) => {
                    validator = validator.rule(converted_key.as_str(), Contains(contains.clone()));
                }
            }
        } else if let Some(Some(nested_schema)) = field.get::<Option<Location>>() {
            for (nested_key, field) in nested_schema.fields() {
                let _camel = nested_key.to_case(Case::Camel);
                let nested_conv_key = format!("{converted_key}.{_camel}");

                if let Some(Some(rule)) = field.get::<Option<ValidationRules>>() {
                    log::trace!("Adding rule for: {:?} {:?}", nested_conv_key, rule);
                    match rule {
                        ValidationRules::Range(range) => {
                            validator = validator
                                .rule(nested_conv_key.as_str(), Range::new(range.min..range.max));
                        }
                        ValidationRules::Required => {
                            validator = validator.rule(nested_conv_key.as_str(), Required);
                        }
                        ValidationRules::Contains(contains) => {
                            validator = validator
                                .rule(nested_conv_key.as_str(), Contains(contains.clone()));
                        }
                    }
                }
            }
        }
    }

    validator
}

// fn apply_rules<'a, T: FieldAccess + Debug>(
//     mut validator: Validator<'a, Message>,
//     schema: &'a T,
// ) -> Validator<'a, Message> {
//     for (key, field) in schema.fields() {
//         let converted_key = key.to_case(Case::Camel);
//
//         if let Some(Some(rule)) = field.get::<Option<ValidationRules>>() {
//             log::trace!("Adding rule for: {:?} {:?}", converted_key, rule);
//             match rule {
//                 ValidationRules::Range(range) => {
//                     validator = validator.rule(
//                         converted_key.as_str(),
//                         Range::new(range.min..range.max),
//                     );
//                 }
//                 ValidationRules::Required => {
//                     validator = validator.rule(converted_key.as_str(), Required);
//                 }
//                 ValidationRules::Contains(contains) => {
//                     validator = validator.rule(
//                         converted_key.as_str(),
//                         Contains(contains.clone()),
//                     );
//                 }
//             }
//         }
//
//         // Handle NestedValidationRules
//         if let Some(Some(nested_schema)) = field.get::<Option<T>>() {
//             validator = apply_rules(validator, nested_schema);
//         }
//     }
//
//     validator
// }
