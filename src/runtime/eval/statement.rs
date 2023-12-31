use std::collections::{BTreeMap, HashMap};

use crate::ast::Statement;
use crate::runtime::value::{BuiltinType, Value};
use crate::runtime::{DeclType, ScopeStack, Simple, Type};

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
    prototypes: &HashMap<String, HashMap<String, Value>>,
) -> Result<Escape, String> {
    match statement {
        Statement::Expression(expr) => {
            eval_expression(scopes, expr, prototypes)?;
        }
        Statement::Let(name, datatype, rhs) => {
            let value = eval_expression(scopes, rhs, prototypes)?;

            if let Some(datatype) = datatype {
                scopes.declare_variable(name, datatype, &value, DeclType::Mutable)?;
            } else {
                scopes.declare_variable(name, &Type::from(&value), &value, DeclType::Mutable)?;
            }
        }
        Statement::Const(name, datatype, rhs) => {
            let value = eval_expression(scopes, rhs, prototypes)?;

            if let Some(datatype) = datatype {
                scopes.declare_variable(name, datatype, &value, DeclType::Immutable)?;
            } else {
                scopes.declare_variable(name, &Type::from(&value), &value, DeclType::Immutable)?;
            }
        }
        Statement::Import(args, items) => {
            let module = get_module(scopes, args)?;

            match items {
                Some(list) => {
                    for (key, value) in module {
                        if list.contains(&key) {
                            scopes.declare(
                                &key,
                                value.clone(),
                                &Some(Type::from(&value)),
                                DeclType::Immutable,
                            )?;
                        }
                    }
                }
                None => {
                    if let Some(m) = args.last() {
                        scopes.declare(
                            m,
                            Value::Module(module),
                            &Some(Type::Alias("todo".to_string())),
                            DeclType::Immutable,
                        )?;
                    }
                }
            }
        }
        Statement::Assignment(name, rhs) => {
            let value = eval_expression(scopes, rhs, prototypes)?;
            scopes.assgin(name.to_string(), value)?;
        }
        Statement::If(branchs, else_block) => {
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
        Statement::Return(expr) => {
            let value = eval_expression(scopes, expr, prototypes)?;
            return Ok(Escape::Return(value));
        }
        Statement::Fn(name, args, ret_type, block) => {
            if let Some(ret_type) = ret_type {
                scopes.declare_fn_statement(name, args, ret_type, block)?;
            } else {
                scopes.declare_fn_statement(
                    name,
                    args,
                    &Type::Builtin(BuiltinType::Null),
                    block,
                )?;
            }
        }
        Statement::For(lhs, iter, block) => {
            let iter_val = eval_expression(scopes, iter, prototypes)?;

            match &iter_val {
                Value::List(values) | Value::Tuple(values) => {
                    for (i, value) in values.iter().enumerate() {
                        let mut inner_scopes = scopes.new_from_push(HashMap::new());

                        inner_scopes.declare(
                            lhs,
                            value.clone(),
                            &Some(Type::from(value)),
                            DeclType::Mutable,
                        )?;

                        inner_scopes.declare(
                            &"index".to_string(),
                            Value::Int(i as i32),
                            &Some(Type::Builtin(BuiltinType::Int)),
                            DeclType::Immutable,
                        )?;

                        let ret = eval_statements(&mut inner_scopes, block, prototypes)?;

                        match ret {
                            Escape::None => {}
                            Escape::Continue => {}
                            Escape::Return(v) => return Ok(Escape::Return(v)),
                            Escape::Break => return Ok(Escape::None),
                        }
                    }
                }
                _ => {
                    return Err(format!(
                        "iterator most be a list, found {}",
                        Type::simple(&iter_val)
                    ))
                }
            }
        }
        Statement::Break => return Ok(Escape::Break),
        Statement::Continue => return Ok(Escape::Continue),
        Statement::While(cond, block) => loop {
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
        Statement::Module(name, statements) => {
            let module = eval_module(scopes, prototypes, name, statements)?;

            scopes.declare(
                name,
                Value::Module(module),
                &Some(Type::Alias("todo".to_string())),
                DeclType::Immutable,
            )?;
        }
        Statement::Type(name, datatype) => {
            scopes.declare_type_alias(name, datatype)?;
        }
    };

    Ok(Escape::None)
}

pub fn eval_statements(
    scopes: &mut ScopeStack,
    statements: &Vec<Statement>,
    prototypes: &HashMap<String, HashMap<String, Value>>,
) -> Result<Escape, String> {
    let mut inner_scopes = scopes.new_from_push(HashMap::new());

    for statement in statements {
        let e = eval_statement(&mut inner_scopes, statement, prototypes)?;

        if let Statement::Fn(..) = statement {
            continue;
        }

        if let Escape::None = e {
            continue;
        }

        return Ok(e);
    }

    Ok(Escape::None)
}

pub fn eval_statements_and_push_scope(
    scopes: &mut ScopeStack,
    statements: &Vec<Statement>,
    prototypes: &HashMap<String, HashMap<String, Value>>,
) -> Result<Escape, String> {
    scopes.push(HashMap::new());

    for statement in statements {
        let e = eval_statement(scopes, statement, prototypes)?;

        if let Statement::Fn(..) = statement {
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
    prototypes: &HashMap<String, HashMap<String, Value>>,
    name: &String,
    statements: &Vec<Statement>,
) -> Result<BTreeMap<String, Value>, String> {
    let mut exports: BTreeMap<String, Value> = BTreeMap::new();

    let mut inner_scope = scopes.new_from_push(HashMap::new());
    for statement in statements {
        match statement {
            Statement::Const(name, datatype, expr) => {
                let value = eval_expression(&mut inner_scope, expr, prototypes)?;

                // type checking
                if let Some(datatype) = datatype {
                    if &Type::from(&value) != datatype {
                        return Err(format!(
                            "expected {} found {}",
                            datatype,
                            Type::from(&value),
                        ));
                    }
                }
                exports.insert(name.to_string(), value);
            }
            Statement::Let(name, datatype, expr) => {
                let value = eval_expression(&mut inner_scope, expr, prototypes)?;

                // type checking
                if let Some(datatype) = datatype {
                    if &Type::from(&value) != datatype {
                        return Err(format!(
                            "expected {} found {}",
                            datatype,
                            Type::from(&value),
                        ));
                    }
                }
                exports.insert(name.to_string(), value);
            }
            Statement::Fn(name, args, ret_type, block) => {
                exports.insert(
                    name.to_string(),
                    Value::Func(args.to_vec(), ret_type.clone(), block.to_vec()),
                );
            }
            Statement::Module(name2, statements2) => {
                let exports2 = eval_module(&mut inner_scope, prototypes, name2, statements2)?;
                exports.insert(name2.to_string(), Value::Module(exports2));
            }
            other => return Err(format!("'{:?}' is not supported in modules", other)),
        }
    }

    inner_scope.declare(
        name,
        Value::Module(exports.clone()),
        &Some(Type::Alias("module".to_string())),
        DeclType::Immutable,
    )?;
    Ok(exports)
}
