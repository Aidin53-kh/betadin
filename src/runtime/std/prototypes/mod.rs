use std::collections::HashMap;

use crate::runtime::value::{Type, Value};

pub mod float;
pub mod int;
pub mod list;
pub mod string;

pub type Prototypes = HashMap<Type, HashMap<String, Value>>;

pub fn prototypes() -> Prototypes {
    let mut proto: HashMap<Type, HashMap<String, Value>> = HashMap::new();

    let mut int_proto = HashMap::new();
    let mut string_proto = HashMap::new();
    let mut list_proto = HashMap::new();

    int_proto.insert(String::from("pow"), Value::BuiltInMethod(int::_pow));
    string_proto.insert(String::from("len"), Value::BuiltInMethod(string::_len));
    list_proto.insert(String::from("push"), Value::BuiltInMethod(list::_push));

    proto.insert(Type::Int, int_proto);
    proto.insert(Type::String, string_proto);
    proto.insert(Type::List, list_proto);

    proto
}
