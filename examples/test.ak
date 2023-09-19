module core {
    const name = "core";
    const version = "0.5.0";

    module fs {
        fn read_file(path) {
            import std::fs::{read_file};

            const content = read_file(path);
            return content;
        }
    }
}

import core::fs;

const x = fs::read_file("readme.md");

println(x);
