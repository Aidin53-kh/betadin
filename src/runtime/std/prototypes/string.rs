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
    string_proto.insert(String::from("chars"), Value::BuiltInMethod(_chars, None));
    string_proto.insert(String::from("split"), Value::BuiltInMethod(_split, None));
    string_proto.insert(String::from("to_upper"), Value::BuiltInMethod(_upper, None));
    string_proto.insert(String::from("to_lower"), Value::BuiltInMethod(_lower, None));
    string_proto.insert(String::from("trim"), Value::BuiltInMethod(_trim, None));
    string_proto.insert(
        String::from("is_ascii"),
        Value::BuiltInMethod(_is_ascii, None),
    );
    string_proto.insert(
        String::from("contains"),
        Value::BuiltInMethod(_contains, None),
    );
    string_proto.insert(String::from("repeat"), Value::BuiltInMethod(_repeat, None));
    string_proto.insert(
        String::from("replace"),
        Value::BuiltInMethod(_replace, None),
    );
    string_proto.insert(String::from("push"), Value::BuiltInMethod(_push, None));

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

pub fn _chars(vs: Vec<Value>, this: Value) -> Result<Value, String> {
    if vs.len() > 0 {
        return Err(format!("expected 0 argument, but found {}", vs.len()));
    }

    match this {
        Value::String(s) => {
            let mut chars: Vec<Value> = vec![];
            for char in s.chars() {
                chars.push(Value::String(char.to_string()));
            }
            Ok(Value::List(chars))
        }
        _ => Err(format!(
            "to_string dose not exist in {:?} prototype",
            Type::from(&this)
        )),
    }
}

pub fn _split(vs: Vec<Value>, this: Value) -> Result<Value, String> {
    if vs.len() > 1 || vs.len() < 1 {
        return Err(format!("expected 1 argument, but found {}", vs.len()));
    }

    match this {
        Value::String(s) => match vs.get(0) {
            Some(value) => match value {
                Value::String(val) => {
                    let mut res: Vec<Value> = vec![];
                    for i in s.split(val) {
                        res.push(Value::String(i.to_string()));
                    }

                    return Ok(Value::List(res));
                }
                _ => return Err(format!("the first argument most be a string")),
            },
            None => Err(format!("expected 1 argument, but found {}", vs.len())),
        },
        _ => Err(format!(
            "split() dose not exist in '{:?}' prototype",
            Type::from(&this)
        )),
    }
}

pub fn _upper(vs: Vec<Value>, this: Value) -> Result<Value, String> {
    if vs.len() > 0 {
        return Err(format!("expected 0 argument, but found {}", vs.len()));
    }

    match this {
        Value::String(s) => Ok(Value::String(s.to_uppercase())),
        _ => Err(format!(
            "upper() dose not exist in {:?} prototype",
            Type::from(&this)
        )),
    }
}

pub fn _lower(vs: Vec<Value>, this: Value) -> Result<Value, String> {
    if vs.len() > 0 {
        return Err(format!("expected 0 argument, but found {}", vs.len()));
    }

    match this {
        Value::String(s) => Ok(Value::String(s.to_lowercase())),
        _ => Err(format!(
            "lower() dose not exist in {:?} prototype",
            Type::from(&this)
        )),
    }
}

pub fn _trim(vs: Vec<Value>, this: Value) -> Result<Value, String> {
    if vs.len() > 0 {
        return Err(format!("expected 0 argument, but found {}", vs.len()));
    }

    match this {
        Value::String(s) => Ok(Value::String(s.trim().to_string())),
        _ => Err(format!(
            "trim() dose not exist in {:?} prototype",
            Type::from(&this)
        )),
    }
}

pub fn _contains(vs: Vec<Value>, this: Value) -> Result<Value, String> {
    if vs.len() > 1 || vs.len() < 1 {
        return Err(format!("expected 1 argument, but found {}", vs.len()));
    }

    match this {
        Value::String(s) => match vs.get(0) {
            Some(value) => match value {
                Value::String(val) => return Ok(Value::Bool(s.contains(val))),
                _ => return Err(format!("the first argument most be a string")),
            },
            None => Err(format!("expected 1 argument, but found {}", vs.len())),
        },
        Value::List(list) => match vs.get(0) {
            Some(value) => Ok(Value::Bool(list.contains(&value))),
            None => Err(format!("expected 1 argument, but found {}", vs.len())),
        },
        Value::Object(obj) => match vs.get(0) {
            Some(value) => match value {
                Value::String(s) => {
                    let mut keys: Vec<Value> = vec![];

                    for prop in obj {
                        keys.push(Value::String(prop.key))
                    }

                    Ok(Value::Bool(keys.contains(&Value::String(s.to_string()))))
                }
                _ => return Err(format!("the first argument most be a string")),
            },
            None => Err(format!("expected 1 argument, but found {}", vs.len())),
        },
        _ => Err(format!(
            "contains() dose not exist in '{:?}' prototype",
            Type::from(&this)
        )),
    }
}

pub fn _is_ascii(vs: Vec<Value>, this: Value) -> Result<Value, String> {
    if vs.len() > 0 {
        return Err(format!("expected 0 argument, but found {}", vs.len()));
    }

    match this {
        Value::String(s) => Ok(Value::Bool(s.is_ascii())),
        _ => Err(format!(
            "len dose not exist in {:?} prototype",
            Type::from(&this)
        )),
    }
}

pub fn _repeat(vs: Vec<Value>, this: Value) -> Result<Value, String> {
    if vs.len() > 1 || vs.len() < 1 {
        return Err(format!("expected 1 argument, but found {}", vs.len()));
    }

    match this {
        Value::String(s) => match vs.get(0) {
            Some(value) => match value {
                Value::Int(val) => return Ok(Value::String(s.repeat(*val as usize))),
                _ => return Err(format!("the first argument most be a integer")),
            },
            None => Err(format!("expected 1 argument, but found {}", vs.len())),
        },
        _ => Err(format!(
            "split() dose not exist in '{:?}' prototype",
            Type::from(&this)
        )),
    }
}

pub fn _replace(vs: Vec<Value>, this: Value) -> Result<Value, String> {
    if vs.len() > 2 || vs.len() < 2 {
        return Err(format!("expected 2 argument, but found {}", vs.len()));
    }

    match this {
        Value::String(s) => match vs.get(0) {
            Some(v1) => match v1 {
                Value::String(from) => match vs.get(1) {
                    Some(v2) => match v2 {
                        Value::String(to) => Ok(Value::String(s.replace(from, to))),
                        _ => Err(format!("the seconde argument most be a string")),
                    },
                    None => Err(format!("expected 2 argument, but found {}", vs.len())),
                },
                _ => Err(format!("the first argument most be a string")),
            },
            None => Err(format!("expected 2 argument, but found {}", vs.len())),
        },
        _ => Err(format!(
            "split() dose not exist in '{:?}' prototype",
            Type::from(&this)
        )),
    }
}

pub fn _push(vs: Vec<Value>, this: Value) -> Result<Value, String> {
    if vs.len() > 1 || vs.len() < 1 {
        return Err(format!("expected 1 argument, but found {}", vs.len()));
    }

    match this {
        Value::String(s) => match vs.get(0) {
            Some(value) => match value {
                Value::String(val) => {
                    let mut res = s;
                    res.push_str(&val);
                    return Ok(Value::String(res));
                }
                _ => return Err(format!("the first argument most be a string")),
            },
            None => Err(format!("expected 1 argument, but found {}", vs.len())),
        },
        _ => Err(format!(
            "split() dose not exist in '{:?}' prototype",
            Type::from(&this)
        )),
    }
}
