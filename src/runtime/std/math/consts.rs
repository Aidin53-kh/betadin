use std::f32::consts::PI;

use crate::{runtime::value::Value, Export};

pub fn exports() -> Vec<Export> {
    vec![Export::Item {
        name: String::from("PI"),
        value: Value::Float(PI),
    }]
}
