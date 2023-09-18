use std::collections::HashMap;

use crate::runtime::value::Value;

use super::list::_at;

pub fn tuple_proto() -> HashMap<String, Value> {
    let mut tuple_proto = HashMap::new();

    tuple_proto.insert(String::from("at"), Value::BuiltInMethod(_at, None));

    tuple_proto
}
