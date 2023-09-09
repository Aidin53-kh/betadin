use std::collections::HashMap;

use crate::ast::{BinaryOpKind, Expression, UnaryOpKind};
use crate::runtime::std::Prototypes;
use crate::runtime::value::{Type, Value};
use crate::Export;

use super::statement::eval_statement;

pub fn eval_expression(
    env: &mut HashMap<String, Value>,
    expression: Expression,
    modules: Vec<Export>,
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
                let value = eval_expression(env, expr, modules.clone(), prototypes.clone())?;

                values.push(value);
            }

            Ok(Value::List(values))
        }
        Expression::Call(expr, args) => {
            let value = eval_expression(env, *expr, modules.clone(), prototypes.clone())?;

            match value {
                Value::BuiltInFn(f) => {
                    let mut values = vec![];

                    for arg in args {
                        let val = eval_expression(env, arg, modules.clone(), prototypes.clone())?;
                        values.push(val);
                    }

                    let value = f(values)?;
                    return Ok(value);
                }
                _ => {
                    return Err(format!(
                        "value of type '{:?}' is not callable (5)",
                        Type::from(&value)
                    ));
                }
            }
        }
        Expression::Identifier(name) => {
            let data = env.get(&name);

            if let Some(data) = data {
                return Ok(data.clone());
            } else {
                return Err(format!("{} is not defied", name));
            }
        }
        Expression::MethodCall(object, calle) => {
            let obj_value =
                eval_expression(env, *object.clone(), modules.clone(), prototypes.clone())?;

            match *calle {
                Expression::Identifier(name) => match prototypes.get(&Type::from(&obj_value)) {
                    Some(proto) => match proto.get(&name) {
                        Some(value) => return Ok(value.to_owned()),
                        None => {
                            return Err(format!(
                                "'{}' dose not exist in '{:?}' prototype",
                                name,
                                Type::from(&obj_value)
                            ))
                        }
                    },
                    None => {
                        return Err(format!(
                            "the prototype for type {:?} is not implemented",
                            Type::from(&obj_value)
                        ))
                    }
                },
                Expression::Call(expr, args) => match *expr.clone() {
                    Expression::Identifier(name) => match prototypes.get(&Type::from(&obj_value)) {
                        Some(proto) => match proto.get(&name) {
                            Some(value) => match value {
                                Value::BuiltInMethod(f) => {
                                    let mut values = vec![];

                                    for arg in args {
                                        let val = eval_expression(
                                            env,
                                            arg,
                                            modules.clone(),
                                            prototypes.clone(),
                                        )?;
                                        values.push(val);
                                    }

                                    let res = f(values, obj_value.to_owned())?;
                                    return Ok(res);
                                }
                                _ => todo!(),
                            },
                            None => {
                                return Err(format!(
                                    "'{}' dose not exist in '{:?}' prototype",
                                    name,
                                    Type::from(&obj_value)
                                ))
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
            let expr_value = eval_expression(env, *expr, modules.clone(), prototypes.clone())?;

            match &expr_value {
                Value::String(s) => {
                    let loc_value =
                        eval_expression(env, *loc, modules.clone(), prototypes.clone())?;

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
                    let loc_value = eval_expression(env, *loc, modules, prototypes.clone())?;

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
            // println!("loc: {:?}", loc);
        }
        Expression::BinaryOp(lhs_expr, op, rhs_expr) => {
            let lhs = eval_expression(env, *lhs_expr, modules.clone(), prototypes.clone())?;
            let rhs = eval_expression(env, *rhs_expr, modules.clone(), prototypes.clone())?;

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
            let value = eval_expression(env, *expr, modules, prototypes)?;

            match op {
                UnaryOpKind::Not => !value,
            }
        }
        Expression::If(branchs, else_block) => {
            for branch in branchs {
                let value =
                    eval_expression(env, branch.condition, modules.clone(), prototypes.clone())?;

                match value {
                    Value::Bool(b) => {
                        if b {
                            for statement in branch.statements {
                                eval_statement(
                                    env,
                                    statement,
                                    modules.clone(),
                                    prototypes.clone(),
                                )?;
                            }
                            return Ok(Value::Null);
                        }
                    }
                    _ => return Err(format!("condition most be a boolean")),
                }
            }

            if let Some(stmts) = else_block {
                for statement in stmts {
                    eval_statement(env, statement, modules.clone(), prototypes.clone())?;
                }
            }

            Ok(Value::Null)
        }
    }
}
