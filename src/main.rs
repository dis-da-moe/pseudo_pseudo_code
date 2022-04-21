mod lexer;
mod evaluate;
mod tokens;
mod parser;
mod test;

use parser::*;
use lexer::*;
use evaluate::*;

use chumsky::{prelude::*};

fn print_error<Error: std::fmt::Debug>(error: Error) -> String { format!("error: {:?}", error) }

fn main() -> Result<(), String>{
    let source = include_str!("main.psps");
    
    let lexed =  lexer().parse(source).map_err(print_error)?;
    
    let parsed = parser().parse(lexed).map_err(print_error)?;
    
    evaluate(parsed).map_err(print_error)?;
    
    Ok(())
}