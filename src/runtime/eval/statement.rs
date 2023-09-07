use std::collections::HashMap;

use super::expression::eval_expression;
use crate::ast::Statement;
use crate::runtime::std::Prototypes;
use crate::runtime::value::Value;
use crate::Export;

pub fn eval_statement(
    env: &mut HashMap<String, Value>,
    statement: Statement,
    modules: Vec<Export>,
    prototypes: Prototypes,
) -> Result<(), String> {
    match statement {
        Statement::ExpressionStatement(expr) => {
            eval_expression(env, expr, prototypes)?;
            return Ok(());
        }
        Statement::LetStatement(name, rhs) => {
            if let Some(_) = env.get(&name) {
                return Err(format!("Duplicate variable {}", name));
            } else {
                let value = eval_expression(env, rhs, prototypes)?;
                env.insert(name, value);
                return Ok(());
            }
        }
        Statement::ImportStatement(args) => {
            apply_imports(env, modules, args)?;
            return Ok(());
        }
        Statement::AssignmentStatement(name, rhs) => {
            if let Some(_) = env.get(&name) {
                let value = eval_expression(env, rhs, prototypes)?;
                env.insert(name, value);
                return Ok(());
            } else {
                return Err(format!("variable {} is not defined", name));
            }
        }
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
