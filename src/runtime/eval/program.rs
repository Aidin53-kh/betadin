use std::collections::HashMap;

use crate::ast::Program;
use crate::runtime::std::Prototypes;
use crate::runtime::value::Value;
use crate::Export;

use super::statement::{eval_statements, Escape};

pub fn eval_program(
    env: &mut HashMap<String, Value>,
    program: Program,
    modules: Vec<Export>,
    prototypes: Prototypes,
) -> Result<Escape, String> {
    let e = eval_statements(env, program.statements, modules, prototypes)?;

    println!("result of program: {:?}", e);
    Ok(e)
}
