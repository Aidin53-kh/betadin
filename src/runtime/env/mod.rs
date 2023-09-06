use crate::Value;
use std::collections::HashMap;

mod builtins;

pub struct Env;

impl Env {
    pub fn new() -> HashMap<String, Value> {
        let mut env = HashMap::<String, Value>::new();

        env.insert(String::from("print"), Value::BuiltInFn(builtins::ak_print));
        env.insert(
            String::from("println"),
            Value::BuiltInFn(builtins::ak_println),
        );

        env
    }
}
