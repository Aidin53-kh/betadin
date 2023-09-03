#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal)
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i32),
    String(String)
}