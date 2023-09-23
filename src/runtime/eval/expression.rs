use std::collections::{BTreeMap, HashMap};
use std::fs;

use crate::ast::{BinaryOpKind, Branch, Expr, Prop, Statement, UnaryOpKind};
use crate::grammar;
use crate::runtime::value::{KeyValue, Type, Value};
use crate::runtime::{DeclType, Prototypes, ScopeStack};

use super::program::eval_program_and_push_scope;
use super::statement::{eval_module, eval_statements, Escape};

pub fn eval_expression(
    scopes: &mut ScopeStack,
    expression: &Expr,
    prototypes: &HashMap<Type, HashMap<String, Value>>,
) -> Result<Value, String> {
    match expression {
        Expr::Null => eval_null_expr(),
        Expr::Int(n) => eval_int_expr(n),
        Expr::Float(n) => eval_float_expr(n),
        Expr::String(s) => eval_stirng_expr(s),
        Expr::Bool(b) => eval_bool_expr(b),
        Expr::List(list) => eval_list_expr(scopes, prototypes, list),
        Expr::Call(expr, args) => eval_call_expr(scopes, prototypes, expr, args),
        Expr::Identifier(name) => eval_ident_expr(scopes, name),
        Expr::MethodCall(object, calle) => eval_method_call_expr(scopes, prototypes, object, calle),
        Expr::Index(expr, loc) => eval_index_expr(scopes, prototypes, expr, loc),
        Expr::BinaryOp(lhs, op, rhs) => eval_binary_expr(scopes, prototypes, lhs, op, rhs),
        Expr::UnaryOp(op, expr) => eval_unary_expr(scopes, prototypes, op, expr),
        Expr::Object(props) => eval_object_expr(scopes, prototypes, props),
        Expr::Fn(args, block) => eval_fn_expr(args, block),
        Expr::ModuleCall(paths, expr) => eval_module_call_expr(scopes, prototypes, paths, expr),
        Expr::Module(statements) => eval_module_expr(scopes, prototypes, statements),
        Expr::If(branchs, else_block) => eval_if_expr(scopes, prototypes, branchs, else_block),
        Expr::Tuple(exprs) => eval_tuple_expr(scopes, prototypes, exprs),
        Expr::Range(start, end) => eval_range_expr(scopes, prototypes, start, end),
    }
}

pub fn eval_null_expr() -> Result<Value, String> {
    Ok(Value::Null)
}

pub fn eval_stirng_expr(s: &String) -> Result<Value, String> {
    Ok(Value::String(s.to_string()))
}

pub fn eval_int_expr(n: &i32) -> Result<Value, String> {
    Ok(Value::Int(*n))
}

pub fn eval_float_expr(n: &f32) -> Result<Value, String> {
    Ok(Value::Float(*n))
}

pub fn eval_bool_expr(b: &bool) -> Result<Value, String> {
    Ok(Value::Bool(*b))
}

pub fn eval_list_expr(
    scopes: &mut ScopeStack,
    prototypes: &HashMap<Type, HashMap<String, Value>>,
    list: &Vec<Expr>,
) -> Result<Value, String> {
    let mut values: Vec<Value> = Vec::new();

    for expr in list {
        let value = eval_expression(scopes, expr, &prototypes)?;

        values.push(value);
    }

    Ok(Value::List(values))
}

