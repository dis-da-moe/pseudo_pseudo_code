use chumsky::{prelude::*};
use chumsky::text::{newline};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DataTypes {
    Integer,
    Real
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Token {
    Declare,
    Identifier(String),
    Assign,
    Integer(u32),
    Out,
    Colon,
    DataType(DataTypes),
    NewLine
}

pub enum Value {
    Integer(u32)
}

pub enum Statement {
    Declare(String, DataTypes),
    Assign(String),
    Out
}

pub fn lexer() -> impl Parser<char, Vec<Token>, Error = Simple<char>>{
    
    let number = text::digits(10).map(|int: String| Token::Integer(int.parse().unwrap()));
    
    let identifiers = text::ident().map(|identifier: String| match identifier.as_str() {
        "DECLARE" => Token::Declare,
        "OUT" => Token::Out,
        _ => Token::Identifier(identifier)
    });
    let assign = just("<-").to(Token::Assign);
    
    let colon = just(':').to(Token::Colon);
    let data_types = choice::<_, Simple<char>>((
        just("INTEGER").to(Token::DataType(DataTypes::Integer)),
        just("REAL").to(Token::DataType(DataTypes::Real)),
        ));
    let new_line = newline().to(Token::NewLine);
    choice::<_, Simple<char>>((
        new_line,
        number,
        data_types,
        identifiers,
        assign,
        colon,
        )).padded_by(one_of(" \t").ignored().repeated())
        .repeated().then_ignore(end())
}
