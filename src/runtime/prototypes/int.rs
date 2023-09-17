use crate::runtime::value::Value;
use std::collections::HashMap;

pub use super::string::_to_string;

pub fn int_proto() -> HashMap<String, Value> {
    let mut int_proto = HashMap::new();

    int_proto.insert(String::from("pow"), Value::BuiltInMethod(_pow, None));
    int_proto.insert(String::from("to_string"), Value::BuiltInMethod(_to_string, None));

    int_proto
}

pub fn _pow(vs: Vec<Value>, this: Value) -> Result<Value, String> {
    match vs.get(0) {
        Some(value) => match value {
            Value::Int(n) => match this {
                Value::Int(n2) => return Ok(Value::Float((n2 as f32).powi(*n))),
                Value::Float(n2) => return Ok(Value::Float(n2.powi(*n))),
                _ => return Err(format!("invalid this argument")),
            },
            Value::Float(n) => match this {
                Value::Int(n2) => return Ok(Value::Float((n2 as f32).powf(*n))),
                Value::Float(n2) => return Ok(Value::Float((n2 as f32).powf(*n))),
                _ => return Err(format!("invalid this argument")),
            },
            _ => return Err(format!("the first argument most be a number")),
        },
        None => Err(format!("the first argument is required")),
    }
}
