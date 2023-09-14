use crate::runtime::value::Value;
use crate::Export;

pub mod collections;
pub mod fs;
pub mod math;
pub mod prototypes;

pub use prototypes::{prototypes, Prototypes};

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
                name: String::from("collections"),
                exports: vec![Export::Item {
                    name: String::from("set"),
                    value: Value::BuiltInFn(collections::ak_set),
                }],
            },
            Export::Module {
                name: String::from("fs"),
                exports: vec![
                    Export::Item {
                        name: String::from("read"),
                        value: Value::BuiltInFn(fs::_fs_read_file),
                    },
                    Export::Item {
                        name: String::from("write"),
                        value: Value::BuiltInFn(fs::_fs_write_file),
                    },
                    Export::Item {
                        name: String::from("rename"),
                        value: Value::BuiltInFn(fs::_fs_rename_file),
                    },
                    Export::Item {
                        name: String::from("remove"),
                        value: Value::BuiltInFn(fs::_fs_remove_file),
                    },
                    Export::Item {
                        name: String::from("read_dir"),
                        value: Value::BuiltInFn(fs::_fs_read_dir),
                    },
                    Export::Item {
                        name: String::from("remove_dir"),
                        value: Value::BuiltInFn(fs::_fs_remove_dir),
                    },
                ],
            },
            Export::Module {
                name: String::from("system"),
                exports: vec![],
            },
        ],
    }]
}
