mod lexer;
use lexer::*;
use chumsky::{prelude::*};

fn main() {
    let source = include_str!("main.psps");
    let lexed = lexer().parse(source);
    match lexed {
        Ok(result) => {
            let parsed = parser().parse(result);
            match parsed {
                Ok(result) => {
                    let evaluated  = evaluate(result);
                    match evaluated {
                        Err(error) => println!("{:?}", error),
                        _ => {}
                    }
                },
                Err(error) => println!("{:?}", error)
            }
        },
        Err(error) => println!("{:?}", error)
    };
}