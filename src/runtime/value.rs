use std::fmt::Display;
use std::ops::{Add, Div, Mul, Not, Sub};

use crate::ast::Block;

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
    Func(Vec<String>, Block),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct KeyValue {
    pub key: String,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Null,
    Int,
    Float,
    String,
    List,
    Func,
    Bool,
    Object,
}

pub fn value_list(values: Vec<Value>) -> String {
    let mut res: String = String::new();

    for (i, value) in values.iter().enumerate() {
        if i != 0 {
            res.push_str(", ");
        }
        res.push_str(&value.to_string());
    }

    res
}

impl From<&Value> for Type {
    fn from(value: &Value) -> Self {
        match value {
            Value::Null => Type::Null,
            Value::Int(_) => Type::Int,
            Value::Float(_) => Type::Float,
            Value::String(_) => Type::String,
            Value::Bool(_) => Type::Bool,
            Value::List(_) => Type::List,
            Value::BuiltInFn(_) => Type::Func,
            Value::BuiltInMethod(_, _) => Type::Func,
            Value::Func(_, _) => Type::Func,
            Value::Object(_) => Type::Object,
        }
    }
}
impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Null => write!(f, "null"),
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::String => write!(f, "string"),
            Type::List => write!(f, "list"),
            Type::Func => write!(f, "function"),
            Type::Bool => write!(f, "bool"),
            Type::Object => write!(f, "object"),
        }
    }
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
            Value::Func(args, block) => Value::Func(args.to_vec(), block.to_vec()),
            Value::Object(props) => Value::Object(props.to_vec()),
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
            Value::Func(_, _) => write!(f, "function"),
            Value::Object(_) => write!(f, "{{ ... }}"),
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
                Type::from(&self)
            )),
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
                Value::BuiltInMethod(_, _) => todo!(),
                Value::Bool(_) => todo!(),
                Value::Func(_, _) => todo!(),
                Value::Object(_) => todo!(),
            },
            Value::Float(lhs) => match rhs {
                Value::Null => Err(format!("cannot add float with null")),
                Value::Int(rhs) => Ok(Value::Float(lhs + *rhs as f32)),
                Value::Float(rhs) => Ok(Value::Float(lhs + rhs)),
                Value::String(rhs) => Ok(Value::String(lhs.to_string() + &rhs)),
                Value::BuiltInFn(_) => return Err(format!("cannot float with function")),
                Value::List(_) => todo!(),
                Value::BuiltInMethod(_, _) => todo!(),
                Value::Bool(_) => todo!(),
                Value::Func(_, _) => todo!(),
                Value::Object(_) => todo!(),
            },
            Value::String(lhs) => match rhs {
                Value::Null => Err(format!("cannot add string with null")),
                Value::Int(rhs) => Ok(Value::String(lhs.to_owned() + &rhs.to_string().to_owned())),
                Value::Float(rhs) => {
                    Ok(Value::String(lhs.to_owned() + &rhs.to_string().to_owned()))
                }
                Value::String(rhs) => Ok(Value::String(lhs.to_owned() + rhs)),
                Value::BuiltInFn(_) => return Err(format!("cannot add string with function")),
                Value::List(rhs) => Ok(Value::String(
                    lhs.to_owned() + Value::List(rhs.to_owned()).to_string().as_str(),
                )),
                Value::BuiltInMethod(_, _) => todo!(),
                Value::Bool(_) => todo!(),
                Value::Func(_, _) => todo!(),
                Value::Object(_) => todo!(),
            },
            Value::BuiltInFn(_) => return Err(format!("cannot add function with anything")),
            Value::List(_) => todo!(),
            Value::BuiltInMethod(_, _) => todo!(),
            Value::Bool(_) => todo!(),
            Value::Func(_, _) => todo!(),
            Value::Object(_) => todo!(),
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
                Value::BuiltInMethod(_, _) => todo!(),
                Value::Bool(_) => todo!(),
                Value::Func(_, _) => todo!(),
                Value::Object(_) => todo!(),
            },
            Value::Float(lhs) => match rhs {
                Value::Null => Err(format!("cannot mul float with null")),
                Value::Int(rhs) => Ok(Value::Float(lhs * *rhs as f32)),
                Value::Float(rhs) => Ok(Value::Float(lhs * rhs)),
                Value::String(_) => Err(format!("cannot mul float with string")),
                Value::BuiltInFn(_) => return Err(format!("cannot nul float with function")),
                Value::List(_) => todo!(),
                Value::BuiltInMethod(_, _) => todo!(),
                Value::Bool(_) => todo!(),
                Value::Func(_, _) => todo!(),
                Value::Object(_) => todo!(),
            },
            Value::String(_) => match rhs {
                Value::Null => Err(format!("cannot mul string with null")),
                Value::Int(_) => Err(format!("cannot mul string with int")),
                Value::Float(_) => Err(format!("cannot mul string with float")),
                Value::String(_) => Err(format!("cannot mul string with string")),
                Value::BuiltInFn(_) => return Err(format!("cannot mul string with function")),
                Value::List(_) => todo!(),
                Value::BuiltInMethod(_, _) => todo!(),
                Value::Bool(_) => todo!(),
                Value::Func(_, _) => todo!(),
                Value::Object(_) => todo!(),
            },
            Value::BuiltInFn(_) => return Err(format!("cannot mul function with anything")),
            Value::List(_) => todo!(),
            Value::BuiltInMethod(_, _) => todo!(),
            Value::Bool(_) => todo!(),
            Value::Func(_, _) => todo!(),
            Value::Object(_) => todo!(),
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
                Value::BuiltInMethod(_, _) => todo!(),
                Value::Bool(_) => todo!(),
                Value::Func(_, _) => todo!(),
                Value::Object(_) => todo!(),
            },
            Value::Float(lhs) => match rhs {
                Value::Null => Err(format!("cannot div float with null")),
                Value::Int(rhs) => Ok(Value::Float(lhs / *rhs as f32)),
                Value::Float(rhs) => Ok(Value::Float(lhs / rhs)),
                Value::String(_) => Err(format!("cannot div float with string")),
                Value::BuiltInFn(_) => return Err(format!("cannot nul float with function")),
                Value::List(_) => todo!(),
                Value::BuiltInMethod(_, _) => todo!(),
                Value::Bool(_) => todo!(),
                Value::Func(_, _) => todo!(),
                Value::Object(_) => todo!(),
            },
            Value::String(_) => match rhs {
                Value::Null => Err(format!("cannot div string with null")),
                Value::Int(_) => Err(format!("cannot div string with int")),
                Value::Float(_) => Err(format!("cannot div string with float")),
                Value::String(_) => Err(format!("cannot div string with string")),
                Value::BuiltInFn(_) => return Err(format!("cannot div string with function")),
                Value::List(_) => todo!(),
                Value::BuiltInMethod(_, _) => todo!(),
                Value::Bool(_) => todo!(),
                Value::Func(_, _) => todo!(),
                Value::Object(_) => todo!(),
            },
            Value::BuiltInFn(_) => return Err(format!("cannot div function with anything")),
            Value::List(_) => todo!(),
            Value::BuiltInMethod(_, _) => todo!(),
            Value::Bool(_) => todo!(),
            Value::Func(_, _) => todo!(),
            Value::Object(_) => todo!(),
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
                Value::Int(rhs) => Ok(Value::Int(lhs - rhs)),
                Value::Float(rhs) => Ok(Value::Float(*lhs as f32 - rhs)),
                Value::String(_) => Err(format!("cannot sub int with string")),
                Value::BuiltInFn(_) => Err(format!("cannot sub int with function")),
                Value::List(_) => todo!(),
                Value::BuiltInMethod(_, _) => todo!(),
                Value::Bool(_) => todo!(),
                Value::Func(_, _) => todo!(),
                Value::Object(_) => todo!(),
            },
            Value::Float(lhs) => match rhs {
                Value::Null => Err(format!("cannot sub float with null")),
                Value::Int(rhs) => Ok(Value::Float(lhs - *rhs as f32)),
                Value::Float(rhs) => Ok(Value::Float(lhs - rhs)),
                Value::String(_) => Err(format!("cannot sub float with string")),
                Value::BuiltInFn(_) => return Err(format!("cannot nul float with function")),
                Value::List(_) => todo!(),
                Value::BuiltInMethod(_, _) => todo!(),
                Value::Bool(_) => todo!(),
                Value::Func(_, _) => todo!(),
                Value::Object(_) => todo!(),
            },
            Value::String(_) => match rhs {
                Value::Null => Err(format!("cannot sub string with null")),
                Value::Int(_) => Err(format!("cannot sub string with int")),
                Value::Float(_) => Err(format!("cannot sub string with float")),
                Value::String(_) => Err(format!("cannot sub string with string")),
                Value::BuiltInFn(_) => return Err(format!("cannot sub string with function")),
                Value::List(_) => todo!(),
                Value::BuiltInMethod(_, _) => todo!(),
                Value::Bool(_) => todo!(),
                Value::Func(_, _) => todo!(),
                Value::Object(_) => todo!(),
            },
            Value::BuiltInFn(_) => return Err(format!("cannot sub function with anything")),
            Value::List(_) => todo!(),
            Value::BuiltInMethod(_, _) => todo!(),
            Value::Bool(_) => todo!(),
            Value::Func(_, _) => todo!(),
            Value::Object(_) => todo!(),
        }
    }
}
