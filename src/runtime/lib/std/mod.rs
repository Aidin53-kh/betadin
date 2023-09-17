use std::collections::BTreeMap;

use crate::runtime::value::Value;

use self::collections::Collections;
use self::env::Env;
use self::fs::Fs;
use self::system::System;

pub mod collections;
pub mod env;
pub mod fs;
pub mod system;

pub struct Std(BTreeMap<String, Value>);

impl Std {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn exports() -> BTreeMap<String, Value> {
        let mut std = Std::new();

        // std modules
        std.declare("system", Value::Module(System::exports()));
        std.declare("fs", Value::Module(Fs::exports()));
        std.declare("env", Value::Module(Env::exports()));
        std.declare("collections", Value::Module(Collections::exports()));

        return std.items();
    }

    pub fn declare(&mut self, name: &str, value: Value) {
        self.0.insert(String::from(name), value);
    }

    fn items(self) -> BTreeMap<String, Value> {
        return self.0;
    }
}
