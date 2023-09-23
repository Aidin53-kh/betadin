# Betadin

betadin is a simple programing language for research purposes, written in rust.
<br/>

## Features

-   variables and constants
-   functions
-   conditional commands
-   common operators
-   for and while loops
-   prototypes
-   builtin modules and functions
-   and more

## Usage
 1. install betadin with cargo
```bash
cargo install betadin
```

2. running files with betadin cli 
```bash
betadin path.ak
```

## Examples


```rust

# variables and constants

let name1 = "something"; # can be change
const name2 = "something"; # cannot be change



# functions

fn add(x, y) {
    return x + y;
}

println(add(2, 5)); # prints 7;



# conditional commands and operators

let x = 10;

# if statement
if x >= 10 || true {
    println("if block");
} else {
    println("else block");
}

# if expression
let y = if x == 10 {
    return "if block";
} else {
    return "else block";
};

println(y); # prints "if block"



# for and while loops

for i in 1..10 {
    println(i); # prints 1 to 10
}

let x = 0;
while x <= 10 {
    if x == 5 {
        break;
    }
    println(x); # prints 0 to 4
    x = x + 1;
}



# builtin modules and prototypes
# betadin have some builtin modules like "fs", "system", "env"

# system module

import std::system;
println(system::platform());

# or

import std::system::{platform};
println(platform());

# or 

println(std::system::platform());

# file system

import std::fs;

const content = fs::read_file("path"); # read

# reading file content line
# file content is a string so we can use string methods

println(content.lines()); # prints array of lines
println(content.len()); # prints number of chars
println(content.lines().len()); # prints number of lines;

# more methods
fs::write_file("path");
fs::read_dir("path");
fs::remove_file("path");
# and more


# env module
import std::env;
const args = env::args(); # getting arguments

for arg in args {
    println(arg);
}


# custom module
module custom {
    const name = "custom";
    module inner {
        fn get_parent_name() {
            return custom::inner::name;
        }
    }
}

println(custom::inner::get_parent_name()); # custom

# or 
import custom::inner::{get_parent_name};
println(get_parent_name()) # custom

```

more examples: https://github.com/Aidin53-kh/betadin/examples
