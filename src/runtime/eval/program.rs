use std::collections::HashMap;

use crate::ast::{program::Program, statement::Statement};

use super::statement::eval_statement;

pub enum Value {
    Int(i32),
    String(String),
}

pub fn eval_program(env: &mut HashMap<String, Value>, program: Program) -> Result<(), String> {
    for statement in program.statements {
        eval_statement(env, statement)?;
    }

    Ok(())
}
