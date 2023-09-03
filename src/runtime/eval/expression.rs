use std::collections::HashMap;

use crate::{ast::expression::Expression, Value};

pub fn eval_expression(
    env: &mut HashMap<String, Value>,
    expression: Expression,
) -> Result<Value, String> {
    match expression {
        Expression::Literal(v) => Ok(Value::Literal(v)),
    }
}
