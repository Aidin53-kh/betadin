use std::collections::HashMap;

use super::expression::eval_expression;
use crate::ast::statement::Statement;
use crate::{Export, Value};

pub fn eval_statement(
    env: &mut HashMap<String, Value>,
    statement: Statement,
    modules: Vec<Export>,
) -> Result<(), String> {
    match statement {
        Statement::ExpressionStatement(expr) => {
            eval_expression(env, expr)?;
            return Ok(());
        }
        Statement::LetStatement(name, rhs) => {
            let value = eval_expression(env, rhs)?;
            env.insert(name, value);
            return Ok(());
        }
        Statement::ImportStatement(args) => {
            apply_imports(env, modules, args)?;
            return Ok(());
        }
        Statement::AssignmentStatement(name, rhs) => {
            let value = eval_expression(env, rhs)?;
            env.insert(name, value);
            return Ok(());
        } // _ => Err(format!("unhandeled statement: {:?}", statement)),
    };
}

pub fn apply_imports(
    env: &mut HashMap<String, Value>,
    modules: Vec<Export>,
    args: Vec<String>,
) -> Result<(), String> {
    let mut last = modules;

    for (i, arg) in args.iter().enumerate() {
        if let Some(m) = last.to_vec().into_iter().find(|e| match e {
            Export::Module { name, exports: _ } => {
                return name == arg;
            }
            Export::Item { name, value: _ } => {
                return name == arg;
            }
        }) {
            match m {
                Export::Module { name: _, exports } => {
                    if let None = args.get(i + 1) {
                        for export in exports.iter() {
                            if let Export::Item { name, value } = export {
                                env.insert(name.to_string(), value.clone());
                            }
                        }
                    } else {
                        last = exports.to_owned();
                    }
                }
                Export::Item { name: _, value } => {
                    if let Some(_) = args.get(i + 1) {
                        return Err(format!("{} is not a module", arg));
                    } else {
                        env.insert(arg.to_string(), value.to_owned());
                    }
                }
            }
        } else {
            return Err(format!("module or item {} not found", arg));
        }
    }

    Ok(())
}
