use runtime::eval::{
    self,
    program::{eval_program, Value},
};
use std::{collections::HashMap, fs};

#[macro_use]
extern crate lalrpop_util;

lalrpop_util::lalrpop_mod!(pub grammar);

mod ast;
mod runtime;

fn main() {
    let mut env = HashMap::<String, Value>::new();

    let code = fs::read_to_string("./examples/test.ak").expect("unable to read the file");
    let parser = grammar::programParser::new();
    let ast = parser.parse(&code).expect("unable to parse the grammar");

    println!("{:#?}", ast);

    let result = eval_program(&mut env, ast);

    println!("program result: {:?}", result);
}
