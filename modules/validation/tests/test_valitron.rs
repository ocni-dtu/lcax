use serde::Serialize;
use valitron::available::{Gt, Required};
use valitron::{RuleExt, Validator};

#[test]
fn test_validate_pure() -> Result<(), String> {
    #[derive(Serialize, Debug)]
    struct Person {
        introduce: String,
        age: Option<u8>,
        weight: f32,
    }

    let validator = Validator::new()
        .rule("introduce", Required)
        .rule("age", Required.and(Gt(10_u8)));
    // .message([
    //     ("introduce.required", "introduce is required"),
    //     (
    //         "introduce.start_with",
    //         "introduce should be starts with `I am`",
    //     ),
    //     ("age.required", "age is required"),
    //     ("age.gt", "age should be greater than 10"),
    // ]);

    let person = Person {
        introduce: "".to_string(),
        age: Some(18),
        weight: 20.0,
    };

    let res = validator.validate(person).unwrap_err();
    println!("{:?}", res);
    assert_eq!(res.len(), 1);
    Ok(())
}
