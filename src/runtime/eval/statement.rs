use std::collections::HashMap;

use crate::ast::statement::Statement;

use super::program::Value;

pub fn eval_statement(
    env: &mut HashMap<String, Value>,
    statement: Statement,
) -> Result<(), String> {
    match statement {
        Statement::ExpressionStatement(_) => {
            return Ok(());
        } // _ => Err(format!("unhandeled statement: {:?}", statement)),
    };
}
