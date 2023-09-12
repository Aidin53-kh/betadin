use std::collections::HashMap;

use super::expression::eval_expression;
use crate::ast::Statement;
use crate::runtime::std::Prototypes;
use crate::runtime::value::Value;
use crate::runtime::ScopeStack;
use crate::Export;

#[derive(Debug, Clone)]
pub enum Escape {
    None,
    Return(Value),
}

pub fn eval_statement(
    scopes: &mut ScopeStack,
    statement: Statement,
    modules: Vec<Export>,
    prototypes: Prototypes,
) -> Result<Escape, String> {
    match statement {
        Statement::ExpressionStatement(expr) => {
            eval_expression(scopes, expr, modules, prototypes)?;
        }
        Statement::LetStatement(name, rhs) => {
            let value = eval_expression(scopes, rhs, modules, prototypes)?;
            scopes.declare(name, value)?;
        }
        Statement::ImportStatement(args) => {
            apply_imports(scopes, modules, args)?;
        }
        Statement::AssignmentStatement(name, rhs) => {
            let value = eval_expression(scopes, rhs, modules, prototypes)?;
            scopes.assgin(name, value)?;
        }
        Statement::IfStatement(branchs, else_block) => {
            for branch in branchs {
                let value = eval_expression(
                    scopes,
                    branch.condition,
                    modules.clone(),
                    prototypes.clone(),
                )?;

                match value {
                    Value::Bool(b) => {
                        if b {
                            let ret = eval_statements(
                                scopes,
                                branch.statements,
                                modules.clone(),
                                prototypes.clone(),
                            )?;
                            return Ok(ret);
                        }
                    }
                    _ => return Err(format!("condition most be a boolean")),
                }
            }

            if let Some(stmts) = else_block {
                let e = eval_statements(scopes, stmts, modules.clone(), prototypes.clone())?;
                return Ok(e);
            }
        }
        Statement::ReturnStatement(expr) => {
            let value = eval_expression(scopes, expr, modules.clone(), prototypes.clone())?;
            return Ok(Escape::Return(value));
        }
        Statement::FnStatement(name, args, block) => {
            scopes.declare(name, Value::Func(args, block))?;
        }
        Statement::ForStatement(lhs, iter, block) => {
            let iter_val = eval_expression(scopes, iter, modules.clone(), prototypes.clone())?;

            match iter_val {
                Value::List(values) => {
                    for value in values {
                        let mut inner_scopes = scopes.new_from_push(HashMap::new());

                        inner_scopes.declare(lhs.clone(), value)?;
                        let ret = eval_statements(
                            &mut inner_scopes,
                            block.to_vec(),
                            modules.clone(),
                            prototypes.clone(),
                        )?;

                        match ret {
                            Escape::None => {}
                            Escape::Return(v) => return Ok(Escape::Return(v)),
                        }
                    }
                }
                _ => return Err(format!("iterator most be a list")),
            }
        }
    };

    Ok(Escape::None)
}

pub fn eval_statements(
    scopes: &mut ScopeStack,
    statements: Vec<Statement>,
    modules: Vec<Export>,
    prototypes: Prototypes,
) -> Result<Escape, String> {
    let mut inner_scopes = scopes.new_from_push(HashMap::new());

    for statement in &statements {
        let e = eval_statement(
            &mut inner_scopes,
            statement.clone(),
            modules.clone(),
            prototypes.clone(),
        )?;

        if let Statement::FnStatement(_, _, _) = statement {
            continue;
        }

        if let Escape::Return(_) = &e {
            return Ok(e);
        }
    }

    Ok(Escape::None)
}

pub fn apply_imports(
    scopes: &mut ScopeStack,
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
                                scopes.declare(name.to_string(), value.clone())?;
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
                        scopes.declare(arg.to_string(), value.to_owned())?;
                    }
                }
            }
        } else {
            return Err(format!("module or item {} not found", arg));
        }
    }

    Ok(())
}
