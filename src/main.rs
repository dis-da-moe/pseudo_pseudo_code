mod appendix;
mod ast;
mod evaluate;
mod lexer;
mod parser;

use std::io::{stdin, Read};

use appendix::*;
use evaluate::*;
use lexer::*;
use parser::*;

use ariadne::*;
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

fn validate_file_arg(arg: Option<&String>) -> Result<(String, String), String> {
    let string = arg.ok_or_else(|| print_error("No file provided"))?;
    let path = std::path::Path::new(string);
    let source = std::fs::read_to_string(path).map_err(print_error)?;
    Ok((
        source,
        path.file_name().unwrap().to_str().unwrap().to_string(),
    ))
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();

    let (source, file_name) = validate_file_arg(args.get(1))?;

    //let source = include_str!("main.psps").trim();

    check_empty!(source.trim());

    let lexed = lexer().parse(source.clone()).map_err(|errors| {
        for error in errors {
            let span = error.span();
            Report::build(ReportKind::Error, file_name.clone(), 10)
                .with_label(Label::new((file_name.clone(), span)))
                .with_message(format!("{:?}", error.reason()))
                .finish()
                .print(sources(vec![(file_name.clone(), source.as_str())]))
                .unwrap();
        }
        String::from("")
    })?;

    check_empty!(lexed);

    //println!("{:#?}", lexed);

    let parsed = parser().parse(lexed).map_err(print_error)?;

    //println!("{:#?}", parsed);
    let start_state = State {
        functions: built_ins(),
        scopes: vec![],
    };

    evaluate(&parsed, start_state, false).map_err(print_error)?;

    println!("\x1b[93mThe Program has run successfully. Press enter to exit.\x1b[0m");
    stdin().read(&mut [0]).unwrap();
    Ok(())
}
