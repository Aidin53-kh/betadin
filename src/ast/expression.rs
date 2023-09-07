use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    Call(String, Vec<Expression>),
    MethodCall(Box<Expression>, Box<Expression>),
    Index(Box<Expression>, Box<Expression>)
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
            Literal::List(_) => write!(f, "list"),
        }
    }
}
