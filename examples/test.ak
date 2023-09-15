use std::env;
use std::fs;

let args = env.args();

if args.len() < 3 {
    panic("invalid args");
}

let path = args.at(1);
let target_line = args.at(2).to_numeric() - 1;

let lines = fs.read(path).lines();

if lines.len() < target_line {
    panic("line " + (target_line + 1) + " not found");
}

println(lines[target_line]);

