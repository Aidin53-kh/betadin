use crate::runtime::value::{KeyValue, Type, Value};
use std::collections::HashMap;

pub use super::list::_clear;
pub use super::string::{_contains, _to_string};

pub fn object_proto() -> HashMap<String, Value> {
    let mut object_proto = HashMap::new();

    object_proto.insert(String::from("get"), Value::BuiltInMethod(_obj_get, None));
    object_proto.insert(String::from("set"), Value::BuiltInMethod(_obj_set, None));
    object_proto.insert(String::from("keys"), Value::BuiltInMethod(_obj_keys, None));
    object_proto.insert(
        String::from("values"),
        Value::BuiltInMethod(_obj_values, None),
    );
    object_proto.insert(
        String::from("remove"),
        Value::BuiltInMethod(_obj_remove, None),
    );
    object_proto.insert(
        String::from("contains"),
        Value::BuiltInMethod(_contains, None),
    );
    object_proto.insert(String::from("clear"), Value::BuiltInMethod(_clear, None));

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

pub fn _obj_set(vs: Vec<Value>, this: Value) -> Result<Value, String> {
    if vs.len() > 2 || vs.len() < 2 {
        return Err(format!("expected 2 argument, but found {}", vs.len()));
    }

    match this {
        Value::Object(obj) => match vs.get(0) {
            Some(v1) => match v1 {
                Value::String(arg1) => match vs.get(1) {
                    Some(arg2) => {
                        let mut new_obj: Vec<KeyValue> = vec![];
                        let mut keys: Vec<String> = vec![];

                        for prop in &obj {
                            keys.push(prop.key.to_string());
                        }

                        if !keys.contains(arg1) {
                            new_obj.push(KeyValue {
                                key: arg1.to_string(),
                                value: arg2.clone(),
                            });
                        }
                        
                        for prop in &obj {
                            if &prop.key == arg1 {
                                new_obj.push(KeyValue {
                                    key: prop.key.to_string(),
                                    value: arg2.clone(),
                                });
                            } else {
                                new_obj.push(prop.clone());
                            }
                        }
                        Ok(Value::Object(new_obj))
                    }
                    None => Err(format!("expected 2 argument, but found {}", vs.len())),
                },
                _ => Err(format!("the first argument most be a string")),
            },
            None => Err(format!("expected 2 argument, but found {}", vs.len())),
        },
        _ => Err(format!(
            "set() dose not exist in '{:?}' prototype",
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

pub fn _obj_values(vs: Vec<Value>, this: Value) -> Result<Value, String> {
    if vs.len() > 0 {
        return Err(format!("expected 0 argument, but found {}", vs.len()));
    }

    match this {
        Value::Object(props) => {
            let mut values: Vec<Value> = vec![];

            for prop in props {
                values.push(prop.value);
            }

            Ok(Value::List(values))
        }
        _ => Err(format!(
            "values() dose not exist in {:?} prototype",
            Type::from(&this)
        )),
    }
}

pub fn _obj_remove(vs: Vec<Value>, this: Value) -> Result<Value, String> {
    if vs.len() > 1 || vs.len() < 1 {
        return Err(format!("expected 1 argument, but found {}", vs.len()));
    }

    match this {
        Value::Object(obj) => match vs.get(0) {
            Some(value) => match value {
                Value::String(s) => {
                    let mut keys: Vec<Value> = vec![];

                    for prop in &obj {
                        keys.push(Value::String(prop.key.to_string()));
                    }

                    if keys.contains(&Value::String(s.to_string())) {
                        let new_obj: Vec<KeyValue> =
                            obj.into_iter().filter(|kv| &kv.key != s).collect();
                        return Ok(Value::Object(new_obj));
                    } else {
                        return Err(format!("property '{}' is not defind", s));
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
