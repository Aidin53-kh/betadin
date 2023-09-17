use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::sync::{Arc, Mutex};

use runtime::eval::program::eval_program;
use runtime::value::Value;
use runtime::{DeclType, ScopeStack};

#[macro_use]
extern crate lalrpop_util;
extern crate sys_info;

lalrpop_util::lalrpop_mod!(pub grammar);

mod ast;
mod runtime;
mod utils;

#[derive(Debug, Clone)]
pub enum Export {
    Module { name: String, exports: Vec<Export> },
    Item { name: String, value: Value },
}

pub fn ak_print(vs: Vec<Value>) -> Result<Value, String> {
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

pub fn ak_println(vs: Vec<Value>) -> Result<Value, String> {
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

pub fn ak_panic(vs: Vec<Value>) -> Result<Value, String> {
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

fn main() -> Result<(), String> {
    let mut gs = HashMap::new();
    let mut core = BTreeMap::new();

    core.insert(String::from("version"), Value::String("1.0.0".to_string()));

    gs.insert(
        String::from("core"),
        (Value::Module(core), DeclType::Immutable),
    );

    gs.insert(
        String::from("print"),
        (Value::BuiltInFn(ak_print), DeclType::Immutable),
    );
    gs.insert(
        String::from("println"),
        (Value::BuiltInFn(ak_println), DeclType::Immutable),
    );

    gs.insert(
        String::from("panic"),
        (Value::BuiltInFn(ak_panic), DeclType::Immutable),
    );

    let global_scope = Arc::new(Mutex::new(gs));
    let mut scopes = ScopeStack::new(vec![global_scope]);

    let code = fs::read_to_string("./examples/test.ak").expect("unable to read the file");
    let parser = grammar::programParser::new();
    let ast = parser.parse(&code).expect("unable to parse the grammar");
    // println!("{:#?}", ast);
    eval_program(
        &mut scopes,
        ast,
        runtime::std::modules(),
        runtime::std::prototypes(),
    )?;
    // println!("{:#?}", scopes);
    Ok(())
}
