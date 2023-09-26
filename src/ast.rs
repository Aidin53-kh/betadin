use crate::runtime::Type;

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
    Let(String, Option<Type>, Expr),
    Const(String, Option<Type>, Expr),
    Expression(Expr),
    Assignment(String, Expr),
    Import(Vec<String>, Option<Vec<String>>),
    If(Vec<Branch>, Option<Block>),
    Return(Expr),
    Fn(String, Vec<Arg>, Option<Type>, Block),
    Module(String, Block),
    For(String, Expr, Block),
    While(Expr, Block),
    Type(String, Type),
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
    Fn(Vec<Arg>, Option<Type>, Block),
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
pub struct Arg {
    pub ident: String,
    pub datatype: Type,
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

    pub fn insert_to_branch_stmt(
        cond: Expr,
        if_block: Vec<Statement>,
        else_if_block: Statement,
    ) -> Statement {
        if let Statement::If(mut branches, else_stmt) = else_if_block {
            branches.insert(0, Branch::new(cond, if_block));
            return Statement::If(branches, else_stmt);
        } else {
            panic!("grammar error: if statement");
        }
    }

    pub fn insert_to_branch_expr(
        cond: Expr,
        if_block: Vec<Statement>,
        else_if_block: Expr,
    ) -> Expr {
        if let Expr::If(mut branches, else_stmt) = else_if_block {
            branches.insert(0, Branch::new(cond, if_block));
            return Expr::If(branches, else_stmt);
        } else {
            panic!("grammar error: if statement");
        }
    }
}

pub fn append<T>(mut accum: Vec<T>, item: T) -> Vec<T> {
    accum.push(item);
    accum
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
