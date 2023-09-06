use crate::runtime::value::Value;

pub fn ak_set(vs: Vec<Value>) -> Result<Value, String> {
    match vs.get(0) {
        Some(value) => {
            if let Value::List(list) = value {
                let mut set = Vec::new();
                for val in list.clone() {
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
