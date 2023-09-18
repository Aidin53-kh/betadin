use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::runtime::{
    value::{Type, Value},
    DeclType, ScopeStack, eval::statement::{eval_statements, Escape},
};

use super::{string::{_contains, _len, _to_string}, Prototypes};

pub fn list_proto() -> HashMap<String, Value> {
    let mut list_proto = HashMap::new();

    list_proto.insert(String::from("push"), Value::BuiltInMethod(_push, None));
    list_proto.insert(String::from("pop"), Value::BuiltInMethod(_pop, None));
    list_proto.insert(String::from("at"), Value::BuiltInMethod(_at, None));
    list_proto.insert(String::from("len"), Value::BuiltInMethod(_len, None));
    list_proto.insert(String::from("rev"), Value::BuiltInMethod(_rev, None));
    list_proto.insert(String::from("join"), Value::BuiltInMethod(_join, None));
    list_proto.insert(String::from("clear"), Value::BuiltInMethod(_clear, None));
    list_proto.insert(String::from("find"), Value::BuiltInMethod(_find, None));
    list_proto.insert(
        String::from("contains"),
        Value::BuiltInMethod(_contains, None),
    );
    list_proto.insert(
        String::from("to_string"),
        Value::BuiltInMethod(_to_string, None),
    );

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
                        return Ok(Value::from(val));
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

    match this {
        Value::List(list) => match vs.get(0) {
            Some(value) => {
                let mut new_list = list;
                new_list.push(value.clone());
                return Ok(Value::List(new_list));
            }
            None => return Err(format!("expected 1 argument, but found {}", vs.len())),
        },
        _ => {
            return Err(format!(
                "push() dose not exist in '{:?}' prototype",
                Type::from(&this)
            ))
        }
    }
}

pub fn _pop(vs: Vec<Value>, this: Value) -> Result<Value, String> {
    if vs.len() > 0 {
        return Err(format!("expected 1 argument, but found {}", vs.len()));
    }

    match this {
        Value::List(list) => {
            let mut new_list = list;
            new_list.pop();
            return Ok(Value::List(new_list));
        }
        _ => {
            return Err(format!(
                "pop() dose not exist in '{:?}' prototype",
                Type::from(&this)
            ))
        }
    }
}

pub fn _rev(vs: Vec<Value>, this: Value) -> Result<Value, String> {
    if vs.len() > 0 {
        return Err(format!("expected 0 argument, but found {}", vs.len()));
    }

    match this {
        Value::List(list) => Ok(Value::List(list.into_iter().rev().collect())),
        _ => Err(format!(
            "rev() dose not exist in {:?} prototype",
            Type::from(&this)
        )),
    }
}

pub fn _join(vs: Vec<Value>, this: Value) -> Result<Value, String> {
    if vs.len() > 1 || vs.len() < 1 {
        return Err(format!("expected 1 argument, but found {}", vs.len()));
    }

    match this {
        Value::List(list) => match vs.get(0) {
            Some(value) => match value {
                Value::String(s) => {
                    let mut string_list: Vec<String> = vec![];
                    for i in list {
                        string_list.push(i.to_string());
                    }
                    let joined = string_list.join(s);
                    return Ok(Value::String(joined));
                }
                _ => return Err(format!("the first argument most be a string")),
            },
            None => Err(format!("expected 1 argument, but found {}", vs.len())),
        },
        _ => Err(format!(
            "join() dose not exist in '{:?}' prototype",
            Type::from(&this)
        )),
    }
}

pub fn _clear(vs: Vec<Value>, this: Value) -> Result<Value, String> {
    if vs.len() > 0 {
        return Err(format!("expected 0 argument, but found {}", vs.len()));
    }

    match this {
        Value::List(_) => Ok(Value::List(vec![])),
        Value::Object(_) => Ok(Value::Object(vec![])),
        _ => Err(format!(
            "clear() dose not exist in {:?} prototype",
            Type::from(&this)
        )),
    }
}

pub fn _find(vs: Vec<Value>, this: Value) -> Result<Value, String> {
    if vs.len() > 1 || vs.len() < 1 {
        return Err(format!("expected 1 argument, but found {}", vs.len()));
    }

    match this {
        Value::List(list) => match vs.get(0) {
            Some(value) => match value {
                Value::Func(args, block) => {
                    let mut scopes = ScopeStack::new(vec![Arc::new(Mutex::new(HashMap::new()))]);

                    let item = list.iter().find(|i| {
                        if let Some(arg1) = args.get(0) {
                            let value = i.to_owned().to_owned();
                            scopes.declare(arg1, value, DeclType::Immutable).unwrap();
                        }

                        let ret = eval_statements(&mut scopes, block, &Prototypes::exports()).unwrap();
                        
                        if let Escape::Return(val) = ret {
                            match val {
                                Value::Bool(b) => {
                                    return b;
                                }
                                _ => {
                                    panic!("the first argument of find method most returns a boolean");
                                }
                            }
                        }

                        panic!("the first argument of find method most returns a boolean");
                    });

                    Ok(Value::Null)
                }
                _ => return Err(format!("the first argument most be a function")),
            },
            None => Err(format!("expected 1 argument, but found {}", vs.len())),
        },
        _ => Err(format!(
            "find() dose not exist in '{:?}' prototype",
            Type::from(&this)
        )),
    }
}
