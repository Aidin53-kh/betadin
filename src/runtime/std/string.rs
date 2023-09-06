use crate::runtime::value::Value;

pub fn ak_len(vs: Vec<Value>) -> Result<Value, String> {
    match vs.get(0) {
        Some(value) => {
            if let Value::String(s) = value {
                return Ok(Value::Int(s.len() as i32));
            } else {
                return Err(format!("the first argument most be string"));
            }
        }
        None => Err(format!("the first argument is required")),
    }
}
