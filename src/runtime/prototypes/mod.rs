use std::collections::HashMap;

use crate::runtime::value::{Type, Value};

pub mod float;
pub mod int;
pub mod list;
pub mod null;
pub mod object;
pub mod string;
pub struct Prototypes(HashMap<Type, HashMap<String, Value>>);

impl Prototypes {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn exports() -> HashMap<Type, HashMap<String, Value>> {
        let mut proto = Prototypes::new();

        proto.declare(Type::Int, int::int_proto());
        proto.declare(Type::String, string::string_proto());
        proto.declare(Type::List, list::list_proto());
        proto.declare(Type::Float, float::float_proto());
        proto.declare(Type::Null, null::null_proto());
        proto.declare(Type::Object, object::object_proto());

        return proto.items();
    }

    pub fn declare(&mut self, t: Type, proto: HashMap<String, Value>) {
        self.0.insert(t, proto);
    }

    fn items(self) -> HashMap<Type, HashMap<String, Value>> {
        return self.0;
    }
}
