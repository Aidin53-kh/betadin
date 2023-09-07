use std::collections::HashMap;

use crate::runtime::value::{Type, Value};

pub fn list_proto() -> HashMap<String, Value> {
    let mut list_proto = HashMap::new();

    list_proto.insert(String::from("push"), Value::BuiltInMethod(_push));
    list_proto.insert(String::from("at"), Value::BuiltInMethod(_at));

    list_proto
}

pub fn _at(vs: Vec<Value>, this: Value) -> Result<Value, String> {
    if vs.len() > 1 || vs.len() < 1 {
        return Err(format!("expected 1 argument, but found {}", vs.len()));
    }

    match this {
        Value::String(s) => match vs.get(0) {
            Some(value) => match value {
                Value::Int(i) => {
                    if let Some(res) = s.chars().nth(*i as usize) {
                        return Ok(Value::String(res.to_string()));
                    } else {
                        return Err(format!("index out of bounds"));
                    }
                }
                _ => return Err(format!("the first argument most be an integer")),
            },
            None => return Err(format!("expected 1 argument, but found {}", vs.len())),
        },
        Value::List(l) => match vs.get(0) {
            Some(value) => match value {
                Value::Int(i) => {
                    if let Some(val) = l.get(*i as usize) {
                        return Ok(Value::from(val))
                    } else {
                        return Err(format!("index out of bounds"));
                    }
                }
                _ => return Err(format!("the first argument most be an integer")),
            },
            None => return Err(format!("expected 1 argument, but found {}", vs.len())),
        },
        _ => Err(format!(
            "at() dose not exist in '{:?}' prototype",
            Type::from(&this)
        )),
    }
}

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
                    "push() dose not exist in '{:?}' prototype",
                    Type::from(value)
                ))
            }
        },
        None => {
            return Err(format!("expected 1 argument, but found {}", vs.len()));
        }
    }
}
