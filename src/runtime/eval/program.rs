use crate::ast::Program;
use crate::runtime::std::Prototypes;
use crate::runtime::ScopeStack;
use crate::Export;

use super::statement::{eval_statements, Escape};

pub fn eval_program(
    scopes: &mut ScopeStack,
    program: Program,
    modules: Vec<Export>,
    prototypes: Prototypes,
) -> Result<Escape, String> {
    let e = eval_statements(scopes, program.statements, modules, prototypes)?;

    if let Escape::Return(_) = e {
        return Err(format!("return outside of a function"));
    }

    Ok(e)
}