pub fn eval_call_expr(
    scopes: &mut ScopeStack,
    prototypes: &HashMap<Type, HashMap<String, Value>>,
    expr: &Box<Expr>,
    args: &Vec<Expr>,
) -> Result<Value, String> {
    let value = eval_expression(scopes, &expr, &prototypes)?;

    match value {
        Value::BuiltInFn(f) => {
            let mut values = vec![];

            for arg in args {
                let val = eval_expression(scopes, arg, &prototypes)?;
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
                        let value = eval_expression(&mut inner_scope, expr, &prototypes)?;

                        inner_scope.declare(param, value, DeclType::Mutable)?;
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

            let ret = eval_statements(&mut inner_scope, &block, prototypes)?;
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
                let val = eval_expression(scopes, arg, &prototypes)?;
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

pub fn eval_ident_expr(scopes: &mut ScopeStack, name: &String) -> Result<Value, String> {
    match scopes.get(&name) {
        Some(v) => Ok(v),
        None => Err(format!("{} is not defied (8)", name)),
    }
}

pub fn eval_method_call_expr(
    scopes: &mut ScopeStack,
    prototypes: &HashMap<Type, HashMap<String, Value>>,
    object: &Box<Expr>,
    calle: &Box<Expr>,
) -> Result<Value, String> {
    let obj_value = eval_expression(scopes, &*object, &prototypes)?;

    match *calle.clone() {
        Expr::Identifier(name) => match prototypes.get(&Type::from(&obj_value)) {
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
        Expr::Call(expr, args) => match *expr {
            Expr::Identifier(name) => match prototypes.get(&Type::from(&obj_value)) {
                Some(proto) => match proto.get(&name) {
                    Some(value) => match value {
                        Value::BuiltInMethod(f, _) => {
                            let mut values = vec![];

                            for arg in args {
                                let val = eval_expression(scopes, &arg, &prototypes)?;
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
        Expr::Int(n) => {
            return Err(format!(
                "{} is not found in {} prototype",
                n,
                Type::from(&obj_value)
            ))
        }

        _ => {
            return Err(format!(
                "value of type {:?} not callable (1)",
                Type::from(&obj_value)
            ));
        }
    }
}

pub fn eval_index_expr(
    scopes: &mut ScopeStack,
    prototypes: &HashMap<Type, HashMap<String, Value>>,
    expr: &Box<Expr>,
    loc: &Box<Expr>,
) -> Result<Value, String> {
    let expr_value = eval_expression(scopes, &*expr, &prototypes)?;

    match &expr_value {
        Value::String(s) => {
            let loc_value = eval_expression(scopes, &*loc, &prototypes)?;

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
        Value::List(l) | Value::Tuple(l) => {
            let loc_value = eval_expression(scopes, &*loc, &prototypes)?;

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

pub fn eval_binary_expr(
    scopes: &mut ScopeStack,
    prototypes: &HashMap<Type, HashMap<String, Value>>,
    lhs: &Box<Expr>,
    op: &BinaryOpKind,
    rhs: &Box<Expr>,
) -> Result<Value, String> {
    let lhs = eval_expression(scopes, &*lhs, &prototypes)?;
    let rhs = eval_expression(scopes, &*rhs, &prototypes)?;

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

pub fn eval_unary_expr(
    scopes: &mut ScopeStack,
    prototypes: &HashMap<Type, HashMap<String, Value>>,
    op: &UnaryOpKind,
    expr: &Box<Expr>,
) -> Result<Value, String> {
    let value = eval_expression(scopes, &*expr, prototypes)?;

    match op {
        UnaryOpKind::Not => !value,
        UnaryOpKind::Typeof => Ok(Value::String(Type::from(&value).to_string())),
    }
}

pub fn eval_object_expr(
    scopes: &mut ScopeStack,
    prototypes: &HashMap<Type, HashMap<String, Value>>,
    props: &Vec<Prop>,
) -> Result<Value, String> {
    let mut values: Vec<KeyValue> = Vec::new();

    for prop in props {
        let value = eval_expression(scopes, &prop.value, &prototypes)?;

        values.push(KeyValue {
            key: prop.key.to_string(),
            value,
        });
    }

    Ok(Value::Object(values))
}

pub fn eval_fn_expr(args: &Vec<String>, block: &Vec<Statement>) -> Result<Value, String> {
    Ok(Value::Func(args.to_vec(), block.to_vec()))
}

pub fn eval_module_call_expr(
    scopes: &mut ScopeStack,
    prototypes: &HashMap<Type, HashMap<String, Value>>,
    paths: &Vec<String>,
    expr: &Box<Expr>,
) -> Result<Value, String> {
    let module = get_module(scopes, paths)?;

    let mut inner_scopes = scopes.new_from_push(HashMap::new());

    for (key, value) in module {
        inner_scopes.declare(&key, value, DeclType::Immutable)?;
    }

    let value = eval_expression(&mut inner_scopes, &*expr, &prototypes)?;
    Ok(value)
}

pub fn eval_module_expr(
    scopes: &mut ScopeStack,
    prototypes: &HashMap<Type, HashMap<String, Value>>,
    statements: &Vec<Statement>,
) -> Result<Value, String> {
    let module = eval_module(scopes, prototypes, &String::from("test"), statements)?;
    Ok(Value::Module(module))
}

pub fn eval_if_expr(
    scopes: &mut ScopeStack,
    prototypes: &HashMap<Type, HashMap<String, Value>>,
    branchs: &Vec<Branch>,
    else_block: &Option<Vec<Statement>>,
) -> Result<Value, String> {
    for branch in branchs {
        let value = eval_expression(scopes, &branch.condition, &prototypes)?;

        match value {
            Value::Bool(b) => {
                if b {
                    let ret = eval_statements(scopes, &branch.statements, &prototypes)?;

                    if let Escape::Return(value) = ret {
                        return Ok(value);
                    }

                    return Ok(Value::Null);
                }
            }
            _ => return Err(format!("condition most be a boolean")),
        }
    }

    if let Some(stmts) = else_block {
        let e = eval_statements(scopes, stmts, &prototypes)?;

        if let Escape::Return(value) = e {
            return Ok(value);
        }
    }

    Ok(Value::Null)
}

pub fn eval_tuple_expr(
    scopes: &mut ScopeStack,
    prototypes: &HashMap<Type, HashMap<String, Value>>,
    exprs: &Vec<Expr>,
) -> Result<Value, String> {
    let mut values = Vec::new();

    for expr in exprs {
        let value = eval_expression(scopes, expr, &prototypes)?;
        values.push(value);
    }

    Ok(Value::Tuple(values))
}

pub fn eval_range_expr(
    scopes: &mut ScopeStack,
    prototypes: &HashMap<Type, HashMap<String, Value>>,
    start: &Box<Expr>,
    end: &Box<Expr>,
) -> Result<Value, String> {
    let start = eval_expression(scopes, &start, prototypes)?;
    let end = eval_expression(scopes, &end, prototypes)?;

    match start {
        Value::Int(s) => match end {
            Value::Int(e) => {
                let mut list = Vec::new();

                for num in s..=e {
                    list.push(Value::Int(num));
                }

                return Ok(Value::List(list));
            }
            other => return Err(format!("extected integer, found {}", Type::from(&other))),
        },
        other => return Err(format!("expected integer, found {}", Type::from(&other))),
    }
}

pub fn get_module(
    scopes: &mut ScopeStack,
    paths: &Vec<String>,
) -> Result<BTreeMap<String, Value>, String> {
    let mut exports: BTreeMap<String, Value> = BTreeMap::new();

    for path in paths {
        match exports.get(path) {
            Some(value) => match value.to_owned() {
                Value::Module(items) => {
                    exports = items;
                }
                _ => return Err(format!("module {} not found (1)", path)),
            },
            None => match scopes.get(path) {
                Some(value) => match value {
                    Value::Module(items) => {
                        exports = items;
                    }
                    _ => return Err(format!("module {} not found (2)", path)),
                },
                None => {
                    let mut path = String::new();
                    path.push_str("./examples/");
                    path.push_str(&paths.join("/"));
                    path.push_str(".ak");

                    let file_result = fs::read_to_string(path);

                    if let Ok(file) = file_result {
                        // scopes.push();
                        let program = grammar::programParser::new().parse(&file).expect(&format!(
                            "unable to compile module {}",
                            paths.last().unwrap()
                        ));
                        eval_program_and_push_scope(scopes, program, &Prototypes::exports())?;
                        break;
                    } else {
                        return Err(format!("module {} not found (3)", paths.last().unwrap()));
                    }
                }
            },
        }
    }

    Ok(exports)
}
