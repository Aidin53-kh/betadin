use std::collections::HashMap;

use super::expression::eval_expression;
use crate::ast::Statement;
use crate::runtime::std::Prototypes;
use crate::runtime::value::Value;
use crate::Export;

#[derive(Debug, Clone)]
pub enum Escape {
    None,
    Return(Value),
}

pub fn eval_statement(
    env: &mut HashMap<String, Value>,
    statement: Statement,
    modules: Vec<Export>,
    prototypes: Prototypes,
) -> Result<Escape, String> {
    match statement {
        Statement::ExpressionStatement(expr) => {
            eval_expression(env, expr, modules, prototypes)?;
        }
        Statement::LetStatement(name, rhs) => {
            if let Some(_) = env.get(&name) {
                return Err(format!("Duplicate variable {}", name));
            } else {
                let value = eval_expression(env, rhs, modules, prototypes)?;
                env.insert(name, value);
            }
        }
        Statement::ImportStatement(args) => {
            apply_imports(env, modules, args)?;
        }
        Statement::AssignmentStatement(name, rhs) => {
            if let Some(_) = env.get(&name) {
                let value = eval_expression(env, rhs, modules, prototypes)?;
                env.insert(name, value);
            } else {
                return Err(format!("variable {} is not defined", name));
            }
        }
        Statement::IfStatement(branchs, else_block) => {
            for branch in branchs {
                let value =
                    eval_expression(env, branch.condition, modules.clone(), prototypes.clone())?;

                match value {
                    Value::Bool(b) => {
                        if b {
                            let e = eval_statements(
                                env,
                                branch.statements,
                                modules.clone(),
                                prototypes.clone(),
                            )?;
                            return Ok(e);
                        }
                    }
                    _ => return Err(format!("condition most be a boolean")),
                }
            }

            if let Some(stmts) = else_block {
                let e = eval_statements(env, stmts, modules.clone(), prototypes.clone())?;
                return Ok(e);
            }
        }
        Statement::ReturnStatement(expr) => {
            let value = eval_expression(env, *expr, modules.clone(), prototypes.clone())?;
            return Ok(Escape::Return(value));
        }
    };

    Ok(Escape::None)
}

pub fn eval_statements(
    env: &mut HashMap<String, Value>,
    statements: Vec<Statement>,
    modules: Vec<Export>,
    prototypes: Prototypes,
) -> Result<Escape, String> {
    for statement in &statements {
        let e = eval_statement(env, statement.clone(), modules.clone(), prototypes.clone())?;
        match &e {
            Escape::None => {}
            Escape::Return(_) => {
                return Ok(e);
            }
        }
    }

    Ok(Escape::None)
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
