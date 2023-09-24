use std::collections::HashMap;

use crate::runtime::value::Value;

pub mod float;
pub mod int;
pub mod list;
pub mod null;
pub mod object;
pub mod string;
pub mod tuple;

pub struct Prototypes(HashMap<String, HashMap<String, Value>>);

impl Prototypes {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn exports() -> HashMap<String, HashMap<String, Value>> {
        let mut proto = Prototypes::new();

        proto.declare("int".to_string(), int::int_proto());
        proto.declare("string".to_string(), string::string_proto());
        proto.declare("list".to_string(), list::list_proto());
        proto.declare("float".to_string(), float::float_proto());
        proto.declare("null".to_string(), null::null_proto());
        // proto.declare(ValueType::Object, object::object_proto());
        proto.declare("tuple".to_string(), tuple::tuple_proto());

        return proto.items();
    }

    pub fn declare(&mut self, t: String, proto: HashMap<String, Value>) {
        self.0.insert(t, proto);
    }

    fn items(self) -> HashMap<String, HashMap<String, Value>> {
        return self.0;
    }
}
