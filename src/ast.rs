#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    LetStatement(String, Expression),
    ExpressionStatement(Expression),
    AssignmentStatement(String, Expression),
    ImportStatement(Vec<String>),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Int(i32),
    Float(f32),
    String(String),
    Bool(bool),
    List(Vec<Expression>),
    Identifier(String),
    Call(String, Vec<Expression>),
    MethodCall(Box<Expression>, Box<Expression>),
    Index(Box<Expression>, Box<Expression>),
}
