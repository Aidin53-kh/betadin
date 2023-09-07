use crate::runtime::value::Value;
use std::collections::HashMap;

pub use super::string::_to_string;

pub fn null_proto() -> HashMap<String, Value> {
    let mut null_proto = HashMap::new();

    null_proto.insert(String::from("to_string"), Value::BuiltInMethod(_to_string));

    null_proto
}
