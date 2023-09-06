use crate::runtime::value::Value;
use crate::Export;

pub mod collections;
pub mod math;
pub mod string;

pub fn modules() -> Vec<Export> {
    vec![Export::Module {
        name: String::from("std"),
        exports: vec![
            Export::Module {
                name: String::from("math"),
                exports: vec![
                    Export::Module {
                        name: String::from("consts"),
                        exports: math::consts::exports(),
                    },
                    Export::Item {
                        name: String::from("add"),
                        value: Value::BuiltInFn(math::ak_add),
                    },
                    Export::Item {
                        name: String::from("mul"),
                        value: Value::BuiltInFn(math::ak_mul),
                    },
                    Export::Item {
                        name: String::from("div"),
                        value: Value::BuiltInFn(math::ak_div),
                    },
                    Export::Item {
                        name: String::from("sub"),
                        value: Value::BuiltInFn(math::ak_sub),
                    },
                    Export::Item {
                        name: String::from("cos"),
                        value: Value::BuiltInFn(math::ak_cos),
                    },
                    Export::Item {
                        name: String::from("sin"),
                        value: Value::BuiltInFn(math::ak_sin),
                    },
                    Export::Item {
                        name: String::from("abs"),
                        value: Value::BuiltInFn(math::ak_abs),
                    },
                    Export::Item {
                        name: String::from("tan"),
                        value: Value::BuiltInFn(math::ak_tan),
                    },
                    Export::Item {
                        name: String::from("pow"),
                        value: Value::BuiltInFn(math::ak_pow),
                    },
                ],
            },
            Export::Module {
                name: String::from("string"),
                exports: vec![Export::Item {
                    name: String::from("len"),
                    value: Value::BuiltInFn(string::ak_len),
                }],
            },
            Export::Module {
                name: String::from("collections"),
                exports: vec![Export::Item {
                    name: String::from("set"),
                    value: Value::BuiltInFn(collections::ak_set),
                }],
            },
        ],
    }]
}
