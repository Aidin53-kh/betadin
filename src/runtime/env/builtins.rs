use crate::Value;

pub fn ak_print(vs: Vec<Value>) -> Result<Value, String> {
    match vs.get(0) {
        Some(value) => {
            print!("{}", value);
            return Ok(Value::Null);
        }
        None => return Err(format!("the first argument is required")),
    }
}

pub fn ak_println(vs: Vec<Value>) -> Result<Value, String> {
    match vs.get(0) {
        Some(value) => {
            println!("{}", value);
            return Ok(Value::Null);
        }
        None => return Err(format!("the first argument is required")),
    }
}
