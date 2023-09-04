use ast::expression::Literal;
use runtime::eval::program::eval_program;
use std::{collections::HashMap, fs};

#[macro_use]
extern crate lalrpop_util;

lalrpop_util::lalrpop_mod!(pub grammar);

mod ast;
mod runtime;

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Literal(Literal),
    BuiltInFn(fn(Vec<Value>) -> Result<Value, String>),
}

fn ak_print(vs: Vec<Value>) -> Result<Value, String> {
    match &vs[0] {
        Value::Literal(v) => {
            print!("{}", v);
            return Ok(Value::Null);
        }
        _ => {
            return Err(format!("unsupported arg"));
        }
    }
}

fn ak_println(vs: Vec<Value>) -> Result<Value, String> {
    match &vs[0] {
        Value::Literal(v) => {
            println!("{}", v);
            return Ok(Value::Null);
        }
        _ => {
            return Err(format!("unsupported arg"));
        }
    }
}


#[derive(Debug, Clone)]
pub enum Export {
    Module { name: String, exports: Vec<Export> },
    Item { name: String, value: Value },
}

#[derive(Debug, Clone)]

pub enum Exports {
    Module(HashMap<String, Export>),
    Item(Value),
}

fn main() -> Result<(), String> {
    let mut env = HashMap::<String, Value>::new();

    env.insert(String::from("print"), Value::BuiltInFn(ak_print));
    env.insert(String::from("println"), Value::BuiltInFn(ak_println));

    let modules: Vec<Export> = vec![];

    let code = fs::read_to_string("./examples/test.ak").expect("unable to read the file");
    let parser = grammar::programParser::new();
    let ast = parser.parse(&code).expect("unable to parse the grammar");

    // println!("{:#?}", ast);
    eval_program(&mut env, ast, modules)
    // println!("program result: {:?}", result);
}
