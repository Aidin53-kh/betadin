use std::collections::{BTreeMap, HashMap};

use crate::ast::Statement;
use crate::runtime::value::{Type, Value};
use crate::runtime::{DeclType, ScopeStack};

use super::expression::{eval_expression, get_module};

#[derive(Debug, Clone)]
pub enum Escape {
    None,
    Return(Value),
    Break,
    Continue,
}

pub fn eval_statement(
    scopes: &mut ScopeStack,
    statement: &Statement,
    prototypes: &HashMap<Type, HashMap<String, Value>>,
) -> Result<Escape, String> {
    match statement {
        Statement::ExpressionStatement(expr) => {
            eval_expression(scopes, expr, prototypes)?;
        }
        Statement::LetStatement(name, rhs) => {
            let value = eval_expression(scopes, rhs, prototypes)?;
            scopes.declare(name, value, DeclType::Mutable)?;
        }
        Statement::ConstStatement(name, rhs) => {
            let value = eval_expression(scopes, rhs, prototypes)?;
            scopes.declare(name, value, DeclType::Immutable)?;
        }
        Statement::ImportStatement(args, items) => {
            let module = get_module(scopes, args)?;

            match items {
                Some(list) => {
                    for (key, value) in module {
                        if list.contains(&key) {
                            scopes.declare(&key, value, DeclType::Immutable)?;
                        }
                    }
                }
                None => {
                    if let Some(m) = args.last() {
                        scopes.declare(m, Value::Module(module), DeclType::Immutable)?;
                    }
                }
            }
            // apply_imports(scopes, modules, args, items)?;
        }
        Statement::AssignmentStatement(name, rhs) => {
            let value = eval_expression(scopes, rhs, prototypes)?;
            scopes.assgin(name.to_string(), value)?;
        }
        Statement::IfStatement(branchs, else_block) => {
            for branch in branchs {
                let value = eval_expression(scopes, &branch.condition, prototypes)?;

                match value {
                    Value::Bool(b) => {
                        if b {
                            let ret = eval_statements(scopes, &branch.statements, prototypes)?;
                            return Ok(ret);
                        }
                    }
                    _ => return Err(format!("condition most be a boolean")),
                }
            }

            if let Some(stmts) = else_block {
                let e = eval_statements(scopes, stmts, prototypes)?;
                return Ok(e);
            }
        }
        Statement::ReturnStatement(expr) => {
            let value = eval_expression(scopes, expr, prototypes)?;
            return Ok(Escape::Return(value));
        }
        Statement::FnStatement(name, args, block) => {
            scopes.declare(
                name,
                Value::Func(args.to_vec(), block.to_vec()),
                DeclType::Immutable,
            )?;
        }
        Statement::ForStatement(lhs, iter, block) => {
            let iter_val = eval_expression(scopes, iter, prototypes)?;

            match iter_val {
                Value::List(values) | Value::Tuple(values) => {
                    for value in values {
                        let mut inner_scopes = scopes.new_from_push(HashMap::new());

                        inner_scopes.declare(lhs, value, DeclType::Mutable)?;
                        let ret = eval_statements(&mut inner_scopes, block, prototypes)?;

                        match ret {
                            Escape::None => {}
                            Escape::Continue => {}
                            Escape::Return(v) => return Ok(Escape::Return(v)),
                            Escape::Break => return Ok(Escape::None),
                        }
                    }
                }
                _ => return Err(format!("iterator most be a list")),
            }
        }
        Statement::BreakStatement => return Ok(Escape::Break),
        Statement::ContinueStatement => return Ok(Escape::Continue),
        Statement::WhileStatement(cond, block) => loop {
            let value = eval_expression(scopes, cond, prototypes)?;

            match value {
                Value::Bool(b) => {
                    if !b {
                        break;
                    }

                    let ret = eval_statements(scopes, block, prototypes)?;

                    match ret {
                        Escape::None => {}
                        Escape::Continue => {}
                        Escape::Return(v) => return Ok(Escape::Return(v)),
                        Escape::Break => return Ok(Escape::None),
                    }
                }
                _ => return Err(format!("condition most be a boolean")),
            }
        },
        Statement::ModuleStatement(name, statements) => {
            let module = eval_module(scopes, prototypes, name, statements)?;

            scopes.declare(name, Value::Module(module), DeclType::Immutable)?;
        }
    };

    Ok(Escape::None)
}

pub fn eval_statements(
    scopes: &mut ScopeStack,
    statements: &Vec<Statement>,
    prototypes: &HashMap<Type, HashMap<String, Value>>,
) -> Result<Escape, String> {
    let mut inner_scopes = scopes.new_from_push(HashMap::new());

    for statement in statements {
        let e = eval_statement(&mut inner_scopes, statement, prototypes)?;

        if let Statement::FnStatement(_, _, _) = statement {
            continue;
        }

        if let Escape::None = e {
            continue;
        }

        return Ok(e);
    }

    Ok(Escape::None)
}

pub fn eval_module(
    scopes: &mut ScopeStack,
    prototypes: &HashMap<Type, HashMap<String, Value>>,
    name: &String,
    statements: &Vec<Statement>,
) -> Result<BTreeMap<String, Value>, String> {
    let mut exports: BTreeMap<String, Value> = BTreeMap::new();

    let mut inner_scope = scopes.new_from_push(HashMap::new());
    for statement in statements {
        match statement {
            Statement::ConstStatement(name, expr) => {
                let value = eval_expression(&mut inner_scope, expr, prototypes)?;

                exports.insert(name.to_string(), value);
            }
            Statement::LetStatement(name, expr) => {
                let value = eval_expression(&mut inner_scope, expr, prototypes)?;

                exports.insert(name.to_string(), value);
            }
            Statement::FnStatement(name, args, block) => {
                exports.insert(name.to_string(), Value::Func(args.to_vec(), block.to_vec()));
            }
            Statement::ModuleStatement(name2, statements2) => {
                let exports2 = eval_module(&mut inner_scope, prototypes, name2, statements2)?;
                exports.insert(name.to_string(), Value::Module(exports2));
            }
            other => return Err(format!("'{:?}' is not supported in modules", other)),
        }
    }

    inner_scope.declare(name, Value::Module(exports.clone()), DeclType::Immutable)?;
    Ok(exports)
}
