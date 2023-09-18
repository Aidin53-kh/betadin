use std::fs;
use std::sync::{Arc, Mutex};

use runtime::eval::eval_program;
use runtime::Lib;
use runtime::Prototypes;
use runtime::ScopeStack;

#[macro_use]
extern crate lalrpop_util;
extern crate sys_info;

lalrpop_util::lalrpop_mod!(pub grammar);

mod ast;
mod runtime;
mod utils;

fn main() -> Result<(), String> {
    let mut scopes = ScopeStack::new(vec![Arc::new(Mutex::new(Lib::exports()))]);

    let code = fs::read_to_string("./examples/test.ak").expect("unable to read the file");
    let parser = grammar::programParser::new();
    let ast = parser.parse(&code).expect("unable to parse the grammar");
    // println!("{:#?}", ast);
    eval_program(&mut scopes, ast, &Prototypes::exports())?;
    // println!("{:#?}", scopes);
    Ok(())
}
