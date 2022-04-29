use std::collections::HashMap;

use crate::ast::*;
use rand::Rng;

macro_rules! insert {
    ($functions: ident, $($call: ident, $name: expr),+) => {
        $($functions.insert(
            String::from($name),
            Function::BuiltIn($call)
        );)+
    };
}

macro_rules! expect {
    ($args: expr, $($index: expr => $enum: ident),+) => {
        ($(match &$args[$index] {
            Literal::$enum(value) => Ok(value),
            _ => Err(Execution::IncorrectType((&LiteralType::$enum).into(), (&LiteralType::from(&$args[$index])).into()))
        }),+)
    };
}

macro_rules! expect_return {
    ($args: expr, $($index: expr => $enum: ident),+) => {
        ($(match &$args[$index] {
            Literal::$enum(value) => value,
            _ => return Err(Execution::IncorrectType((&LiteralType::$enum).into(), (&LiteralType::from(&$args[$index])).into()))
        }),+)
    };
}

macro_rules! wrong_type {
    ($expected: ident, $received: ident) => {wrong_type!(LiteralType::$expected, LiteralType::$received)};
    ($expected: ident, $received: expr) => {wrong_type!((&LiteralType::$expected).into(), (&LiteralType::from(&$received)).into())};
    ($expected: expr, $received: ident) => {wrong_type!(LiteralType::from(&$expected)), LiteralType::$received};
    ($expected: expr, $received: expr) => {
        return Err(Execution::IncorrectType($expected, $received))
    };
}

macro_rules! number_args {
    ($args: expr, $name: expr, $expected_len: expr) => {{
        let length = $args.len();
        let fixed_args: [Literal; $expected_len] = $args.try_into().map_err(|_| {
            Execution::IncorrectNumberArguments(String::from($name), $expected_len, length)
        })?;
        fixed_args
    }};
}

pub fn built_ins() -> HashMap<String, Function> {
    let mut functions = HashMap::new();

    insert!(
        functions,
        str_to_num,
        "STR_TO_NUM",
        num_to_str,
        "NUM_TO_STR",
        randombetween,
        "RANDOMBETWEEN"
    );

    functions
}

fn randombetween(args: Vec<Literal>) -> Result<Literal, Execution> {
    let args: [Literal; 2] = number_args!(args, "RANDOMBETWEEN", 2);
    let (lower, upper) = expect_return!(args,
        0 => Integer,
        1 => Integer
    );
    let mut rng = rand::thread_rng();
    return Ok(Literal::Integer(rng.gen_range(*lower..*upper)));
}

fn str_to_num(args: Vec<Literal>) -> Result<Literal, Execution> {
    let args: [Literal; 1] = number_args!(args, "STR_TO_NUM", 1);
    let string = expect!(args,
        0 => String
    )?;

    let value: isize = string
        .parse()
        .map_err(|_| Execution::CanNotParse(String::from("")))?;

    Ok(Literal::Integer(value))
}

fn num_to_str(args: Vec<Literal>) -> Result<Literal, Execution> {
    let args: [Literal; 1] = number_args!(args, "NUM_TO_STR", 1);
    let typed_args = expect!(args,
        0 => Integer,
        0 => Real
    );
    let string = match typed_args {
        (Ok(int), _) => int.to_string(),
        (_, Ok(real)) => real.to_string(),
        _ => wrong_type!(Integer, args[0]),
    };

    return Ok(Literal::String(string));
}
