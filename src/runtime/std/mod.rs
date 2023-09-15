use std::env;

use crate::runtime::value::Value;
use crate::Export;

pub mod collections;
pub mod fs;
pub mod math;
pub mod prototypes;
pub mod system;

pub use prototypes::{prototypes, Prototypes};

use super::value::KeyValue;

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
                        value: Value::BuiltInFn(fs::_fs_read_file),
                    },
                    Export::Item {
                        name: String::from("write"),
                        value: Value::BuiltInFn(fs::_fs_write_file),
                    },
                    Export::Item {
                        name: String::from("rename"),
                        value: Value::BuiltInFn(fs::_fs_rename_file),
                    },
                    Export::Item {
                        name: String::from("remove"),
                        value: Value::BuiltInFn(fs::_fs_remove_file),
                    },
                    Export::Item {
                        name: String::from("read_dir"),
                        value: Value::BuiltInFn(fs::_fs_read_dir),
                    },
                    Export::Item {
                        name: String::from("remove_dir"),
                        value: Value::BuiltInFn(fs::_fs_remove_dir),
                    },
                ],
            },
            Export::Module {
                name: String::from("system"),
                exports: vec![
                    Export::Item {
                        name: String::from("platform"),
                        value: Value::BuiltInFn(system::_platform),
                    },
                    Export::Item {
                        name: String::from("free_mem"),
                        value: Value::BuiltInFn(system::_free_mem),
                    },
                    Export::Item {
                        name: String::from("total_mem"),
                        value: Value::BuiltInFn(system::_total_mem),
                    },
                    Export::Item {
                        name: String::from("total_disk"),
                        value: Value::BuiltInFn(system::_total_disk),
                    },
                    Export::Item {
                        name: String::from("free_disk"),
                        value: Value::BuiltInFn(system::_free_disk),
                    },
                    Export::Item {
                        name: String::from("cpus"),
                        value: Value::BuiltInFn(system::_cpus),
                    },
                    Export::Item {
                        name: String::from("cpu_speed"),
                        value: Value::BuiltInFn(system::_cpu_speed),
                    },
                    Export::Item {
                        name: String::from("version"),
                        value: Value::BuiltInFn(system::_version),
                    },
                    Export::Item {
                        name: String::from("processes"),
                        value: Value::BuiltInFn(system::_processes),
                    },
                    Export::Item {
                        name: String::from("arch"),
                        value: Value::BuiltInFn(system::_arch),
                    },
                    Export::Item {
                        name: String::from("family"),
                        value: Value::BuiltInFn(system::_family),
                    },
                ],
            },
            Export::Module {
                name: String::from("env"),
                exports: vec![
                    Export::Item {
                        name: String::from("args"),
                        value: Value::BuiltInFn(_env_args),
                    },
                    Export::Item {
                        name: String::from("var"),
                        value: Value::BuiltInFn(_env_var),
                    },
                    Export::Item {
                        name: String::from("vars"),
                        value: Value::BuiltInFn(_env_vars),
                    },
                    Export::Item {
                        name: String::from("set_var"),
                        value: Value::BuiltInFn(_env_set_var),
                    },
                    Export::Item {
                        name: String::from("remove_var"),
                        value: Value::BuiltInFn(_env_remove_var),
                    },
                ],
            },
        ],
    }]
}

pub fn _env_args(vs: Vec<Value>) -> Result<Value, String> {
    if vs.len() > 0 {
        return Err(format!("expected 0 arguments, but found {}", vs.len()));
    }

    let mut args = Vec::new();
    for arg in env::args() {
        args.push(Value::String(arg))
    }
    Ok(Value::List(args))
}

pub fn _env_vars(vs: Vec<Value>) -> Result<Value, String> {
    if vs.len() > 0 {
        return Err(format!("expected 0 arguments, but found {}", vs.len()));
    }

    let mut vars = Vec::new();
    for (key, value) in env::vars() {
        vars.push(KeyValue {
            key,
            value: Value::String(value),
        });
    }

    Ok(Value::Object(vars))
}

pub fn _env_var(vs: Vec<Value>) -> Result<Value, String> {
    if vs.len() > 1 && vs.len() < 1 {
        return Err(format!("expected 1 arguments, but found {}", vs.len()));
    }

    match vs.get(0) {
        Some(value) => match value {
            Value::String(arg1) => match env::var(arg1) {
                Ok(v) => return Ok(Value::String(v)),
                Err(e) => return Err(e.to_string()),
            },
            _ => return Err(format!("the first argument most be a string")),
        },
        None => return Err(format!("expected 1 arguments, but found {}", vs.len())),
    }
}

pub fn _env_remove_var(vs: Vec<Value>) -> Result<Value, String> {
    if vs.len() > 1 && vs.len() < 1 {
        return Err(format!("expected 1 arguments, but found {}", vs.len()));
    }

    match vs.get(0) {
        Some(value) => match value {
            Value::String(key) => {
                env::remove_var(key);
                return Ok(Value::Null);
            }
            _ => return Err(format!("the first argument most be a string")),
        },
        None => return Err(format!("expected 1 arguments, but found {}", vs.len())),
    }
}

pub fn _env_set_var(vs: Vec<Value>) -> Result<Value, String> {
    if vs.len() > 1 && vs.len() < 1 {
        return Err(format!("expected 1 arguments, but found {}", vs.len()));
    }

    match vs.get(0) {
        Some(value) => match value {
            Value::String(arg1) => match vs.get(1) {
                Some(value2) => match value2 {
                    Value::String(arg2) => {
                        env::set_var(arg1, arg2);
                        return Ok(Value::Null);
                    }
                    _ => return Err(format!("the second argument most be a string")),
                },
                None => return Err(format!("expected 1 arguments, but found {}", vs.len())),
            },
            _ => return Err(format!("the first argument most be a string")),
        },
        None => return Err(format!("expected 1 arguments, but found {}", vs.len())),
    }
}
