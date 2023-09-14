use std::fs;

use crate::runtime::value::Value;

pub fn _fs_read_file(vs: Vec<Value>) -> Result<Value, String> {
    if vs.len() > 1 || vs.len() < 1 {
        return Err(format!("expected 1 argument, but found {}", vs.len()));
    }

    match vs.get(0) {
        Some(value) => match value {
            Value::String(s) => {
                let file_result = fs::read_to_string(s);

                match file_result {
                    Ok(content) => return Ok(Value::String(content)),
                    Err(e) => return Err(e.to_string()),
                }
            }
            _ => return Err(format!("the first argument most be a string")),
        },
        None => {
            return Err(format!("expected 1 argument, but found {}", vs.len()));
        }
    }
}

pub fn _fs_write_file(vs: Vec<Value>) -> Result<Value, String> {
    if vs.len() > 2 || vs.len() < 2 {
        return Err(format!("expected 2 argument, but found {}", vs.len()));
    }

    match vs.get(0) {
        Some(v1) => match v1 {
            Value::String(path) => match vs.get(1) {
                Some(v2) => match v2 {
                    Value::String(content) => {
                        let res = fs::write(path, content);
                        match res {
                            Ok(_) => return Ok(Value::Null),
                            Err(e) => return Err(e.to_string()),
                        }
                    }
                    _ => return Err(format!("the seconde argument most be a string")),
                },
                None => return Err(format!("expected 2 argument, but found {}", vs.len())),
            },
            _ => return Err(format!("the first argument most be a string")),
        },
        None => {
            return Err(format!("expected 2 argument, but found {}", vs.len()));
        }
    }
}

pub fn _fs_rename_file(vs: Vec<Value>) -> Result<Value, String> {
    if vs.len() > 2 || vs.len() < 2 {
        return Err(format!("expected 2 argument, but found {}", vs.len()));
    }

    match vs.get(0) {
        Some(v1) => match v1 {
            Value::String(from) => match vs.get(1) {
                Some(v2) => match v2 {
                    Value::String(to) => {
                        let res = fs::rename(from, to);
                        match res {
                            Ok(_) => return Ok(Value::Null),
                            Err(e) => return Err(e.to_string()),
                        }
                    }
                    _ => return Err(format!("the first argument most be a string")),
                },
                None => return Err(format!("expected 2 argument, but found {}", vs.len())),
            },
            _ => return Err(format!("the first argument most be a string")),
        },
        None => {
            return Err(format!("expected 2 argument, but found {}", vs.len()));
        }
    }
}

pub fn _fs_remove_file(vs: Vec<Value>) -> Result<Value, String> {
    if vs.len() > 1 || vs.len() < 1 {
        return Err(format!("expected 1 argument, but found {}", vs.len()));
    }

    match vs.get(0) {
        Some(value) => match value {
            Value::String(s) => {
                let file_result = fs::remove_file(s);

                match file_result {
                    Ok(_) => return Ok(Value::Null),
                    Err(e) => return Err(e.to_string()),
                }
            }
            _ => return Err(format!("the first argument most be a string")),
        },
        None => {
            return Err(format!("expected 1 argument, but found {}", vs.len()));
        }
    }
}

pub fn _fs_read_dir(vs: Vec<Value>) -> Result<Value, String> {
    if vs.len() > 1 || vs.len() < 1 {
        return Err(format!("expected 1 argument, but found {}", vs.len()));
    }

    match vs.get(0) {
        Some(value) => match value {
            Value::String(s) => {
                let file_result = fs::read_dir(s);

                match file_result {
                    Ok(rd) => {
                        let mut items = vec![];

                        for entry in rd.into_iter() {
                            let t = entry.unwrap().file_name().to_string_lossy().to_string();
                            items.push(Value::String(t));
                        }

                        return Ok(Value::List(items));
                    }
                    Err(e) => return Err(e.to_string()),
                }
            }
            _ => return Err(format!("the first argument most be a string")),
        },
        None => {
            return Err(format!("expected 1 argument, but found {}", vs.len()));
        }
    }
}

pub fn _fs_remove_dir(vs: Vec<Value>) -> Result<Value, String> {
    if vs.len() > 1 || vs.len() < 1 {
        return Err(format!("expected 1 argument, but found {}", vs.len()));
    }

    match vs.get(0) {
        Some(value) => match value {
            Value::String(s) => {
                let file_result = fs::remove_dir(s);

                match file_result {
                    Ok(_) => return Ok(Value::Null),
                    Err(e) => return Err(e.to_string()),
                }
            }
            _ => return Err(format!("the first argument most be a string")),
        },
        None => {
            return Err(format!("expected 1 argument, but found {}", vs.len()));
        }
    }
}
