use std::fmt::Display;
use valitron::available::Message;
use valitron::{Rule, Value};

#[derive(Clone, Debug)]
pub struct Equal<T>(pub T);

impl<T> Equal<T>
where
    T: Display,
{
    fn message_in(&self) -> Message {
        Message::from(format!("Field is not equal to: {}", self.0))
    }
}


impl Rule for Equal<f64> {
    type Message = Message;

    const NAME: &'static str = "Equal";

    fn message(&self) -> Self::Message {
        self.message_in()
    }
    fn call(&mut self, data: &mut Value) -> bool {
        match data {
            Value::Uint8(_value) => {
                if _value.to_owned() as f64 == self.0 {
                    true
                } else { false }
            }
            Value::Float64(_value) => {
                if _value.get().eq(&self.0) {
                    true
                } else {
                    false
                }
            }
            Value::Option(_option) => match &**_option {
                Some(_value) => match _value {
                    Value::Float64(_value) => {
                        if _value.get().eq(&self.0) {
                            true
                        } else {
                            false
                        }
                    }
                    Value::Uint8(_value) => {
                        if _value.to_owned() as f64 == self.0 {
                            true
                        } else { false }
                    }
                    _ => false,
                },
                None => false,
            },
            _ => false,
        }
    }
}
