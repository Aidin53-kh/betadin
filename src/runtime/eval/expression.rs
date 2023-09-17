use std::collections::{BTreeMap, HashMap};

use crate::ast::{BinaryOpKind, Expression, UnaryOpKind};
use crate::runtime::prototypes::Prototypes;
use crate::runtime::value::{KeyValue, Type, Value};
use crate::runtime::{DeclType, ScopeStack};

use super::statement::{eval_statements, Escape};

pub fn eval_expression(
    scopes: &mut ScopeStack,
    expression: Expression,
    prototypes: Prototypes,
) -> Result<Value, String> {
    match expression {
        Expression::Null => Ok(Value::Null),
        Expression::Int(n) => Ok(Value::Int(n)),
        Expression::Float(n) => Ok(Value::Float(n)),
        Expression::String(s) => Ok(Value::String(s)),
        Expression::Bool(b) => Ok(Value::Bool(b)),
        Expression::List(list) => {
            let mut values: Vec<Value> = Vec::new();

            for expr in list {
                let value = eval_expression(scopes, expr, prototypes.clone())?;

                values.push(value);
            }

            Ok(Value::List(values))
        }
        Expression::Call(expr, args) => {
            let value = eval_expression(scopes, *expr.clone(), prototypes.clone())?;

            match value {
                Value::BuiltInFn(f) => {
                    let mut values = vec![];

                    for arg in args {
                        let val = eval_expression(scopes, arg, prototypes.clone())?;
                        values.push(val);
                    }

                    let value = f(values)?;
                    return Ok(value);
                }
                Value::Func(params, block) => {
                    if params.len() != args.len() {
                        return Err(format!(
                            "expected {} arguments but found {}",
                            params.len(),
                            args.len()
                        ));
                    }

                    let mut inner_scope = scopes.new_from_push(HashMap::new());
                    for (i, param) in params.iter().enumerate() {
                        match args.get(i) {
                            Some(expr) => {
                                let value = eval_expression(
                                    &mut inner_scope,
                                    expr.clone(),
                                    prototypes.clone(),
                                )?;

                                inner_scope.declare(param.to_string(), value, DeclType::Mutable)?;
                            }
                            None => {
                                return Err(format!(
                                    "expected {} arguments but found {}",
                                    params.len(),
                                    args.len()
                                ))
                            }
                        }
                    }

                    let ret = eval_statements(&mut inner_scope, block, prototypes)?;
                    match ret {
                        Escape::None => Ok(Value::Null),
                        Escape::Return(value) => Ok(value),
                        Escape::Break => Err(format!("break outside of loop (2)")),
                        Escape::Continue => Err(format!("continue out side of loop (2)")),
                    }
                }
                Value::BuiltInMethod(f, this) => {
                    let mut values = vec![];

                    for arg in args {
                        let val = eval_expression(scopes, arg, prototypes.clone())?;
                        values.push(val);
                    }
                    if let Some(this) = this {
                        let res = f(values, *this)?;
                        return Ok(res);
                    } else {
                        return Err("dev error".to_string());
                    }
                }
                _ => {
                    return Err(format!(
                        "value of type '{:?}' is not callable (5)",
                        Type::from(&value)
                    ));
                }
            }
        }
        Expression::Identifier(name) => match scopes.get(&name) {
            Some(v) => Ok(v),
            None => Err(format!("{} is not defied (8)", name)),
        },
        Expression::MethodCall(object, calle) => {
            let obj_value = eval_expression(scopes, *object.clone(), prototypes.clone())?;

            match *calle.clone() {
                Expression::Identifier(name) => match prototypes.get(&Type::from(&obj_value)) {
                    Some(proto) => match proto.get(&name) {
                        Some(value) => {
                            if let Value::BuiltInMethod(f, _) = value {
                                return Ok(Value::BuiltInMethod(*f, Some(Box::new(obj_value))));
                            }
                            return Ok(value.to_owned());
                        }
                        None => {
                            if let Value::Object(props) = &obj_value {
                                let prop = props.into_iter().find(|kv| kv.key == name);

                                if let Some(kv) = prop {
                                    return Ok(kv.value.clone());
                                }
                            }
                            return Err(format!(
                                "'{}' dose not exist in '{:?}' prototype (6)",
                                name,
                                Type::from(&obj_value)
                            ));
                        }
                    },
                    None => {
                        return Err(format!(
                            "the prototype for type {:?} is not implemented (8)",
                            Type::from(&obj_value)
                        ));
                    }
                },
                Expression::Call(expr, args) => match *expr.clone() {
                    Expression::Identifier(name) => match prototypes.get(&Type::from(&obj_value)) {
                        Some(proto) => match proto.get(&name) {
                            Some(value) => match value {
                                Value::BuiltInMethod(f, _) => {
                                    let mut values = vec![];

                                    for arg in args {
                                        let val = eval_expression(scopes, arg, prototypes.clone())?;
                                        values.push(val);
                                    }

                                    let res = f(values, obj_value.to_owned())?;
                                    return Ok(res);
                                }
                                _ => todo!(),
                            },
                            None => {
                                if let Value::Object(props) = &obj_value {
                                    let prop = props.into_iter().find(|kv| kv.key == name);
                                    if let Some(kv) = prop {
                                        return Ok(kv.value.to_owned());
                                    }
                                }
                                return Err(format!(
                                    "'{}' dose not exist in '{:?}' prototype (3)",
                                    name,
                                    Type::from(&obj_value)
                                ));
                            }
                        },
                        None => {
                            return Err(format!(
                                "the prototype for type {:?} is not implemented",
                                Type::from(&obj_value)
                            ))
                        }
                    },
                    _ => {
                        return Err(format!(
                            "value of type {:?} not callable (2)",
                            Type::from(&obj_value)
                        ));
                    }
                },
                Expression::Index(_, _) => todo!(),
                _ => {
                    return Err(format!(
                        "value of type {:?} not callable (1)",
                        Type::from(&obj_value)
                    ));
                }
            }
        }
        Expression::Index(expr, loc) => {
            let expr_value = eval_expression(scopes, *expr, prototypes.clone())?;

            match &expr_value {
                Value::String(s) => {
                    let loc_value = eval_expression(scopes, *loc, prototypes.clone())?;

                    match loc_value {
                        Value::Int(index) => {
                            if let Some(res) = s.chars().nth(index as usize) {
                                return Ok(Value::String(res.to_string()));
                            } else {
                                return Err(format!("index out of bounds"));
                            }
                        }
                        _ => {
                            return Err(format!(
                                "the type {:?} cannot be indexed by {:?}",
                                Type::from(&expr_value),
                                Type::from(&loc_value)
                            ))
                        }
                    }
                }
                Value::List(l) => {
                    let loc_value = eval_expression(scopes, *loc, prototypes.clone())?;

                    match loc_value {
                        Value::Int(index) => {
                            if let Some(res) = l.get(index as usize) {
                                return Ok(Value::from(res));
                            } else {
                                return Err(format!("index out of bounds"));
                            }
                        }
                        _ => {
                            return Err(format!(
                                "the type {:?} cannot be indexed by {:?}",
                                Type::from(&expr_value),
                                Type::from(&loc_value)
                            ))
                        }
                    }
                }
                _ => {
                    return Err(format!(
                        "cannot index into a value of type {:?}",
                        Type::from(&expr_value)
                    ));
                }
            }
        }
        Expression::BinaryOp(lhs_expr, op, rhs_expr) => {
            let lhs = eval_expression(scopes, *lhs_expr, prototypes.clone())?;
            let rhs = eval_expression(scopes, *rhs_expr, prototypes.clone())?;

            let res = match op {
                BinaryOpKind::Add => &lhs + &rhs,
                BinaryOpKind::Sub => &lhs - &rhs,
                BinaryOpKind::Mul => &lhs * &rhs,
                BinaryOpKind::Div => &lhs / &rhs,
                BinaryOpKind::EQ => Ok(Value::Bool(lhs == rhs)),
                BinaryOpKind::NE => Ok(Value::Bool(lhs != rhs)),
                BinaryOpKind::GT => Ok(Value::Bool(lhs > rhs)),
                BinaryOpKind::LT => Ok(Value::Bool(lhs < rhs)),
                BinaryOpKind::GTE => Ok(Value::Bool(lhs >= rhs)),
                BinaryOpKind::LTE => Ok(Value::Bool(lhs <= rhs)),
                BinaryOpKind::And => {
                    if let Value::Bool(v1) = lhs {
                        if let Value::Bool(v2) = rhs {
                            return Ok(Value::Bool(v1 && v2));
                        } else {
                            return Err(format!("expected bool found {:?}", Type::from(&rhs)));
                        }
                    } else {
                        return Err(format!("expected bool found {:?}", Type::from(&lhs)));
                    }
                }
                BinaryOpKind::Or => {
                    if let Value::Bool(v1) = lhs {
                        if let Value::Bool(v2) = rhs {
                            return Ok(Value::Bool(v1 || v2));
                        } else {
                            return Err(format!("expected bool found {:?}", Type::from(&rhs)));
                        }
                    } else {
                        return Err(format!("expected bool found {:?}", Type::from(&lhs)));
                    }
                }
            };

            res
        }
        Expression::UnaryOp(op, expr) => {
            let value = eval_expression(scopes, *expr, prototypes)?;

            match op {
                UnaryOpKind::Not => !value,
                UnaryOpKind::Typeof => Ok(Value::String(Type::from(&value).to_string())),
            }
        }
        Expression::Object(props) => {
            let mut values: Vec<KeyValue> = Vec::new();

            for prop in &props {
                let value = eval_expression(scopes, prop.value.clone(), prototypes.clone())?;

                values.push(KeyValue {
                    key: prop.key.to_string(),
                    value,
                });
            }

            Ok(Value::Object(values))
        }
        Expression::Fn(args, block) => Ok(Value::Func(args, block)),
        Expression::ModuleCall(paths, expr) => {
            let module = get_module(scopes, paths)?;

            let mut inner_scopes = scopes.new_from_push(HashMap::new());

            for (key, value) in module {
                inner_scopes.declare(key, value, DeclType::Immutable)?;
            }

            let value = eval_expression(&mut inner_scopes, *expr, prototypes.clone())?;
            Ok(value)
        }
    }
}

pub fn get_module(
    scopes: &mut ScopeStack,
    paths: Vec<String>,
) -> Result<BTreeMap<String, Value>, String> {
    let mut exports: BTreeMap<String, Value> = BTreeMap::new();

    for path in &paths {
        match exports.get(path) {
            Some(value) => match value {
                Value::Module(items) => {
                    exports = items.clone();
                }
                _ => return Err(format!("module {} not found", path)),
            },
            None => match scopes.get(path) {
                Some(value) => match value {
                    Value::Module(items) => {
                        exports = items;
                    }
                    _ => return Err(format!("module {} not found", path)),
                },
                None => return Err(format!("module {} not found", path)),
            },
        }
    }

    Ok(exports)
}
