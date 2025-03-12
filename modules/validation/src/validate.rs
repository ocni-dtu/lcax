use crate::model::{NestedValidationRules, ValidationRules, ValidationSchema};
use lcax_models::project::Project as LCAxProject;
use valitron::available::{Message, Required};
use valitron::register::ValidatorError;
use valitron::*;
use field_access::FieldAccess;

pub fn validate(
    project: &LCAxProject,
    validation_schema: &ValidationSchema,
) -> Result<(), ValidatorError<Message>> {
    // let validator = Validator::new()
    //     .rule("name", Required.and(StartWith("hello")))
    //     .rule("age", custom(age_limit))
    //     .message([
    //         ("name.required", "name is required"),
    //         ("name.start_with", "name should be starts with `hello`"),
    //     ]);
    let mut validator = Validator::new();
    apply_rules(validator, validation_schema);;


    validator.validate(project)
}


fn apply_rules(validator: Validator<Message>, rules: &ValidationSchema) {

    for (key, field) in rules.fields() {
        validator.rule("no key", Required);

        // match field.get().unwrap() {
        //     NestedValidationRules::Rules(rule) => {
        //         validator.rule(key, Required);
        //     }
        //     NestedValidationRules::Nested(rules) => {
        //         apply_rules(validator, rules);
        //     }
        // }
    }
}