use crate::runtime::value::{Value, Type};

pub fn _len(vs: Vec<Value>, value: Value) -> Result<Value, String> {
    if vs.len() > 0 {
        return Err(format!("expected 0 argument, but found {}", vs.len()));
    }

    match value {
        Value::String(s) => Ok(Value::Int(s.len() as i32)),
        Value::List(l) => Ok(Value::Int(l.len() as i32)),
        _ => Err(format!(
            "len dose not exist in {:?} prototype",
            Type::from(&value)
        )),
    }
}