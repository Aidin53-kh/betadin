use std::{
    fmt::{Display, Formatter, Result},
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    Call(String, Vec<Expression>),
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i32),
    Float(f32),
    String(String),
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Literal::Int(n) => write!(f, "{}", n),
            Literal::Float(n) => write!(f, "{}", n),
            Literal::String(s) => write!(f, "{}", s),
        }
    }
}

impl Add for Literal {
    type Output = core::result::Result<Literal, String>;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Literal::Int(lhs) => match rhs {
                Literal::Int(rhs) => Ok(Literal::Int(lhs + rhs)),
                Literal::Float(rhs) => Ok(Literal::Float(lhs as f32 + rhs)),
                Literal::String(rhs) => Ok(Literal::String(lhs.to_string() + &rhs)),
            },
            Literal::Float(lhs) => match rhs {
                Literal::Int(rhs) => Ok(Literal::Float(lhs + rhs as f32)),
                Literal::Float(rhs) => Ok(Literal::Float(lhs + rhs)),
                Literal::String(rhs) => Ok(Literal::String(lhs.to_string() + &rhs)),
            },
            Literal::String(lhs) => match rhs {
                Literal::Int(rhs) => Ok(Literal::String(lhs + &rhs.to_string())),
                Literal::Float(rhs) => Ok(Literal::String(lhs + &rhs.to_string())),
                Literal::String(rhs) => Ok(Literal::String(lhs + &rhs)),
            },
        }
    }
}

impl Div for Literal {
    type Output = core::result::Result<Literal, String>;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Literal::Int(lhs) => match rhs {
                Literal::Int(rhs) => Ok(Literal::Int(lhs / rhs)),
                Literal::Float(rhs) => Ok(Literal::Float(lhs as f32 / rhs)),
                Literal::String(_) => Err(format!("cannot divide int to string")),
            },
            Literal::Float(lhs) => match rhs {
                Literal::Int(rhs) => Ok(Literal::Float(lhs / rhs as f32)),
                Literal::Float(rhs) => Ok(Literal::Float(lhs / rhs)),
                Literal::String(_) => Err(format!("cannot divide float to string")),
            },
            Literal::String(_) => match rhs {
                Literal::Int(_) => Err(format!("cannot divide string to int")),
                Literal::Float(_) => Err(format!("cannot divide string to float")),
                Literal::String(_) => Err(format!("cannot divide string to string")),
            },
        }
    }
}

impl Mul for Literal {
    type Output = core::result::Result<Literal, String>;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Literal::Int(lhs) => match rhs {
                Literal::Int(rhs) => Ok(Literal::Int(lhs * rhs)),
                Literal::Float(rhs) => Ok(Literal::Float(lhs as f32 * rhs)),
                Literal::String(_) => Err(format!("cannot mul int to string")),
            },
            Literal::Float(lhs) => match rhs {
                Literal::Int(rhs) => Ok(Literal::Float(lhs * rhs as f32)),
                Literal::Float(rhs) => Ok(Literal::Float(lhs * rhs)),
                Literal::String(_) => Err(format!("cannot mul float to string")),
            },
            Literal::String(_) => match rhs {
                Literal::Int(_) => Err(format!("cannot mul string to int")),
                Literal::Float(_) => Err(format!("cannot mul string to float")),
                Literal::String(_) => Err(format!("cannot mul string to string")),
            },
        }
    }
}

impl Sub for Literal {
    type Output = core::result::Result<Literal, String>;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Literal::Int(lhs) => match rhs {
                Literal::Int(rhs) => Ok(Literal::Int(lhs - rhs)),
                Literal::Float(rhs) => Ok(Literal::Float(lhs as f32 - rhs)),
                Literal::String(_) => Err(format!("cannot sub int to string")),
            },
            Literal::Float(lhs) => match rhs {
                Literal::Int(rhs) => Ok(Literal::Float(lhs - rhs as f32)),
                Literal::Float(rhs) => Ok(Literal::Float(lhs - rhs)),
                Literal::String(_) => Err(format!("cannot sub float to string")),
            },
            Literal::String(_) => match rhs {
                Literal::Int(_) => Err(format!("cannot sub string to int")),
                Literal::Float(_) => Err(format!("cannot sub string to float")),
                Literal::String(_) => Err(format!("cannot sub string to string")),
            },
        }
    }
}
