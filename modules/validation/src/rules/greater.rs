use std::fmt::Display;
use valitron::available::Message;
use valitron::{Rule, Value};

#[derive(Clone, Debug)]
pub struct Greater<T>(pub T);

impl<T> Greater<T>
where
    T: Display,
{
    fn message_in(&self) -> Message {
        Message::from(format!("Field is not greater than: {}", self.0))
    }
}

impl Rule for Greater<f64> {
    type Message = Message;

    const NAME: &'static str = "Greater";

    fn message(&self) -> Self::Message {
        self.message_in()
    }
    fn call(&mut self, data: &mut Value) -> bool {
        match data {
            Value::Uint8(_value) => {
                if _value.to_owned() > self.0 as u8 {
                    true
                } else {
                    false
                }
            }
            Value::Float64(_value) => {
                if _value.get().gt(&self.0) {
                    true
                } else {
                    false
                }
            }
            Value::Option(_option) => match &**_option {
                Some(_value) => match _value {
                    Value::Uint8(_value) => {
                        if _value.to_owned() > self.0 as u8 {
                            true
                        } else {
                            false
                        }
                    }
                    Value::Float64(_value) => {
                        if _value.get().gt(&self.0) {
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
