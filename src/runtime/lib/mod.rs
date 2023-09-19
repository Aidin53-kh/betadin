use ::std::collections::HashMap;

use self::std::Std;

use super::{value::Value, DeclType};

pub mod std;

pub struct StdLib(HashMap<String, (Value, DeclType)>);

impl StdLib {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn exports() -> HashMap<String, (Value, DeclType)> {
        let mut lib = StdLib::new();

        // builtin modules
        lib.declare("std", Value::Module(Std::exports()));

        // builtin functions
        lib.declare("print", Value::BuiltInFn(ak_lib::print));
        lib.declare("println", Value::BuiltInFn(ak_lib::println));
        lib.declare("panic", Value::BuiltInFn(ak_lib::panic));

        return lib.items();
    }

    pub fn declare(&mut self, name: &str, value: Value) {
        self.0
            .insert(String::from(name), (value, DeclType::Immutable));
    }

    fn items(self) -> HashMap<String, (Value, DeclType)> {
        return self.0;
    }
}

mod ak_lib {
    use crate::runtime::value::Value;

    pub fn print(vs: Vec<Value>) -> Result<Value, String> {
        if vs.len() > 1 || vs.len() < 1 {
            return Err(format!("expected 1 argument, but found {}", vs.len()));
        }

        match vs.get(0) {
            Some(value) => {
                print!("{}", value);
                return Ok(Value::Null);
            }
            None => return Err(format!("expected 1 argument, but found {}", vs.len())),
        }
    }

    pub fn println(vs: Vec<Value>) -> Result<Value, String> {
        if vs.len() > 1 || vs.len() < 1 {
            return Err(format!("expected 1 argument, but found {}", vs.len()));
        }

        match vs.get(0) {
            Some(value) => {
                println!("{}", value);
                return Ok(Value::Null);
            }
            None => return Err(format!("expected 1 argument, but found {}", vs.len())),
        }
    }

    pub fn panic(vs: Vec<Value>) -> Result<Value, String> {
        if vs.len() > 1 || vs.len() < 1 {
            return Err(format!("expected 1 argument, but found {}", vs.len()));
        }

        match vs.get(0) {
            Some(value) => {
                return Err(format!("{}", value));
            }
            None => return Err(format!("expected 1 argument, but found {}", vs.len())),
        }
    }
}
