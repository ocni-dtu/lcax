use std::fmt::Display;
use valitron::available::Message;
use valitron::{Rule, Value};

#[derive(Clone, Debug)]
pub struct Includes<T>(pub T);

impl<T> Includes<T>
where
    T: Display,
{
    fn message_in(&self) -> Message {
        Message::from(format!("Field does not include: {}", self.0))
    }
}

impl Rule for Includes<String> {
    type Message = Message;

    const NAME: &'static str = "Includes";

    fn message(&self) -> Self::Message {
        self.message_in()
    }
    fn call(&mut self, data: &mut Value) -> bool {
        match data {
            Value::Array(array) => {
                for _value in array {
                    match _value {
                        Value::EnumUnit(__value) => {
                            if __value.contains(self.0.as_str()) {
                                return true;
                            }
                        }
                        Value::String(__value) => {
                            if __value.contains(self.0.as_str()) {
                                return true;
                            }
                        }
                        _ => continue,
                    }
                }
                false
            }
            Value::String(_value) => {
                if _value.contains(self.0.as_str()) {
                    true
                } else {
                    false
                }
            }
            Value::Option(_option) => match &**_option {
                Some(_value) => match _value {
                    Value::String(_value) => {
                        if _value.contains(self.0.as_str()) {
                            true
                        } else {
                            false
                        }
                    }
                    Value::Array(array) => {
                        for _value in array {
                            match _value {
                                Value::EnumUnit(__value) => {
                                    if __value.contains(self.0.as_str()) {
                                        return true;
                                    }
                                }
                                Value::String(__value) => {
                                    if __value.contains(self.0.as_str()) {
                                        return true;
                                    }
                                }
                                _ => continue,
                            }
                        }
                        false
                    }
                    _ => false,
                },
                None => false,
            },
            _ => false,
        }
    }
}
