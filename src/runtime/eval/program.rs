use std::collections::HashMap;

use crate::runtime::value::Value;
use crate::runtime::ScopeStack;
use crate::{ast::Program, runtime::value::Type};

use super::statement::{eval_statements, eval_statements_and_push_scope, Escape};

pub fn eval_program(
    scopes: &mut ScopeStack,
    program: Program,
    prototypes: &HashMap<Type, HashMap<String, Value>>,
) -> Result<Escape, String> {
    let e = eval_statements(scopes, &program.statements, prototypes)?;

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

pub fn eval_program_and_push_scope(
    scopes: &mut ScopeStack,
    program: Program,
    prototypes: &HashMap<Type, HashMap<String, Value>>,
) -> Result<Escape, String> {
    let e = eval_statements_and_push_scope(scopes, &program.statements, prototypes)?;

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
