mod evaluate;
mod lexer;
mod parser;
mod test;
mod tokens;

use std::{io::{stdin, Read}, collections::HashMap};

use evaluate::*;
use lexer::*;
use parser::*;

use chumsky::prelude::*;

fn print_error<Error: std::fmt::Debug>(error: Error) -> String {
    format!("error: {:?}", error)
}

macro_rules! check_empty {
    ($to_check: expr) => {
        if $to_check.is_empty() {
            println!("Warning: file empty.");
            return Ok(());
        }
    };
}

fn validate_file_arg(arg: Option<&String>) -> Result<String, String> {
    let string = arg.ok_or_else(|| print_error("No file provided"))?;
    let path = std::path::Path::new(string);
    std::fs::read_to_string(path).map_err(print_error)
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();

    let source = validate_file_arg(args.get(1))?;

    //let source = include_str!("main.psps").trim();

    check_empty!(source.trim());

    let lexed = lexer().parse(source).map_err(print_error)?;

    check_empty!(lexed);
    
    //println!("{:#?}", lexed);
    
    let parsed = parser().parse(lexed).map_err(print_error)?;

    //println!("{:#?}", parsed);

    evaluate(parsed, HashMap::new()).map_err(print_error)?;

    println!("Program run successfully. Press enter to exit.");
    stdin().read(&mut [0]).unwrap();
    Ok(())
}
