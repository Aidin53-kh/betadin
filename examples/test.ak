use std::math::pow;
use std::collections::set;
use std::fs;

for file in fs.read_dir("./examples") {
    if file == "test.ak" {
        let content = fs.read("./examples/test.ak");

        println(content);
    }
}

