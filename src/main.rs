use ast::expression::Literal;
use runtime::eval::program::eval_program;
use std::{collections::HashMap, f32::consts::PI, fs};

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

fn ak_add(vs: Vec<Value>) -> Result<Value, String> {
    let first = vs.get(0).expect("the first argument is required");
    let second = vs.get(1).expect("the second argument is required");

    if let Value::Literal(first) = first {
        if let Value::Literal(second) = second {
            return Ok(Value::Literal((first.clone() + second.clone())?));
        } else {
            return Err(format!("the first argument most be a literal"));
        }
    } else {
        return Err(format!("the first argument most be a literal"));
    }
}
fn ak_div(vs: Vec<Value>) -> Result<Value, String> {
    let first = vs.get(0).expect("the first argument is required");
    let second = vs.get(1).expect("the second argument is required");

    if let Value::Literal(first) = first {
        if let Value::Literal(second) = second {
            return Ok(Value::Literal((first.clone() / second.clone())?));
        } else {
            return Err(format!("the first argument most be a literal"));
        }
    } else {
        return Err(format!("the first argument most be a literal"));
    }
}
fn ak_sub(vs: Vec<Value>) -> Result<Value, String> {
    let first = vs.get(0).expect("the first argument is required");
    let second = vs.get(1).expect("the second argument is required");

    if let Value::Literal(first) = first {
        if let Value::Literal(second) = second {
            return Ok(Value::Literal((first.clone() - second.clone())?));
        } else {
            return Err(format!("the first argument most be a literal"));
        }
    } else {
        return Err(format!("the first argument most be a literal"));
    }
}
fn ak_mul(vs: Vec<Value>) -> Result<Value, String> {
    let first = vs.get(0).expect("the first argument is required");
    let second = vs.get(1).expect("the second argument is required");

    if let Value::Literal(first) = first {
        if let Value::Literal(second) = second {
            return Ok(Value::Literal((first.clone() * second.clone())?));
        } else {
            return Err(format!("the first argument most be a literal"));
        }
    } else {
        return Err(format!("the first argument most be a literal"));
    }
}

fn ak_cos(vs: Vec<Value>) -> Result<Value, String> {
    let first = vs.get(0).expect("the first argument is required");

    if let Value::Literal(first) = first {
        match first {
            Literal::Int(n) => return Ok(Value::Literal(Literal::Float((*n as f32).cos()))),
            Literal::Float(n) => return Ok(Value::Literal(Literal::Float((*n).cos()))),
            Literal::String(_) => return Err(format!("cosinus on string no suported")),
        }
    } else {
        return Err(format!("invalid argument"));
    }
}

fn ak_len(vs: Vec<Value>) -> Result<Value, String> {
    let first = vs.get(0).expect("the first argument is required");

    if let Value::Literal(l) = first {
        match l {
            Literal::String(s) => return Ok(Value::Literal(Literal::Int(s.len() as i32))),
            _ => return Err(format!("the first argument most be string")),
        }
    } else {
        return Err(format!("the first argument most be string"));
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

    let modules: Vec<Export> = vec![Export::Module {
        name: String::from("std"),
        exports: vec![
            Export::Module {
                name: String::from("math"),
                exports: vec![
                    Export::Module {
                        name: String::from("consts"),
                        exports: vec![Export::Item {
                            name: String::from("PI"),
                            value: Value::Literal(Literal::Float(PI)),
                        }],
                    },
                    Export::Item {
                        name: String::from("add"),
                        value: Value::BuiltInFn(ak_add),
                    },
                    Export::Item {
                        name: String::from("mul"),
                        value: Value::BuiltInFn(ak_mul),
                    },
                    Export::Item {
                        name: String::from("div"),
                        value: Value::BuiltInFn(ak_div),
                    },
                    Export::Item {
                        name: String::from("sub"),
                        value: Value::BuiltInFn(ak_sub),
                    },
                    Export::Item {
                        name: String::from("cos"),
                        value: Value::BuiltInFn(ak_cos),
                    },
                ],
            },
            Export::Module {
                name: String::from("string"),
                exports: vec![Export::Item {
                    name: String::from("len"),
                    value: Value::BuiltInFn(ak_len),
                }],
            },
        ],
    }];

    let code = fs::read_to_string("./examples/test.ak").expect("unable to read the file");
    let parser = grammar::programParser::new();
    let ast = parser.parse(&code).expect("unable to parse the grammar");

    // println!("{:#?}", ast);
    eval_program(&mut env, ast, modules)
    // println!("program result: {:?}", result);
}
