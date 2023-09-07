use std::collections::HashMap;

use crate::ast::Program;
use crate::runtime::std::Prototypes;
use crate::runtime::value::Value;
use crate::Export;

use super::statement::eval_statement;

pub fn eval_program(
    env: &mut HashMap<String, Value>,
    program: Program,
    modules: Vec<Export>,
    prototypes: Prototypes,
) -> Result<(), String> {
    for statement in program.statements {
        eval_statement(env, statement, modules.clone(), prototypes.clone())?;
    }

    Ok(())
}
