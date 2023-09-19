use std::sync::{Arc, Mutex};
use std::{env, fs};

use runtime::eval::eval_program;
use runtime::Prototypes;
use runtime::ScopeStack;
use runtime::StdLib;

#[macro_use]
extern crate lalrpop_util;
extern crate sys_info;

lalrpop_util::lalrpop_mod!(pub grammar);

mod ast;
mod runtime;
mod utils;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(path) => {
            let mut scopes = ScopeStack::new(vec![Arc::new(Mutex::new(StdLib::exports()))]);

            let code = fs::read_to_string(path).expect("unable to read the file");
            let parser = grammar::programParser::new();
            let ast = parser.parse(&code).expect("unable to parse the grammar");
            // println!("{:#?}", ast);
            eval_program(&mut scopes, ast, &Prototypes::exports())?;
            // println!("{:#?}", scopes);
            Ok(())
        }
        None => Err(format!("The file path is require")),
    }
}
