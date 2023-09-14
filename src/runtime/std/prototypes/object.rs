use crate::runtime::value::{Type, Value};
use std::collections::HashMap;

pub use super::string::_to_string;

pub fn object_proto() -> HashMap<String, Value> {
    let mut object_proto = HashMap::new();

    object_proto.insert(String::from("get"), Value::BuiltInMethod(_obj_get, None));
    object_proto.insert(String::from("keys"), Value::BuiltInMethod(_obj_keys, None));

    object_proto
}

pub fn _obj_get(vs: Vec<Value>, this: Value) -> Result<Value, String> {
    if vs.len() > 1 || vs.len() < 1 {
        return Err(format!("expected 1 argument, but found {}", vs.len()));
    }

    match this {
        Value::Object(props) => match vs.get(0) {
            Some(value) => match value {
                Value::String(s) => {
                    let prop = props.into_iter().find(|kv| &kv.key == s);
                    match prop {
                        Some(kv) => return Ok(kv.value),
                        None => return Err(format!("property '{}' not found", s)),
                    }
                }
                _ => return Err(format!("the first argument most be a string")),
            },
            None => Err(format!("expected 1 argument, but found {}", vs.len())),
        },
        _ => Err(format!(
            "get() dose not exist in '{:?}' prototype",
            Type::from(&this)
        )),
    }
}

pub fn _obj_keys(vs: Vec<Value>, this: Value) -> Result<Value, String> {
    if vs.len() > 0 {
        return Err(format!("expected 0 argument, but found {}", vs.len()));
    }

    match this {
        Value::Object(props) => {
            let mut keys: Vec<Value> = vec![];

            for prop in props {
                keys.push(Value::String(prop.key))
            }
        
            Ok(Value::List(keys))
        }
        _ => Err(format!(
            "keys() dose not exist in {:?} prototype",
            Type::from(&this)
        )),
    }
}
