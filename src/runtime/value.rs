use std::collections::BTreeMap;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Not, Sub};

use crate::ast::{Arg, Block};

use super::{Simple, Type};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Null,
    Int(i32),
    Float(f32),
    String(String),
    Bool(bool),
    List(Vec<Value>),
    Object(Vec<KeyValue>),
    BuiltInFn(fn(Vec<Value>) -> Result<Value, String>),
    BuiltInMethod(
        fn(Vec<Value>, Value) -> Result<Value, String>,
        Option<Box<Value>>,
    ),
    Func(Vec<Arg>, Option<Type>, Block),
    Module(BTreeMap<String, Value>),
    Tuple(Vec<Value>),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct KeyValue {
    pub key: String,
    pub value: Value,
}

pub fn check_list_items(list: &Vec<Value>) -> Result<(), String> {
    if let Some(value) = list.get(0) {
        for item in list {
            if Type::from(item) != Type::from(value) {
                return Err(format!(
                    "expected {}, found {}. help: all list items most have same datatype",
                    Type::simple(value),
                    Type::simple(item)
                ));
            }
        }
    }

    Ok(())
}

pub fn value_list(values: Vec<Value>) -> String {
    let mut res = String::new();

    for (i, value) in values.iter().enumerate() {
        if i != 0 {
            res.push_str(", ");
        }
        res.push_str(&value.to_string());
    }

    res
}

pub fn key_value(obj: Vec<KeyValue>) -> String {
    let mut res = String::new();

    for prop in obj {
        res.push('\t');
        res.push_str(&prop.key);
        res.push_str(": ");
        res.push_str(&prop.value.to_string());
        res.push(',');
        res.push('\n');
    }

    res
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Hash)]
pub enum BuiltinType {
    Null,
    Int,
    Float,
    Bool,
    String,
    List(Box<Type>),
    Tuple(Vec<Type>),
    Fn(Vec<Type>, Box<Type>),
}

impl From<&Value> for Value {
    fn from(value: &Value) -> Self {
        match value {
            Value::Null => Value::Null,
            Value::Int(n) => Value::Int(*n),
            Value::Float(n) => Value::Float(*n),
            Value::String(s) => Value::String(s.to_string()),
            Value::Bool(b) => Value::Bool(*b),
            Value::List(l) => Value::List(l.to_vec()),
            Value::BuiltInFn(f) => Value::BuiltInFn(*f),
            Value::BuiltInMethod(f, this) => Value::BuiltInMethod(*f, this.clone()),
            Value::Func(args, ret_type, block) => {
                Value::Func(args.to_vec(), ret_type.clone(), block.to_vec())
            }
            Value::Object(props) => Value::Object(props.to_vec()),
            Value::Module(items) => Value::Module(items.to_owned()),
            Value::Tuple(t) => Value::Tuple(t.to_vec()),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Int(n) => write!(f, "{}", n),
            Value::Float(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Bool(b) => write!(f, "{}", b),
            Value::BuiltInFn(_) => write!(f, "function"),
            Value::List(v) => write!(f, "[{}]", value_list(v.to_vec())),
            Value::BuiltInMethod(_, _) => write!(f, "function"),
            Value::Func(..) => write!(f, "function"),
            Value::Object(obj) => write!(f, "{{\n{}}}", key_value(obj.to_vec())),
            Value::Module(_) => write!(f, "module"),
            Value::Tuple(t) => write!(f, "({})", value_list(t.to_vec())),
        }
    }
}

impl From<Type> for Value {
    fn from(value: Type) -> Self {
        match value {
            Type::Custom(_) => todo!(),
            Type::Builtin(t) => match t {
                BuiltinType::Null => Value::Null,
                BuiltinType::Int => Value::Int(i32::default()),
                BuiltinType::Float => Value::Float(f32::default()),
                BuiltinType::Bool => Value::Bool(bool::default()),
                BuiltinType::String => Value::String(String::default()),
                BuiltinType::List(_) => Value::List(vec![]),
                BuiltinType::Tuple(_) => Value::Tuple(vec![]),
                BuiltinType::Fn(_, ret_type) => Value::Func(vec![], Some(*ret_type), vec![]),
            },
        }
    }
}

impl Display for BuiltinType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuiltinType::Null => write!(f, "null"),
            BuiltinType::Int => write!(f, "int"),
            BuiltinType::Float => write!(f, "float"),
            BuiltinType::Bool => write!(f, "bool"),
            BuiltinType::String => write!(f, "string"),
            BuiltinType::List(t) => {
                let datatype = *t.clone();

                return write!(f, "{}[]", datatype.to_string());
            }
            BuiltinType::Tuple(types) => {
                let mut res = String::new();

                for (i, t) in types.iter().enumerate() {
                    if i >= 1 {
                        res.push_str(", ");
                    }
                    res.push_str(&String::from(t.clone()));
                }

                return write!(f, "({})", res);
            }
            BuiltinType::Fn(args, ret_type) => {
                let mut args_types = String::new();
                let ret_type = *ret_type.clone();

                for (i, arg) in args.iter().enumerate() {
                    if i >= 1 {
                        args_types.push_str(", ");
                    }
                    args_types.push_str(&String::from(arg.clone()));
                }

                return write!(f, "fn({}) -> {}", args_types, ret_type.to_string());
            }
        }
    }
}

