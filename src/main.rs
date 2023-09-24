use std::sync::{Arc, Mutex};
use std::{env, fs};

use lalrpop_util::ParseError;
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

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(path) => {
            let mut scopes = ScopeStack::new(vec![Arc::new(Mutex::new(StdLib::exports()))]);

            let code = fs::read_to_string(path).expect("unable to read the file");
            let parser = grammar::programParser::new();
            let ast = parser.parse(&code).map_err(|e| match e {
                ParseError::InvalidToken { location } => format!("InvalidToken at {}", location),
                ParseError::UnrecognizedEof {
                    location,
                    expected: _,
                } => format!("UnrecognizedEof at {}", location),
                ParseError::UnrecognizedToken { token, expected: _ } => {
                    format!("UnrecognizedToken: {} -> {}:{}", token.1, token.0, token.2)
                }
                ParseError::ExtraToken { token } => format!("ExtraToken: {}", token.1),
                ParseError::User { error } => format!("Error: {}", error),
            })?;

            // println!("{:#?}", ast);
            eval_program(&mut scopes, ast, &Prototypes::exports())?;
            Ok(())
        }
        None => Err(format!("The file path is require")),
    }
}
