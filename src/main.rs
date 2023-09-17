use std::fs;
use std::sync::{Arc, Mutex};

use runtime::eval::program::eval_program;
use runtime::lib::Lib;
use runtime::value::Value;
use runtime::ScopeStack;

#[macro_use]
extern crate lalrpop_util;
extern crate sys_info;

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
    let lib = Lib::exports();
    let global_scope = Arc::new(Mutex::new(lib));
    let mut scopes = ScopeStack::new(vec![global_scope]);

    let code = fs::read_to_string("./examples/test.ak").expect("unable to read the file");
    let parser = grammar::programParser::new();
    let ast = parser.parse(&code).expect("unable to parse the grammar");
    // println!("{:#?}", ast);
    eval_program(&mut scopes, ast, runtime::prototypes::prototypes())?;
    // println!("{:#?}", scopes);
    Ok(())
}
