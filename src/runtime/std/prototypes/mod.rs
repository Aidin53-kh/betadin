use std::collections::HashMap;

use crate::runtime::value::{Type, Value};

pub mod float;
pub mod int;
pub mod list;
pub mod null;
pub mod string;

pub type Prototypes = HashMap<Type, HashMap<String, Value>>;

pub fn prototypes() -> Prototypes {
    let mut proto: HashMap<Type, HashMap<String, Value>> = HashMap::new();

    let mut int_proto = HashMap::new();
    let mut float_proto = HashMap::new();
    let mut string_proto = HashMap::new();
    let mut list_proto = HashMap::new();
    let mut null_proto = HashMap::new();

    int_proto.insert(String::from("pow"), Value::BuiltInMethod(int::_pow));
    int_proto.insert(
        String::from("to_string"),
        Value::BuiltInMethod(int::_to_string),
    );
    string_proto.insert(String::from("len"), Value::BuiltInMethod(string::_len));
    string_proto.insert(
        String::from("to_string"),
        Value::BuiltInMethod(string::_to_string),
    );
    list_proto.insert(String::from("push"), Value::BuiltInMethod(list::_push));
    float_proto.insert(String::from("pow"), Value::BuiltInMethod(float::_pow));
    float_proto.insert(
        String::from("to_string"),
        Value::BuiltInMethod(float::_to_string),
    );
    null_proto.insert(
        String::from("to_string"),
        Value::BuiltInMethod(null::_to_string),
    );

    proto.insert(Type::Int, int_proto);
    proto.insert(Type::String, string_proto);
    proto.insert(Type::List, list_proto);
    proto.insert(Type::Float, float_proto);
    proto.insert(Type::Null, null_proto);

    proto
}
