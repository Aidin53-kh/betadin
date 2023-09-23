use std::collections::BTreeMap;
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
    Module(BTreeMap<String, Value>),
    Tuple(Vec<Value>),
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
    Module,
    Tuple,
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
            Value::Module(_) => Type::Module,
            Value::Tuple(_) => Type::Tuple,
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
            Type::Module => write!(f, "module"),
            Type::Tuple => write!(f, "tuple"),
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
            Value::BuiltInFn(_) => write!(f, "fn"),
            Value::List(v) => write!(f, "[{}]", value_list(v.to_vec())),
            Value::BuiltInMethod(_, _) => write!(f, "fn"),
            Value::Func(_, _) => write!(f, "fn"),
            Value::Object(obj) => write!(f, "{{\n{}}}", key_value(obj.to_vec())),
            Value::Module(_) => write!(f, "module"),
            Value::Tuple(t) => write!(f, "({})", value_list(t.to_vec())),
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
            Value::Int(lhs) => match rhs {
                Value::Int(rhs) => Ok(Value::Int(lhs + rhs)),
                Value::Float(rhs) => Ok(Value::Float(*lhs as f32 + rhs)),
                Value::String(rhs) => Ok(Value::String(lhs.to_string() + &rhs)),
                other => Err(format!("cannot add int to {}", Type::from(other))),
            },
            Value::Float(lhs) => match rhs {
                Value::Int(rhs) => Ok(Value::Float(lhs + *rhs as f32)),
                Value::Float(rhs) => Ok(Value::Float(lhs + rhs)),
                Value::String(rhs) => Ok(Value::String(lhs.to_string() + &rhs)),
                other => Err(format!("cannot add float to {}", Type::from(other))),
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
                other => Err(format!("cannot add stirng to {}", Type::from(other))),
            },
            other => Err(format!(
                "cannot add {} to {}",
                Type::from(other),
                Type::from(rhs)
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
                other => Err(format!("cannot mut int to {}", Type::from(other))),
            },
            Value::Float(lhs) => match rhs {
                Value::Int(rhs) => Ok(Value::Float(lhs * *rhs as f32)),
                Value::Float(rhs) => Ok(Value::Float(lhs * rhs)),
                other => Err(format!("cannot mut float to {}", Type::from(other))),
            },
            other => Err(format!(
                "cannot mut {} to {}",
                Type::from(other),
                Type::from(rhs)
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
                other => Err(format!("cannot div int to {}", Type::from(other))),
            },
            Value::Float(lhs) => match rhs {
                Value::Int(rhs) => Ok(Value::Float(lhs / *rhs as f32)),
                Value::Float(rhs) => Ok(Value::Float(lhs / rhs)),
                other => Err(format!("cannot div float to {}", Type::from(other))),
            },
            other => Err(format!(
                "cannot div {} to {}",
                Type::from(other),
                Type::from(rhs)
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
                other => Err(format!("cannot sub float to {}", Type::from(other))),
            },
            Value::Float(lhs) => match rhs {
                Value::Int(rhs) => Ok(Value::Float(lhs - *rhs as f32)),
                Value::Float(rhs) => Ok(Value::Float(lhs - rhs)),
                other => Err(format!("cannot sub float to {}", Type::from(other))),
            },
            other => Err(format!(
                "cannot sub {} to {}",
                Type::from(other),
                Type::from(rhs)
            )),
        }
    }
}
