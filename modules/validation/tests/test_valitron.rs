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


#[test]
fn test_validate_nested() -> Result<(), String> {
    #[derive(Serialize, Debug)]
    struct Address {
        street: String,
        number: u8
    }
    
    #[derive(Serialize, Debug)]
    struct Person {
        name: String,
        home: Option<Address>
    }

    let validator = Validator::new()
        .rule("name", Required)
        .rule("home", Required)
        .rule("home.number", Required.and(Gt(8_u8)));

    let person = Person {
        name: "Michael".to_string(),
        home: Some(Address{ street: "Broadway".to_string(), number: 10})
    };

    let res = validator.validate(person).unwrap_err();
    println!("{:?}", res);
    assert_eq!(res.len(), 1);
    Ok(())
}