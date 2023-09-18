use crate::ast::{Branch, Expr, Statement};

pub fn append<T>(mut accum: Vec<T>, item: T) -> Vec<T> {
    accum.push(item);
    accum
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

pub fn insert_to_branch_expr(cond: Expr, if_block: Vec<Statement>, else_if_block: Expr) -> Expr {
    if let Expr::If(mut branches, else_stmt) = else_if_block {
        branches.insert(0, Branch::new(cond, if_block));
        return Expr::If(branches, else_stmt);
    } else {
        panic!("grammar error: if statement");
    }
}
