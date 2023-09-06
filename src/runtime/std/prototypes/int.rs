use crate::runtime::value::Value;

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
