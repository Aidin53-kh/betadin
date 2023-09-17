use crate::ast::Program;
use crate::runtime::prototypes::Prototypes;
use crate::runtime::ScopeStack;

use super::statement::{eval_statements, Escape};

pub fn eval_program(
    scopes: &mut ScopeStack,
    program: Program,
    prototypes: Prototypes,
) -> Result<Escape, String> {
    let e = eval_statements(scopes, program.statements, prototypes)?;

    if let Escape::Return(_) = e {
        return Err(format!("return outside of function"));
    }

    if let Escape::Break = e {
        return Err(format!("break outside of loop"));
    }

    if let Escape::Continue = e {
        return Err(format!("continue outside of loop"));
    }

    Ok(e)
}
