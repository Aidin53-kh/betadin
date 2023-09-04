use std::collections::HashMap;


use crate::{ast::expression::Expression, Value};

pub fn eval_expression(
    env: &mut HashMap<String, Value>,
    expression: Expression,
) -> Result<Value, String> {
    
    match expression {
        Expression::Literal(v) => Ok(Value::Literal(v)),
        Expression::Call(name, args) => {
            let env_clone = env.clone();
            let f = env_clone
                .get(&name)
                .expect(&format!("{} function is not defined", name));

            match f {
                Value::BuiltInFn(f) => {
                    let mut values = vec![];

                    for arg in args {
                        let val = eval_expression(env, arg)?;
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
    }
}
