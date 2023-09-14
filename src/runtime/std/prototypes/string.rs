use std::collections::HashMap;

use crate::runtime::value::{value_list, Type, Value};

use super::list::_at;

pub fn string_proto() -> HashMap<String, Value> {
    let mut string_proto = HashMap::new();

    string_proto.insert(String::from("len"), Value::BuiltInMethod(_len, None));
    string_proto.insert(
        String::from("to_string"),
        Value::BuiltInMethod(_to_string, None),
    );
    string_proto.insert(String::from("at"), Value::BuiltInMethod(_at, None));

    string_proto
}

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
        Value::List(l) => {
            let list = value_list(l.to_vec());
            Ok(Value::String("[".to_string() + &list + &"]"))
        }
        _ => Err(format!(
            "to_string dose not exist in {:?} prototype",
            Type::from(&this)
        )),
    }
}
