use std::collections::HashMap;

use crate::runtime::value::{Type, Value};

pub mod float;
pub mod int;
pub mod list;
pub mod null;
pub mod object;
pub mod string;

pub type Prototypes = HashMap<Type, HashMap<String, Value>>;

pub fn prototypes() -> Prototypes {
    let mut proto: HashMap<Type, HashMap<String, Value>> = HashMap::new();

    proto.insert(Type::Int, int::int_proto());
    proto.insert(Type::String, string::string_proto());
    proto.insert(Type::List, list::list_proto());
    proto.insert(Type::Float, float::float_proto());
    proto.insert(Type::Null, null::null_proto());
    proto.insert(Type::Object, object::object_proto());

    proto
}
