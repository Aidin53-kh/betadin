use std::collections::BTreeMap;

use crate::runtime::value::Value;

pub struct Fs(BTreeMap<String, Value>);

impl Fs {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn exports() -> BTreeMap<String, Value> {
        let mut fs = Fs::new();

        // fs functions
        fs.declare("read_file", Value::BuiltInFn(ak_fs::read_file));
        fs.declare("read_dir", Value::BuiltInFn(ak_fs::read_dir));
        fs.declare("remove_file", Value::BuiltInFn(ak_fs::remove_file));
        fs.declare("remove_dir", Value::BuiltInFn(ak_fs::remove_dir));
        fs.declare("rename_file", Value::BuiltInFn(ak_fs::rename_file));
        fs.declare("write_file", Value::BuiltInFn(ak_fs::write_file));

        return fs.items();
    }

    pub fn declare(&mut self, name: &str, value: Value) {
        self.0.insert(String::from(name), value);
    }

    fn items(self) -> BTreeMap<String, Value> {
        return self.0;
    }
}

mod ak_fs {
    use crate::runtime::value::Value;
    use std::fs;

    pub fn read_file(vs: Vec<Value>) -> Result<Value, String> {
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

    pub fn write_file(vs: Vec<Value>) -> Result<Value, String> {
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

    pub fn rename_file(vs: Vec<Value>) -> Result<Value, String> {
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

    pub fn remove_file(vs: Vec<Value>) -> Result<Value, String> {
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

    pub fn read_dir(vs: Vec<Value>) -> Result<Value, String> {
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

    pub fn remove_dir(vs: Vec<Value>) -> Result<Value, String> {
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
}
