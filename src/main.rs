use std::fs;

use runtime::env::Env;
use runtime::eval::program::eval_program;
use runtime::value::Value;

#[macro_use]
extern crate lalrpop_util;

lalrpop_util::lalrpop_mod!(pub grammar);

mod ast;
mod runtime;
mod utils;

#[derive(Debug, Clone)]
pub enum Export {
    Module { name: String, exports: Vec<Export> },
    Item { name: String, value: Value },
}

fn main() -> Result<(), String> {
    let code = fs::read_to_string("./examples/test.ak").expect("unable to read the file");
    let parser = grammar::programParser::new();
    let ast = parser.parse(&code).expect("unable to parse the grammar");
    // println!("{:#?}", ast);
    eval_program(
        &mut Env::new(),
        ast,
        runtime::std::modules(),
        runtime::std::prototypes(),
    )
}
