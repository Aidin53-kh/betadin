use std::collections::HashMap;

use crate::ast::{BinaryOpKind, Expression, UnaryOpKind};
use crate::runtime::std::Prototypes;
use crate::runtime::value::{Type, Value};

pub fn eval_expression(
    env: &mut HashMap<String, Value>,
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
                let value = eval_expression(env, expr, prototypes.clone())?;

                values.push(value);
            }

            Ok(Value::List(values))
        }
        Expression::Call(name, args) => {
            let env_clone = env.clone();
            let f = env_clone
                .get(&name)
                .expect(&format!("{} function is not defined", name));

            match f {
                Value::BuiltInFn(f) => {
                    let mut values = vec![];

                    for arg in args {
                        let val = eval_expression(env, arg, prototypes.clone())?;
                        values.push(val);
                    }

                    let value = f(values)?;
                    return Ok(value);
                }
                _ => {
                    return Err(format!("{} is not a function", name));
                }
            }
        }
        Expression::Identifier(name) => {
            let data = env.get(&name);

            if let Some(data) = data {
                return Ok(data.clone());
            } else {
                println!("variable {} is not defied", &name);
                return Err(format!("variable {} is not defied", name));
            }
        }
        Expression::MethodCall(object, calle) => {
            let value = eval_expression(env, *object.clone(), prototypes.to_owned())?;

            if let Expression::Call(name, args) = *calle {
                match prototypes.get(&Type::from(&value)) {
                    Some(map) => match map.get(&name) {
                        Some(f) => {
                            if let Value::BuiltInMethod(m) = f {
                                let mut values = vec![];

                                for arg in args {
                                    let val = eval_expression(env, arg, prototypes.clone())?;
                                    values.push(val);
                                }
                                let result = m(values, value)?;
                                return Ok(result);
                            } else {
                                return Err(format!("only method call allowed"));
                            }
                        }
                        None => {
                            return Err(format!(
                                "{} method is not exist in {:?} prototype",
                                name,
                                Type::from(&value)
                            ))
                        }
                    },
                    None => {
                        return Err(format!(
                            "{} method is not exist in {:?} prototype",
                            name,
                            Type::from(&value)
                        ))
                    }
                }
            } else {
                Err(format!("only method call allowed"))
            }
        }
        Expression::Index(expr, loc) => {
            let expr_value = eval_expression(env, *expr, prototypes.clone())?;

            match &expr_value {
                Value::String(s) => {
                    let loc_value = eval_expression(env, *loc, prototypes.clone())?;

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
                    let loc_value = eval_expression(env, *loc, prototypes.clone())?;

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
            let lhs = eval_expression(env, *lhs_expr, prototypes.clone())?;
            let rhs = eval_expression(env, *rhs_expr, prototypes.clone())?;

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
            let value = eval_expression(env, *expr, prototypes)?;

            match op {
                UnaryOpKind::Not => !value,
            }
        }
    }
}
