use crate::runtime::value::{Type, Value};

pub fn _len(vs: Vec<Value>, this: Value) -> Result<Value, String> {
    if vs.len() > 0 {
        return Err(format!("expected 0 argument, but found {}", vs.len()));
    }

    match this {
        Value::String(s) => Ok(Value::Int(s.len() as i32)),
        Value::List(l) => Ok(Value::Int(l.len() as i32)),
        _ => Err(format!(
            "len dose not exist in {:?} prototype",
            Type::from(&this)
        )),
    }
}

pub fn _to_string(vs: Vec<Value>, this: Value) -> Result<Value, String> {
    if vs.len() > 0 {
        return Err(format!("expected 0 argument, but found {}", vs.len()));
    }

    match this {
        Value::Null => Ok(Value::String("null".to_string())),
        Value::Int(n) => Ok(Value::String(n.to_string())),
        Value::Float(n) => Ok(Value::String(n.to_string())),
        Value::String(s) => Ok(Value::String(s.to_string())),
        _ => Err(format!(
            "to_string dose not exist in {:?} prototype",
            Type::from(&this)
        )),
    }
}