impl Not for Value {
    type Output = Result<Value, String>;

    fn not(self) -> Self::Output {
        match &self {
            Value::Bool(b) => Ok(Value::Bool(!b)),
            _ => Err(format!(
                "cannot apply unary operator '!' to type {}",
                Type::simple(&self)
            )),
        }
    }
}

impl Add for &Value {
    type Output = Result<Value, String>;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Value::Int(lhs) => match rhs {
                Value::Int(rhs) => Ok(Value::Int(lhs + rhs)),
                Value::Float(rhs) => Ok(Value::Float(*lhs as f32 + rhs)),
                Value::String(rhs) => Ok(Value::String(lhs.to_string() + &rhs)),
                other => Err(format!("cannot add int to {}", Type::simple(other))),
            },
            Value::Float(lhs) => match rhs {
                Value::Int(rhs) => Ok(Value::Float(lhs + *rhs as f32)),
                Value::Float(rhs) => Ok(Value::Float(lhs + rhs)),
                Value::String(rhs) => Ok(Value::String(lhs.to_string() + &rhs)),
                other => Err(format!("cannot add float to {}", Type::simple(other))),
            },
            Value::String(lhs) => match rhs {
                Value::Int(rhs) => Ok(Value::String(lhs.to_owned() + &rhs.to_string().to_owned())),
                Value::Float(rhs) => {
                    Ok(Value::String(lhs.to_owned() + &rhs.to_string().to_owned()))
                }
                Value::String(rhs) => Ok(Value::String(lhs.to_owned() + rhs)),
                Value::List(rhs) => Ok(Value::String(
                    lhs.to_owned() + Value::List(rhs.to_owned()).to_string().as_str(),
                )),
                other => Err(format!("cannot add stirng to {}", Type::simple(other))),
            },
            other => Err(format!(
                "cannot add {} to {}",
                Type::simple(other),
                Type::simple(rhs)
            )),
        }
    }
}

impl Mul for &Value {
    type Output = Result<Value, String>;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Value::Int(lhs) => match rhs {
                Value::Int(rhs) => Ok(Value::Int(lhs * rhs)),
                Value::Float(rhs) => Ok(Value::Float(*lhs as f32 * rhs)),
                other => Err(format!("cannot mul int to {}", Type::simple(other))),
            },
            Value::Float(lhs) => match rhs {
                Value::Int(rhs) => Ok(Value::Float(lhs * *rhs as f32)),
                Value::Float(rhs) => Ok(Value::Float(lhs * rhs)),
                other => Err(format!("cannot mul float to {}", Type::simple(other))),
            },
            other => Err(format!(
                "cannot mul {} to {}",
                Type::simple(other),
                Type::simple(rhs)
            )),
        }
    }
}

impl Div for &Value {
    type Output = Result<Value, String>;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Value::Int(lhs) => match rhs {
                Value::Int(rhs) => Ok(Value::Int(lhs / rhs)),
                Value::Float(rhs) => Ok(Value::Float(*lhs as f32 / rhs)),
                other => Err(format!("cannot div int to {}", Type::simple(other))),
            },
            Value::Float(lhs) => match rhs {
                Value::Int(rhs) => Ok(Value::Float(lhs / *rhs as f32)),
                Value::Float(rhs) => Ok(Value::Float(lhs / rhs)),
                other => Err(format!("cannot div float to {}", Type::simple(other))),
            },
            other => Err(format!(
                "cannot div {} to {}",
                Type::simple(other),
                Type::simple(rhs)
            )),
        }
    }
}

impl Sub for &Value {
    type Output = Result<Value, String>;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Value::Int(lhs) => match rhs {
                Value::Int(rhs) => Ok(Value::Int(lhs - rhs)),
                Value::Float(rhs) => Ok(Value::Float(*lhs as f32 - rhs)),
                other => Err(format!("cannot sub float to {}", Type::simple(other))),
            },
            Value::Float(lhs) => match rhs {
                Value::Int(rhs) => Ok(Value::Float(lhs - *rhs as f32)),
                Value::Float(rhs) => Ok(Value::Float(lhs - rhs)),
                other => Err(format!("cannot sub float to {}", Type::simple(other))),
            },
            other => Err(format!(
                "cannot sub {} to {}",
                Type::simple(other),
                Type::simple(rhs)
            )),
        }
    }
}
