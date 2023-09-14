#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Statement {
    LetStatement(String, Expression),
    ConstStatement(String, Expression),
    ExpressionStatement(Expression),
    AssignmentStatement(String, Expression),
    ImportStatement(Vec<String>),
    IfStatement(Vec<Branch>, Option<Block>),
    ReturnStatement(Expression),
    FnStatement(String, Vec<String>, Block),
    ForStatement(String, Expression, Block),
    WhileStatement(Expression, Block),
    BreakStatement,
    ContinueStatement,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Expression {
    Null,
    Int(i32),
    Float(f32),
    String(String),
    Bool(bool),
    List(Vec<Expression>),
    Object(Vec<Prop>),
    Identifier(String),
    Call(Box<Expression>, Vec<Expression>),
    MethodCall(Box<Expression>, Box<Expression>),
    Index(Box<Expression>, Box<Expression>),
    BinaryOp(Box<Expression>, BinaryOpKind, Box<Expression>),
    UnaryOp(UnaryOpKind, Box<Expression>),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Prop {
    pub key: String,
    pub value: Expression
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Branch {
    pub condition: Expression,
    pub statements: Block,
}

impl Branch {
    pub fn new(condition: Expression, statements: Block) -> Self {
        Self {
            condition,
            statements,
        }
    }
}

pub type Block = Vec<Statement>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum BinaryOpKind {
    // arithmatic
    Add,
    Sub,
    Mul,
    Div,
    // relational
    EQ,
    NE,
    GT,
    GTE,
    LT,
    LTE,
    // logical
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum UnaryOpKind {
    Not,
    Typeof
}
