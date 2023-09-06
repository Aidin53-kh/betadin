use runtime::eval::program::eval_program;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Sub};
use std::{collections::HashMap, f32::consts::PI, fs};

#[macro_use]
extern crate lalrpop_util;

lalrpop_util::lalrpop_mod!(pub grammar);

mod ast;
mod runtime;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Int(i32),
    Float(f32),
    String(String),
    List(Vec<Value>),
    BuiltInFn(fn(Vec<Value>) -> Result<Value, String>),
}

fn value_list(values: Vec<Value>) -> String {
    let mut res: String = String::new();

    for (i, value) in values.iter().enumerate() {
        if i != 0 {
            res.push_str(", ");
        }

        res.push_str(&value.to_string());
    }

    res
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Int(n) => write!(f, "{}", n),
            Value::Float(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::BuiltInFn(_) => write!(f, "function"),
            Value::List(v) => write!(f, "[{}]", value_list(v.to_vec())),
        }
    }
}
impl Add for &Value {
    type Output = Result<Value, String>;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Value::Null => Err(format!("cannot add null with any thing")),
            Value::Int(lhs) => match rhs {
                Value::Null => Err(format!("cannot add int with null")),
                Value::Int(rhs) => Ok(Value::Int(lhs + rhs)),
                Value::Float(rhs) => Ok(Value::Float(*lhs as f32 + rhs)),
                Value::String(rhs) => Ok(Value::String(lhs.to_string() + &rhs)),
                Value::BuiltInFn(_) => return Err(format!("cannot int with function")),
                Value::List(_) => todo!(),
            },
            Value::Float(lhs) => match rhs {
                Value::Null => Err(format!("cannot add float with null")),
                Value::Int(rhs) => Ok(Value::Float(lhs + *rhs as f32)),
                Value::Float(rhs) => Ok(Value::Float(lhs + rhs)),
                Value::String(rhs) => Ok(Value::String(lhs.to_string() + &rhs)),
                Value::BuiltInFn(_) => return Err(format!("cannot float with function")),
                Value::List(_) => todo!(),
            },
            Value::String(lhs) => match rhs {
                Value::Null => Err(format!("cannot add string with null")),
                Value::Int(rhs) => Ok(Value::String(lhs.to_owned() + &rhs.to_string().to_owned())),
                Value::Float(rhs) => {
                    Ok(Value::String(lhs.to_owned() + &rhs.to_string().to_owned()))
                }
                Value::String(rhs) => Ok(Value::String(lhs.to_owned() + rhs)),
                Value::BuiltInFn(_) => return Err(format!("cannot string with function")),
                Value::List(_) => todo!(),
            },
            Value::BuiltInFn(_) => return Err(format!("cannot add function with anything")),
            Value::List(_) => todo!(),
        }
    }
}
impl Mul for &Value {
    type Output = Result<Value, String>;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Value::Null => Err(format!("cannot mul null with any thing")),
            Value::Int(lhs) => match rhs {
                Value::Null => Err(format!("cannot mul int with null")),
                Value::Int(rhs) => Ok(Value::Int(lhs * rhs)),
                Value::Float(rhs) => Ok(Value::Float(*lhs as f32 * rhs)),
                Value::String(_) => Err(format!("cannot mul int with string")),
                Value::BuiltInFn(_) => Err(format!("cannot mul int with function")),
                Value::List(_) => todo!(),
            },
            Value::Float(lhs) => match rhs {
                Value::Null => Err(format!("cannot mul float with null")),
                Value::Int(rhs) => Ok(Value::Float(lhs * *rhs as f32)),
                Value::Float(rhs) => Ok(Value::Float(lhs * rhs)),
                Value::String(_) => Err(format!("cannot mul float with string")),
                Value::BuiltInFn(_) => return Err(format!("cannot nul float with function")),
                Value::List(_) => todo!(),
            },
            Value::String(_) => match rhs {
                Value::Null => Err(format!("cannot mul string with null")),
                Value::Int(_) => Err(format!("cannot mul string with int")),
                Value::Float(_) => Err(format!("cannot mul string with float")),
                Value::String(_) => Err(format!("cannot mul string with string")),
                Value::BuiltInFn(_) => return Err(format!("cannot mul string with function")),
                Value::List(_) => todo!(),
            },
            Value::BuiltInFn(_) => return Err(format!("cannot mul function with anything")),
            Value::List(_) => todo!(),
        }
    }
}
impl Div for &Value {
    type Output = Result<Value, String>;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Value::Null => Err(format!("cannot div null with any thing")),
            Value::Int(lhs) => match rhs {
                Value::Null => Err(format!("cannot div int with null")),
                Value::Int(rhs) => Ok(Value::Int(lhs / rhs)),
                Value::Float(rhs) => Ok(Value::Float(*lhs as f32 / rhs)),
                Value::String(_) => Err(format!("cannot div int with string")),
                Value::BuiltInFn(_) => Err(format!("cannot div int with function")),
                Value::List(_) => todo!(),
            },
            Value::Float(lhs) => match rhs {
                Value::Null => Err(format!("cannot div float with null")),
                Value::Int(rhs) => Ok(Value::Float(lhs / *rhs as f32)),
                Value::Float(rhs) => Ok(Value::Float(lhs / rhs)),
                Value::String(_) => Err(format!("cannot div float with string")),
                Value::BuiltInFn(_) => return Err(format!("cannot nul float with function")),
                Value::List(_) => todo!(),
            },
            Value::String(_) => match rhs {
                Value::Null => Err(format!("cannot div string with null")),
                Value::Int(_) => Err(format!("cannot div string with int")),
                Value::Float(_) => Err(format!("cannot div string with float")),
                Value::String(_) => Err(format!("cannot div string with string")),
                Value::BuiltInFn(_) => return Err(format!("cannot div string with function")),
                Value::List(_) => todo!(),
            },
            Value::BuiltInFn(_) => return Err(format!("cannot div function with anything")),
            Value::List(_) => todo!(),
        }
    }
}
impl Sub for &Value {
    type Output = Result<Value, String>;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Value::Null => Err(format!("cannot sub null with any thing")),
            Value::Int(lhs) => match rhs {
                Value::Null => Err(format!("cannot sub int with null")),
                Value::Int(rhs) => Ok(Value::Int(lhs / rhs)),
                Value::Float(rhs) => Ok(Value::Float(*lhs as f32 / rhs)),
                Value::String(_) => Err(format!("cannot sub int with string")),
                Value::BuiltInFn(_) => Err(format!("cannot sub int with function")),
                Value::List(_) => todo!(),
            },
            Value::Float(lhs) => match rhs {
                Value::Null => Err(format!("cannot sub float with null")),
                Value::Int(rhs) => Ok(Value::Float(lhs / *rhs as f32)),
                Value::Float(rhs) => Ok(Value::Float(lhs / rhs)),
                Value::String(_) => Err(format!("cannot sub float with string")),
                Value::BuiltInFn(_) => return Err(format!("cannot nul float with function")),
                Value::List(_) => todo!(),
            },
            Value::String(_) => match rhs {
                Value::Null => Err(format!("cannot sub string with null")),
                Value::Int(_) => Err(format!("cannot sub string with int")),
                Value::Float(_) => Err(format!("cannot sub string with float")),
                Value::String(_) => Err(format!("cannot sub string with string")),
                Value::BuiltInFn(_) => return Err(format!("cannot sub string with function")),
                Value::List(_) => todo!(),
            },
            Value::BuiltInFn(_) => return Err(format!("cannot sub function with anything")),
            Value::List(_) => todo!(),
        }
    }
}
fn ak_print(vs: Vec<Value>) -> Result<Value, String> {
    match vs.get(0) {
        Some(value) => {
            print!("{}", value);
            return Ok(Value::Null);
        }
        None => return Err(format!("the first argument is required")),
    }
}
fn ak_println(vs: Vec<Value>) -> Result<Value, String> {
    match vs.get(0) {
        Some(value) => {
            println!("{}", value);
            return Ok(Value::Null);
        }
        None => return Err(format!("the first argument is required")),
    }
}
fn ak_add(vs: Vec<Value>) -> Result<Value, String> {
    match vs.get(0) {
        Some(value1) => match vs.get(1) {
            Some(value2) => value1 + value2,
            None => return Err(format!("the second argument is require")),
        },
        None => return Err(format!("the first argument is require")),
    }
}
fn ak_div(vs: Vec<Value>) -> Result<Value, String> {
    match vs.get(0) {
        Some(value1) => match vs.get(1) {
            Some(value2) => value1 / value2,
            None => return Err(format!("the second argument is require")),
        },
        None => return Err(format!("the first argument is require")),
    }
}
fn ak_sub(vs: Vec<Value>) -> Result<Value, String> {
    match vs.get(0) {
        Some(value1) => match vs.get(1) {
            Some(value2) => value1 - value2,
            None => return Err(format!("the second argument is require")),
        },
        None => return Err(format!("the first argument is require")),
    }
}
fn ak_mul(vs: Vec<Value>) -> Result<Value, String> {
    match vs.get(0) {
        Some(value1) => match vs.get(1) {
            Some(value2) => value1 * value2,
            None => return Err(format!("the second argument is require")),
        },
        None => return Err(format!("the first argument is require")),
    }
}
fn ak_cos(vs: Vec<Value>) -> Result<Value, String> {
    match vs.get(0) {
        Some(value) => match value {
            Value::Int(n) => return Ok(Value::Float((*n as f32).cos())),
            Value::Float(n) => return Ok(Value::Float(n.cos())),
            _ => return Err(format!("the first argument most be a number")),
        },
        None => Err(format!("the first argument is required")),
    }
}
fn ak_sin(vs: Vec<Value>) -> Result<Value, String> {
    match vs.get(0) {
        Some(value) => match value {
            Value::Int(n) => return Ok(Value::Float((*n as f32).sin())),
            Value::Float(n) => return Ok(Value::Float(n.sin())),
            _ => return Err(format!("the first argument most be a number")),
        },
        None => Err(format!("the first argument is required")),
    }
}
fn ak_tan(vs: Vec<Value>) -> Result<Value, String> {
    match vs.get(0) {
        Some(value) => match value {
            Value::Int(n) => return Ok(Value::Float((*n as f32).tan())),
            Value::Float(n) => return Ok(Value::Float(n.tan())),
            _ => return Err(format!("the first argument most be a number")),
        },
        None => Err(format!("the first argument is required")),
    }
}
fn ak_abs(vs: Vec<Value>) -> Result<Value, String> {
    match vs.get(0) {
        Some(value) => match value {
            Value::Int(n) => return Ok(Value::Float((*n as f32).abs())),
            Value::Float(n) => return Ok(Value::Float(n.abs())),
            _ => return Err(format!("the first argument most be a number")),
        },
        None => Err(format!("the first argument is required")),
    }
}
fn ak_pow(vs: Vec<Value>) -> Result<Value, String> {
    match vs.get(0) {
        Some(value1) => match value1 {
            Value::Int(n1) => match vs.get(1) {
                Some(value2) => match value2 {
                    Value::Int(n2) => Ok(Value::Float((*n1 as f32).powi(*n2))),
                    Value::Float(n2) => Ok(Value::Float((*n1 as f32).powf(*n2))),
                    _ => return Err(format!("the second argument most be a number")),
                },
                None => return Err(format!("the second argument is required")),
            },
            Value::Float(n1) => match vs.get(1) {
                Some(value2) => match value2 {
                    Value::Int(n2) => Ok(Value::Float(n1.powi(*n2))),
                    Value::Float(n2) => Ok(Value::Float(n1.powf(*n2))),
                    _ => return Err(format!("the second argument most be a number")),
                },
                None => return Err(format!("the second argument is required")),
            },
            _ => return Err(format!("the first argument most be a number")),
        },
        None => Err(format!("the first argument is required")),
    }
}
fn ak_len(vs: Vec<Value>) -> Result<Value, String> {
    match vs.get(0) {
        Some(value) => {
            if let Value::String(s) = value {
                return Ok(Value::Int(s.len() as i32));
            } else {
                return Err(format!("the first argument most be string"));
            }
        }
        None => Err(format!("the first argument is required")),
    }
}

fn ak_set(vs: Vec<Value>) -> Result<Value, String> {
    match vs.get(0) {
        Some(value) => {
            if let Value::List(list) = value {
                let mut set = Vec::new();
                for val in list.clone() {
                    if !set.contains(&val) {
                        set.push(val);
                    }
                }

                return Ok(Value::List(set));
            } else {
                return Err(format!("the first argument most be a list"));
            }
        }
        None => Err(format!("the first argument is required")),
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
                            value: Value::Float(PI),
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
                    Export::Item {
                        name: String::from("sin"),
                        value: Value::BuiltInFn(ak_sin),
                    },
                    Export::Item {
                        name: String::from("abs"),
                        value: Value::BuiltInFn(ak_abs),
                    },
                    Export::Item {
                        name: String::from("tan"),
                        value: Value::BuiltInFn(ak_tan),
                    },
                    Export::Item {
                        name: String::from("pow"),
                        value: Value::BuiltInFn(ak_pow),
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
            Export::Module {
                name: String::from("collections"),
                exports: vec![Export::Item {
                    name: String::from("set"),
                    value: Value::BuiltInFn(ak_set),
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
