use serde::Serialize;
use valitron::available::Required;
use valitron::{RuleExt, Validator};

use lcax_validation::rules;

#[test]
fn test_validate_includes_success() -> Result<(), String> {
    #[derive(Serialize, Debug)]
    struct Person {
        name: String,
        hobbies: Vec<String>,
    }

    let validator = Validator::new()
        .rule(
            "hobbies",
            Required.and(rules::Includes("Tennis".to_string())),
        )
        .rule("name", Required.and(rules::Includes("Mich".to_string())));

    let person = Person {
        name: "Michael".to_string(),
        hobbies: vec!["Piano".to_string(), "Tennis".to_string()],
    };

    validator.validate(person).unwrap();
    Ok(())
}

#[test]
fn test_validate_includes_fail() -> Result<(), String> {
    #[derive(Serialize, Debug)]
    struct Person {
        name: String,
        hobbies: Vec<String>,
    }

    let validator = Validator::new()
        .rule(
            "hobbies",
            Required.and(rules::Includes("Football".to_string())),
        )
        .rule("name", Required.and(rules::Includes("Bob".to_string())));

    let person = Person {
        name: "Michael".to_string(),
        hobbies: vec!["Piano".to_string(), "Tennis".to_string()],
    };

    let res = validator.validate(person).unwrap_err();
    println!("{:?}", res);
    assert_eq!(res.len(), 2);
    Ok(())
}

#[test]
fn test_validate_equal_success() -> Result<(), String> {
    #[derive(Serialize, Debug)]
    struct Person {
        name: String,
    }

    let validator =
        Validator::new().rule("name", Required.and(rules::Equal("Michael".to_string())));

    let person = Person {
        name: "Michael".to_string(),
    };

    validator.validate(person).unwrap();
    Ok(())
}

#[test]
fn test_validate_equal_fail() -> Result<(), String> {
    #[derive(Serialize, Debug)]
    struct Person {
        name: String,
    }

    let validator = Validator::new().rule(
        "name",
        Required.and(rules::Equal("Not Michael".to_string())),
    );

    let person = Person {
        name: "Michael".to_string(),
    };

    let res = validator.validate(person).unwrap_err();
    println!("{:?}", res);
    assert_eq!(res.len(), 1);
    Ok(())
}

#[test]
fn test_validate_greater_success() -> Result<(), String> {
    #[derive(Serialize, Debug)]
    struct Person {
        age: f64,
    }

    let validator = Validator::new().rule("age", rules::Greater(10.0));

    let person = Person { age: 15.0 };

    validator.validate(person).unwrap();
    Ok(())
}

#[test]
fn test_validate_greater_fail() -> Result<(), String> {
    #[derive(Serialize, Debug)]
    struct Person {
        age: f64,
    }

    let validator = Validator::new().rule("age", rules::Greater(10.0));

    let person = Person { age: 5.0 };

    let res = validator.validate(person).unwrap_err();
    println!("{:?}", res);
    assert_eq!(res.len(), 1);
    Ok(())
}

#[test]
fn test_validate_less_success() -> Result<(), String> {
    #[derive(Serialize, Debug)]
    struct Person {
        age: f64,
    }

    let validator = Validator::new().rule("age", rules::Less(10.0));

    let person = Person { age: 5.0 };

    validator.validate(person).unwrap();
    Ok(())
}

#[test]
fn test_validate_less_fail() -> Result<(), String> {
    #[derive(Serialize, Debug)]
    struct Person {
        age: f64,
    }

    let validator = Validator::new().rule("age", rules::Less(10.0));

    let person = Person { age: 15.0 };

    let res = validator.validate(person).unwrap_err();
    println!("{:?}", res);
    assert_eq!(res.len(), 1);
    Ok(())
}

#[test]
fn test_validate_one_of_success() -> Result<(), String> {
    #[derive(Serialize, Debug)]
    struct Person {
        name: String,
        hobby: String,
    }

    let validator = Validator::new().rule(
        "hobby",
        rules::OneOf(vec!["Piano".to_string(), "Tennis".to_string()]),
    );

    let person = Person {
        name: "Michael".to_string(),
        hobby: "Piano".to_string(),
    };

    validator.validate(person).unwrap();
    Ok(())
}

#[test]
fn test_validate_one_of_fail() -> Result<(), String> {
    #[derive(Serialize, Debug)]
    struct Person {
        name: String,
        hobby: String,
    }

    let validator = Validator::new().rule(
        "hobby",
        rules::OneOf(vec!["Piano".to_string(), "Tennis".to_string()]),
    );

    let person = Person {
        name: "Michael".to_string(),
        hobby: "Dancing".to_string(),
    };

    let res = validator.validate(person).unwrap_err();
    println!("{:?}", res);
    assert_eq!(res.len(), 1);
    Ok(())
}
