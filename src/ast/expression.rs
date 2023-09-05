use std::fmt::{Display, Formatter, Result};

use crate::Value;

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
    List(Vec<Expression>),
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Literal::Int(n) => write!(f, "{}", n),
            Literal::Float(n) => write!(f, "{}", n),
            Literal::String(s) => write!(f, "{}", s),
            Literal::List(l) => write!(f, "list"),
        }
    }
}
