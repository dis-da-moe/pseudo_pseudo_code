mod appendix;
mod ast;
mod evaluate;
mod lexer;
mod parser;
mod test;

use std::io::{stdin, Read};
use std::hash::Hash;
use std::fmt::Debug;
use appendix::*;
use evaluate::*;
use lexer::*;
use parser::*;

use ariadne::*;
use chumsky::{prelude::*, Stream};

use crate::ast::Execution;

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

fn display_error<T: Hash + Eq + Debug>(errors: Vec<Simple<T>>, file_name: &String, source: &String) {
    for error in errors {
        let span = error.span();
        let report = Report::build(ReportKind::Error, file_name.clone(), 10)
            .with_label(Label::new((file_name.clone(), span)))
            .with_message(format!("{:?}", error.reason()));
        
        let report = match error.reason() {
            chumsky::error::SimpleReason::Unexpected => {
                report.with_message(format!("found {:?} but expected {:?}", error.found(), error.expected().filter_map(|value| value.as_ref()).collect::<Vec<&T>>()))
            },
            chumsky::error::SimpleReason::Unclosed { span: _, delimiter: _ } => report.with_message("unclosed"),
            chumsky::error::SimpleReason::Custom(error) => report.with_message(error),
        };

        report.finish()
            .print(sources(vec![(file_name.clone(), source.as_str())]))
            .unwrap();
    }
}

pub fn parse_and_run(source: String, file_name: String) -> Result<(), ()> {
    check_empty!(source.trim());

    let lexed = lexer().parse(source.clone())
    .map_err(|errors| display_error(errors, &file_name, &source))?;

    check_empty!(lexed);

    let parsed = parser()
        .parse(Stream::from_iter(
            source.len()..source.len() + 1,
            lexed.into_iter(),
        ))
        .map_err(|errors| display_error(errors, &file_name, &source))?;

    let start_state = State {
        functions: built_ins(),
        scopes: vec![],
    };

    if let Err(error) = evaluate(&parsed, start_state, false){
        let error: Simple<Execution> = Simple::custom(error.1, error.0);
        return Err(display_error(vec![error], &file_name, &source))
    };

    Ok(())
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();

    let (source, file_name) = validate_file_arg(args.get(1))?;

    let result = if parse_and_run(source, file_name).is_err() {
        "encountered errors"
    }else{
        "has run successfully"
    };

    println!("\x1b[93mThe Program {}. Press enter to exit.\x1b[0m", result);
    stdin().read(&mut [0]).unwrap();
    Ok(())
}
