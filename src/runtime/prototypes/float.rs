use std::collections::HashMap;

use crate::runtime::value::Value;

pub use super::int::_pow;
pub use super::string::_to_string;

pub fn float_proto() -> HashMap<String, Value> {
    let mut float_proto = HashMap::new();

    float_proto.insert(
        String::from("pow"),
        Value::BuiltInMethod(_pow, None),
    );
    float_proto.insert(
        String::from("to_string"),
        Value::BuiltInMethod(_to_string, None),
    );

    float_proto
}
