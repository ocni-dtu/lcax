use std::fmt::Debug;
use valitron::available::Message;
use valitron::{Rule, Value};

#[derive(Clone, Debug)]
pub struct OneOf<T>(pub T);

impl<T> OneOf<T>
where
    T: Debug,
{
    fn message_in(&self) -> Message {
        Message::from(format!("Field does not include one of: {:?}", self.0))
    }
}

impl Rule for OneOf<Vec<String>> {
    type Message = Message;

    const NAME: &'static str = "OneOf";

    fn message(&self) -> Self::Message {
        self.message_in()
    }
    fn call(&mut self, data: &mut Value) -> bool {
        match data {
            Value::EnumUnit(__value) => {
                if self.0.contains(&__value.to_owned()) {
                    true
                } else {
                    false
                }
            }
            Value::String(__value) => {
                if self.0.contains(__value) {
                    true
                } else {
                    false
                }
            }
            Value::Option(_option) => match &**_option {
                Some(_value) => match _value {
                    Value::EnumUnit(__value) => {
                        if self.0.contains(&__value.to_string()) {
                            true
                        } else {
                            false
                        }
                    }
                    Value::String(__value) => {
                        if self.0.contains(__value) {
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                },
                None => false,
            },
            _ => false,
        }
    }
}
