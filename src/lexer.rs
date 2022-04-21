use chumsky::{prelude::*};
use chumsky::text::{newline};
use crate::tokens::*;

fn indent<E: chumsky::Error<char>>() -> impl Parser<char, (), Error = E> + Copy + Clone{
    choice((
        just(' '),
        just('\t')
    )).repeated().ignored()
}

pub fn lexer<'a>() -> impl Parser<char, Vec<Token>, Error = Simple<char>>{
    
    let number = text::digits(10)
        .map(|int: String| Token::Integer(int.parse().unwrap()));
    
    let identifiers = text::ident().map(|identifier: String| match identifier.as_str() {
        "DECLARE" => Token::Declare,
        "OUT" => Token::Out,
        _ => Token::Identifier(identifier)
    });

    let operator = one_of(ALL_OPS).map(|char: char| {
        Token::Operator(char.into())
    });
    
    let assign = just("<-").to(Token::Arrow);
    
    let colon = just(':').to(Token::Colon);
    
    let data_types = choice::<_, Simple<char>>((
        just("INTEGER").to(Token::DataType(DataTypes::Integer)),
        ));
    
    let new_line = newline().to(Token::NewLine);
    
    choice::<_, Simple<char>>((
        new_line,
        number,
        data_types,
        identifiers,
        assign,
        colon,
        operator,
        )).padded_by(indent())
        .repeated().then_ignore(end())
}


