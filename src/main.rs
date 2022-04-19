mod lexer;
use lexer::*;
use chumsky::{prelude::*};

fn main() {
    let source = include_str!("main.psps");
    let result = lexer().parse(source);
    
    println!("{:?}", result);
    let whitespace = ' '.is_whitespace();
    let new_line = '\n'.is_whitespace();
    println!("whitespace: {}, newline: {}", whitespace, new_line);
}