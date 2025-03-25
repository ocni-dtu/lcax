use crate::model::{
    AssemblyReference, BuildingInfo, Location, SoftwareInfo, ValidationRules,
};
use crate::rules;
use convert_case::{Case, Casing};
use field_access::FieldAccess;
use lcax_models::project::Project as LCAxProject;
use log;
use valitron::available::{Message, Range, Required};
use valitron::register::ValidatorError;
use valitron::*;

pub fn validate<T: FieldAccess>(
    project: &LCAxProject,
    validation_schema: &T,
) -> Result<(), ValidatorError<Message>> {
    let mut validator = Validator::new();
    validator = apply_rules(validator, validation_schema, project);

    match validator.validate(project) {
        Ok(()) => {
            log::info!("Validation successful - all checks passed");
            Ok(())
        }
        Err(msg) => Err(msg),
    }
}

fn apply_rules<'a, T: FieldAccess>(
    mut validator: Validator<'a, Message>,
    schema: &T,
    project: &LCAxProject,
) -> Validator<'a, Message> {
    for (key, field) in schema.fields() {
        let converted_key = key.to_case(Case::Camel);

        match field.get::<Option<ValidationRules>>() {
            Some(Some(rule)) => validator = match_rule(rule, &converted_key, validator),
            Some(None) => continue,
            None => {
                if let Some(Some(nested_schema)) = field.get::<Option<Location>>() {
                    for (nested_key, field) in nested_schema.fields() {
                        let _camel = nested_key.to_case(Case::Camel);
                        let nested_conv_key = format!("{converted_key}.{_camel}");

                        if let Some(Some(rule)) = field.get::<Option<ValidationRules>>() {
                            validator = match_rule(rule, &nested_conv_key, validator)
                        }
                    }
                } else if let Some(Some(nested_schema)) = field.get::<Option<BuildingInfo>>() {
                    for (nested_key, field) in nested_schema.fields() {
                        let _camel = nested_key.to_case(Case::Camel);
                        let nested_conv_key = format!("{converted_key}.{_camel}");

                        if let Some(Some(rule)) = field.get::<Option<ValidationRules>>() {
                            validator = match_rule(rule, &nested_conv_key, validator)
                        }
                    }
                } else if let Some(Some(nested_schema)) = field.get::<Option<SoftwareInfo>>() {
                    for (nested_key, field) in nested_schema.fields() {
                        let _camel = nested_key.to_case(Case::Camel);
                        let nested_conv_key = format!("{converted_key}.{_camel}");

                        if let Some(Some(rule)) = field.get::<Option<ValidationRules>>() {
                            validator = match_rule(rule, &nested_conv_key, validator)
                        }
                    }
                } else if let Some(Some(nested_schema)) = field.get::<Option<AssemblyReference>>() {
                    match nested_schema {
                        AssemblyReference::Assembly(_schema) => {
                            let mut errors = vec![];
                            for assembly in project.assemblies {
                                let mut _validator = Validator::new();
                            }
                        }
                        AssemblyReference::Reference(schema) => {
                            for (nested_key, field) in schema.fields() {
                                let _camel = nested_key.to_case(Case::Camel);
                                let nested_conv_key = format!("{converted_key}.{_camel}");

                                if let Some(Some(rule)) = field.get::<Option<ValidationRules>>() {
                                    validator = match_rule(rule, &nested_conv_key, validator)
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    validator
}

// fn apply_rules<'a, T: FieldAccess>(
//     mut validator: Validator<'a, Message>,
//     schema: &T,
// ) -> Validator<'a, Message> {
//     for (key, field) in schema.fields() {
//         let converted_key = key.to_case(Case::Camel);
//
//         if let Some(Some(rule)) = field.get::<Option<ValidationRules>>() {
//             validator = match_rule(rule, &converted_key, validator)
//         } else if let Some(Some(nested_schema)) = field.get::<Option<Location>>() {
//             for (nested_key, field) in nested_schema.fields() {
//                 let _camel = nested_key.to_case(Case::Camel);
//                 let nested_conv_key = format!("{converted_key}.{_camel}");
//
//                 if let Some(Some(rule)) = field.get::<Option<ValidationRules>>() {
//                     validator = match_rule(rule, &nested_conv_key, validator)
//                 }
//             }
//         }
//     }
//
//     validator
// }

// fn print_fields(item: Box<dyn FieldAccess>) {
//     // Iterate over fields of the struct
//     for (field_name, field_value) in item.fields() {
//         // Print the field name and its value
//         println!("Field: {} -> {:?}", field_name, field_value);
//
//         // If the field value implements FieldAccess, recursively call print_fields
//         if let Some(nested) = field_value.as_any().downcast_ref::<dyn FieldAccess>() {
//             // This is a generic recursive call, no need to downcast to a specific type
//             println!("Recursively inspecting nested type:");
//             print_fields(nested);
//         }
//     }
// }

fn match_rule<'a>(
    rule: &ValidationRules,
    key: &str,
    mut validator: Validator<'a, Message>,
) -> Validator<'a, Message> {
    log::trace!("Adding rule for: {:?} {:?}", key, rule);
    match rule {
        ValidationRules::Range(range) => {
            validator = validator.rule(key, Range::new(range.min..range.max));
        }
        ValidationRules::Required => {
            validator = validator.rule(key, Required);
        }
        ValidationRules::Equal(equal) => {
            validator = validator.rule(key, rules::Equal(equal.clone()));
        }
        ValidationRules::Includes(includes) => {
            validator = validator.rule(key, rules::Includes(includes.clone()));
        }
    }
    validator
}

// fn apply_rules<'a, T: FieldAccess>(
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
//                     validator =
//                         validator.rule(converted_key.as_str(), Range::new(range.min..range.max));
//                 }
//                 ValidationRules::Required => {
//                     validator = validator.rule(converted_key.as_str(), Required);
//                 }
//                 ValidationRules::Equal(equal) => {
//                     validator = validator.rule(converted_key.as_str(), rules::Equal(equal.clone()));
//                 }
//                 ValidationRules::Includes(includes) => {
//                     validator =
//                         validator.rule(converted_key.as_str(), rules::Includes(includes.clone()));
//                 }
//             }
//         } else if let Some(Some(nested_schema)) = field.get::<Option<Location>>() {
//             for (nested_key, field) in nested_schema.fields() {
//                 let _camel = nested_key.to_case(Case::Camel);
//                 let nested_conv_key = format!("{converted_key}.{_camel}");
//
//                 if let Some(Some(rule)) = field.get::<Option<ValidationRules>>() {
//                     log::trace!("Adding rule for: {:?} {:?}", nested_conv_key, rule);
//                     match rule {
//                         ValidationRules::Range(range) => {
//                             validator = validator
//                                 .rule(nested_conv_key.as_str(), Range::new(range.min..range.max));
//                         }
//                         ValidationRules::Required => {
//                             validator = validator.rule(nested_conv_key.as_str(), Required);
//                         }
//                         ValidationRules::Equal(equal) => {
//                             validator =
//                                 validator.rule(converted_key.as_str(), rules::Equal(equal.clone()));
//                         }
//                         ValidationRules::Includes(includes) => {
//                             validator = validator
//                                 .rule(nested_conv_key.as_str(), rules::Includes(includes.clone()));
//                         }
//                     }
//                 }
//             }
//         }
//     }
//
//     validator
// }
