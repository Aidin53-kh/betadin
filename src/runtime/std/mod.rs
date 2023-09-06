use std::collections::HashMap;

use crate::runtime::value::Value;
use crate::Export;

use super::value::Type;

pub mod collections;
pub mod math;
pub mod string;

pub type Prototypes = HashMap<Type, HashMap<String, Value>>;

pub fn _pow(vs: Vec<Value>, this: Value) -> Result<Value, String> {
    match vs.get(0) {
        Some(value) => match value {
            Value::Int(n) => match this {
                Value::Int(n2) => return Ok(Value::Float((n2 as f32).powi(*n))),
                Value::Float(n2) => return Ok(Value::Float(n2.powi(*n))),
                _ => return Err(format!("invalid this argument")),
            },
            Value::Float(n) => match this {
                Value::Int(n2) => return Ok(Value::Float((n2 as f32).powf(*n))),
                Value::Float(n2) => return Ok(Value::Float((n2 as f32).powf(*n))),
                _ => return Err(format!("invalid this argument")),
            },
            _ => return Err(format!("the first argument most be a number")),
        },
        None => Err(format!("the first argument is required")),
    }
}

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

pub fn prototypes() -> Prototypes {
    let mut proto = HashMap::new();

    let mut int_proto = HashMap::new();
    let mut string_proto = HashMap::new();

    int_proto.insert(String::from("pow"), Value::BuiltInMethod(_pow));
    string_proto.insert(String::from("len"), Value::BuiltInMethod(_len));

    proto.insert(Type::Int, int_proto);
    proto.insert(Type::String, string_proto);

    proto
}

pub fn modules() -> Vec<Export> {
    vec![Export::Module {
        name: String::from("std"),
        exports: vec![
            Export::Module {
                name: String::from("math"),
                exports: vec![
                    Export::Module {
                        name: String::from("consts"),
                        exports: math::consts::exports(),
                    },
                    Export::Item {
                        name: String::from("add"),
                        value: Value::BuiltInFn(math::ak_add),
                    },
                    Export::Item {
                        name: String::from("mul"),
                        value: Value::BuiltInFn(math::ak_mul),
                    },
                    Export::Item {
                        name: String::from("div"),
                        value: Value::BuiltInFn(math::ak_div),
                    },
                    Export::Item {
                        name: String::from("sub"),
                        value: Value::BuiltInFn(math::ak_sub),
                    },
                    Export::Item {
                        name: String::from("cos"),
                        value: Value::BuiltInFn(math::ak_cos),
                    },
                    Export::Item {
                        name: String::from("sin"),
                        value: Value::BuiltInFn(math::ak_sin),
                    },
                    Export::Item {
                        name: String::from("abs"),
                        value: Value::BuiltInFn(math::ak_abs),
                    },
                    Export::Item {
                        name: String::from("tan"),
                        value: Value::BuiltInFn(math::ak_tan),
                    },
                    Export::Item {
                        name: String::from("pow"),
                        value: Value::BuiltInFn(math::ak_pow),
                    },
                ],
            },
            Export::Module {
                name: String::from("string"),
                exports: vec![Export::Item {
                    name: String::from("len"),
                    value: Value::BuiltInFn(string::ak_len),
                }],
            },
            Export::Module {
                name: String::from("collections"),
                exports: vec![Export::Item {
                    name: String::from("set"),
                    value: Value::BuiltInFn(collections::ak_set),
                }],
            },
        ],
    }]
}
