use crate::runtime::value::Value;

pub mod consts;

pub fn ak_add(vs: Vec<Value>) -> Result<Value, String> {
    match vs.get(0) {
        Some(value1) => match vs.get(1) {
            Some(value2) => value1 + value2,
            None => return Err(format!("the second argument is require")),
        },
        None => return Err(format!("the first argument is require")),
    }
}

pub fn ak_div(vs: Vec<Value>) -> Result<Value, String> {
    match vs.get(0) {
        Some(value1) => match vs.get(1) {
            Some(value2) => value1 / value2,
            None => return Err(format!("the second argument is require")),
        },
        None => return Err(format!("the first argument is require")),
    }
}

pub fn ak_sub(vs: Vec<Value>) -> Result<Value, String> {
    match vs.get(0) {
        Some(value1) => match vs.get(1) {
            Some(value2) => value1 - value2,
            None => return Err(format!("the second argument is require")),
        },
        None => return Err(format!("the first argument is require")),
    }
}

pub fn ak_mul(vs: Vec<Value>) -> Result<Value, String> {
    match vs.get(0) {
        Some(value1) => match vs.get(1) {
            Some(value2) => value1 * value2,
            None => return Err(format!("the second argument is require")),
        },
        None => return Err(format!("the first argument is require")),
    }
}

pub fn ak_cos(vs: Vec<Value>) -> Result<Value, String> {
    match vs.get(0) {
        Some(value) => match value {
            Value::Int(n) => return Ok(Value::Float((*n as f32).cos())),
            Value::Float(n) => return Ok(Value::Float(n.cos())),
            _ => return Err(format!("the first argument most be a number")),
        },
        None => Err(format!("the first argument is required")),
    }
}

pub fn ak_sin(vs: Vec<Value>) -> Result<Value, String> {
    match vs.get(0) {
        Some(value) => match value {
            Value::Int(n) => return Ok(Value::Float((*n as f32).sin())),
            Value::Float(n) => return Ok(Value::Float(n.sin())),
            _ => return Err(format!("the first argument most be a number")),
        },
        None => Err(format!("the first argument is required")),
    }
}

pub fn ak_tan(vs: Vec<Value>) -> Result<Value, String> {
    match vs.get(0) {
        Some(value) => match value {
            Value::Int(n) => return Ok(Value::Float((*n as f32).tan())),
            Value::Float(n) => return Ok(Value::Float(n.tan())),
            _ => return Err(format!("the first argument most be a number")),
        },
        None => Err(format!("the first argument is required")),
    }
}

pub fn ak_abs(vs: Vec<Value>) -> Result<Value, String> {
    match vs.get(0) {
        Some(value) => match value {
            Value::Int(n) => return Ok(Value::Float((*n as f32).abs())),
            Value::Float(n) => return Ok(Value::Float(n.abs())),
            _ => return Err(format!("the first argument most be a number")),
        },
        None => Err(format!("the first argument is required")),
    }
}

pub fn ak_pow(vs: Vec<Value>) -> Result<Value, String> {
    match vs.get(0) {
        Some(value1) => match value1 {
            Value::Int(n1) => match vs.get(1) {
                Some(value2) => match value2 {
                    Value::Int(n2) => Ok(Value::Float((*n1 as f32).powi(*n2))),
                    Value::Float(n2) => Ok(Value::Float((*n1 as f32).powf(*n2))),
                    _ => return Err(format!("the second argument most be a number")),
                },
                None => return Err(format!("the second argument is required")),
            },
            Value::Float(n1) => match vs.get(1) {
                Some(value2) => match value2 {
                    Value::Int(n2) => Ok(Value::Float(n1.powi(*n2))),
                    Value::Float(n2) => Ok(Value::Float(n1.powf(*n2))),
                    _ => return Err(format!("the second argument most be a number")),
                },
                None => return Err(format!("the second argument is required")),
            },
            _ => return Err(format!("the first argument most be a number")),
        },
        None => Err(format!("the first argument is required")),
    }
}
