use std::collections::HashMap;

use crate::{ast::statement::Statement, Value};

use super::expression::eval_expression;

pub fn eval_statement(
    env: &mut HashMap<String, Value>,
    statement: Statement,
) -> Result<(), String> {
    match statement {
        Statement::ExpressionStatement(_) => {
            return Ok(());
        }
        Statement::AssignmentStatement(name, rhs) => {
            let value = eval_expression(env, rhs)?;
            env.insert(name, value);
            return Ok(());
        }
        // _ => Err(format!("unhandeled statement: {:?}", statement)),
    };
}
