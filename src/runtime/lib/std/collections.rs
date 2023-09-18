use std::collections::BTreeMap;

use crate::runtime::value::Value;

pub struct Collections(BTreeMap<String, Value>);

impl Collections {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn exports() -> BTreeMap<String, Value> {
        let mut std = Collections::new();

        // collections functions
        std.declare("set", Value::BuiltInFn(ak_collections::set));

        return std.items();
    }

    pub fn declare(&mut self, name: &str, value: Value) {
        self.0.insert(String::from(name), value);
    }

    fn items(self) -> BTreeMap<String, Value> {
        return self.0;
    }
}

mod ak_collections {
    use crate::runtime::value::Value;

    pub fn set(vs: Vec<Value>) -> Result<Value, String> {
        match vs.get(0) {
            Some(value) => {
                if let Value::List(list) = value {
                    let mut set = Vec::new();
                    let list = list.to_vec();
                    for val in list {
                        if !set.contains(&val) {
                            set.push(val);
                        }
                    }

                    return Ok(Value::List(set));
                } else {
                    return Err(format!("the first argument most be a list"));
                }
            }
            None => Err(format!("the first argument is required")),
        }
    }
}
