use std::fmt::Display;
use valitron::available::Message;
use valitron::{Rule, Value};

#[derive(Clone, Debug)]
pub struct Less<T>(pub T);

impl<T> Less<T>
where
    T: Display,
{
    fn message_in(&self) -> Message {
        Message::from(format!("Field is not less than: {}", self.0))
    }
}

impl Rule for Less<f64> {
    type Message = Message;

    const NAME: &'static str = "Less";

    fn message(&self) -> Self::Message {
        self.message_in()
    }
    fn call(&mut self, data: &mut Value) -> bool {
        match data {
            Value::Uint8(_value) => {
                if _value.to_owned() < self.0 as u8 {
                    true
                } else {
                    false
                }
            }
            Value::Uint16(_value) => {
                if _value.to_owned() < self.0 as u16 {
                    true
                } else {
                    false
                }
            }
            Value::Float64(_value) => {
                if _value.get().lt(&self.0) {
                    true
                } else {
                    false
                }
            }
            Value::Option(_option) => match &**_option {
                Some(_value) => match _value {
                    Value::Uint8(_value) => {
                        if _value.to_owned() < self.0 as u8 {
                            true
                        } else {
                            false
                        }
                    }
                    Value::Uint16(_value) => {
                        if _value.to_owned() < self.0 as u16 {
                            true
                        } else {
                            false
                        }
                    }
                    Value::Float64(_value) => {
                        if _value.get().lt(&self.0) {
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
