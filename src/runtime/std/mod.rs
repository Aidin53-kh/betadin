use std::fs;

use crate::runtime::value::Value;
use crate::Export;

pub mod collections;
pub mod math;
pub mod prototypes;
pub mod string;

pub use prototypes::{prototypes, Prototypes};

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
            Export::Module {
                name: String::from("fs"),
                exports: vec![
                    Export::Item {
                        name: String::from("read"),
                        value: Value::BuiltInFn(_fs_read_file),
                    },
                    Export::Item {
                        name: String::from("write"),
                        value: Value::BuiltInFn(_fs_write_file),
                    },
                ],
            },
        ],
    }]
}

pub fn _fs_read_file(vs: Vec<Value>) -> Result<Value, String> {
    if vs.len() > 1 || vs.len() < 1 {
        return Err(format!("expected 1 argument, but found {}", vs.len()));
    }

    match vs.get(0) {
        Some(value) => match value {
            Value::String(s) => {
                let file_result = fs::read_to_string(s);

                match file_result {
                    Ok(content) => return Ok(Value::String(content)),
                    Err(e) => return Err(e.to_string()),
                }
            }
            _ => return Err(format!("the first argument most be a string")),
        },
        None => {
            return Err(format!("expected 1 argument, but found {}", vs.len()));
        }
    }
}

pub fn _fs_write_file(vs: Vec<Value>) -> Result<Value, String> {
    if vs.len() > 2 || vs.len() < 2 {
        return Err(format!("expected 2 argument, but found {}", vs.len()));
    }

    match vs.get(0) {
        Some(v1) => match v1 {
            Value::String(path) => match vs.get(1) {
                Some(v2) => match v2 {
                    Value::String(content) => {
                        let res = fs::write(path, content);
                        match res {
                            Ok(_) => return Ok(Value::Null),
                            Err(e) => return Err(e.to_string()),
                        }
                    }
                    _ => return Err(format!("the first argument most be a string")),
                },
                None => return Err(format!("expected 1 argument, but found {}", vs.len())),
            },
            _ => return Err(format!("the first argument most be a string")),
        },
        None => {
            return Err(format!("expected 1 argument, but found {}", vs.len()));
        }
    }
}
