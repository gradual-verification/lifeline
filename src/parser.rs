pub mod ast;
pub mod validator;
pub mod expr;
pub mod statement;
use expr::expr;
use ast::*;

use crate::tools::arguments::Config;

pub fn parse(config: & Config) -> AST {
    let parsed = expr(&(*config).source_text);
    match parsed {
        Ok(e) => {
            print!("{}", e.1);
        }
        Err(er) => {print!("{}", er.to_string())}
    }

    
    AST {procedures: vec![], global_structs: vec![]}
}