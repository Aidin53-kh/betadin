use crate::runtime::value::{Type, Value};

pub fn _push(vs: Vec<Value>, this: Value) -> Result<Value, String> {
    if vs.len() > 1 || vs.len() < 1 {
        return Err(format!("expected 1 argument, but found {}", vs.len()));
    }

    match vs.get(0) {
        Some(value) => match this {
            Value::List(mut list) => {
                list.push(value.to_owned());
                Ok(Value::List(list.to_vec()))
            }
            _ => {
                return Err(format!(
                    "push dose not exist in {:?} prototype",
                    Type::from(value)
                ))
            }
        },
        None => {
            return Err(format!("expected 1 argument, but found {}", vs.len()));
        }
    }
}
