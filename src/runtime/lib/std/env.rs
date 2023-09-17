use std::collections::BTreeMap;

use crate::runtime::value::Value;

pub struct Env(BTreeMap<String, Value>);

impl Env {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn exports() -> BTreeMap<String, Value> {
        let mut env = Env::new();

        // env functions
        env.declare("args", Value::BuiltInFn(ak_env::args));
        env.declare("var", Value::BuiltInFn(ak_env::var));
        env.declare("vars", Value::BuiltInFn(ak_env::vars));
        env.declare("remove_var", Value::BuiltInFn(ak_env::remove_var));
        env.declare("set_var", Value::BuiltInFn(ak_env::set_var));

        return env.items();
    }

    pub fn declare(&mut self, name: &str, value: Value) {
        self.0.insert(String::from(name), value);
    }

    fn items(self) -> BTreeMap<String, Value> {
        return self.0;
    }
}

mod ak_env {
    use crate::runtime::value::{KeyValue, Value};
    use std::env;

    pub fn args(vs: Vec<Value>) -> Result<Value, String> {
        if vs.len() > 0 {
            return Err(format!("expected 0 arguments, but found {}", vs.len()));
        }

        let mut args = Vec::new();
        for arg in env::args() {
            args.push(Value::String(arg))
        }
        Ok(Value::List(args))
    }

    pub fn vars(vs: Vec<Value>) -> Result<Value, String> {
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

    pub fn var(vs: Vec<Value>) -> Result<Value, String> {
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

    pub fn remove_var(vs: Vec<Value>) -> Result<Value, String> {
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

    pub fn set_var(vs: Vec<Value>) -> Result<Value, String> {
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
}
