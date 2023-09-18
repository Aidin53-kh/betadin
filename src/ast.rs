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
    Let(String, Expr),
    Const(String, Expr),
    Expression(Expr),
    Assignment(String, Expr),
    Import(Vec<String>, Option<Vec<String>>),
    If(Vec<Branch>, Option<Block>),
    Return(Expr),
    Fn(String, Vec<String>, Block),
    Module(String, Block),
    For(String, Expr, Block),
    While(Expr, Block),
    Break,
    Continue,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Expr {
    Null,
    Int(i32),
    Float(f32),
    String(String),
    Bool(bool),
    List(Vec<Expr>),
    Object(Vec<Prop>),
    Identifier(String),
    Call(Box<Expr>, Vec<Expr>),
    MethodCall(Box<Expr>, Box<Expr>),
    ModuleCall(Vec<String>, Box<Expr>),
    Index(Box<Expr>, Box<Expr>),
    BinaryOp(Box<Expr>, BinaryOpKind, Box<Expr>),
    UnaryOp(UnaryOpKind, Box<Expr>),
    Fn(Vec<String>, Block),
    Module(Block),
    If(Vec<Branch>, Option<Block>),
    Tuple(Vec<Expr>),
    Range(Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Prop {
    pub key: String,
    pub value: Expr,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Branch {
    pub condition: Expr,
    pub statements: Block,
}

impl Branch {
    pub fn new(condition: Expr, statements: Block) -> Self {
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
    Typeof,
}
