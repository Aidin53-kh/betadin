use ::std::collections::HashMap;

use self::std::Std;

use super::{value::Value, DeclType, Type};

pub mod std;

pub struct StdLib(HashMap<String, (Value, DeclType, Type)>);

impl StdLib {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn exports() -> HashMap<String, (Value, DeclType, Type)> {
        let mut lib = StdLib::new();

        // builtin modules
        lib.declare(
            "std",
            Value::Module(Std::exports()),
            Type::Custom("module".to_string()),
        );

        // builtin functions
        lib.declare(
            "print",
            Value::BuiltInFn(ak_lib::print),
            Type::Custom("function".to_string()),
        );
        lib.declare(
            "println",
            Value::BuiltInFn(ak_lib::println),
            Type::Custom("function".to_string()),
        );
        lib.declare(
            "panic",
            Value::BuiltInFn(ak_lib::panic),
            Type::Custom("function".to_string()),
        );

        return lib.items();
    }

    pub fn declare(&mut self, name: &str, value: Value, datatype: Type) {
        self.0
            .insert(String::from(name), (value, DeclType::Immutable, datatype));
    }

    fn items(self) -> HashMap<String, (Value, DeclType, Type)> {
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
