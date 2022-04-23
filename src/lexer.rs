use crate::tokens::*;
use chumsky::prelude::*;
use chumsky::text::{newline, whitespace};

fn indent<E: chumsky::Error<char>>() -> impl Parser<char, (), Error = E> + Copy + Clone {
    choice((just(' '), just('\t'))).ignored()
}

macro_rules! just_to {
    ($token: ident, $enum: ident, $($string: expr => $name: ident),+) => {(
        $(just($string).to(Token::$token($enum::$name))),+
    )}
}

pub fn lexer<'a>() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    let number = text::digits(10)
        .then(just('.').ignore_then(text::digits(10)).or_not())
        .map(|(whole, integral): (String, Option<String>)| {
            if let Some(integral) = integral {
                let joined = whole + "." + &integral;
                Token::Real(joined)
            } else {
                Token::Integer(whole)
            }
        });
    
    let boolean = text::keyword("TRUE").to(Token::Boolean(true))
    .or(text::keyword("FALSE").to(Token::Boolean(false)));
    
    let open_bracket = just('(').to(Token::OpenBracket);
    let close_bracket = just(')').to(Token::CloseBracket);

    let identifiers = filter(|char: &char| char.is_alphabetic())
        .then(filter(|char: &char| char.is_alphanumeric()).repeated())
        .map(|(first, second)| {
            let identifier = second.iter().fold(String::from(first), |mut string, next| {
                string.push(*next);
                string
            });

            match identifier.as_str() {
                "DECLARE" => Token::Declare,
                "OUTPUT" => Token::Out,
                "INPUT" => Token::In,
                _ => Token::Identifier(identifier),
            }
        });

    let string = filter(|char: &char| char != &'\n' && char != &'\"')
        .repeated()
        .map(|vec| Token::String(vec.into_iter().collect()))
        .delimited_by(just('\"'), just('\"'));

    let conditionals = choice::<_, Simple<char>>((
        just("IF").to(Token::If),
        just("ENDIF").to(Token::EndIf),
        just("THEN").to(Token::Then),
        just("ELSE").to(Token::Else),
    ));

    let operators = choice::<_, Simple<char>>(just_to! {
    Operator, Ops,
        "+" => Plus,
        "-" => Minus,
        "/" => Divide,
        "*" => Multiply,
        "&" => Concatenate,
        ">=" => GreaterThanEqual,
        "<=" => LessThanEqual,
        "<>" => NotEqual,
        ">" => GreaterThan,
        "<" => LessThan,
        "=" => Equal,
        "MOD" => Mod,
        "DIV" => Div,
        "AND" => And,
        "OR" => Or,
        "NOT" => Not
    });

    let assign = just("<-").to(Token::Arrow);

    let colon = just(':').to(Token::Colon);

    let data_types = choice::<_, Simple<char>>(just_to! {
        DataType, DataTypes,
        "INTEGER" => Integer,
        "REAL" => Real,
        "STRING" => String,
        "BOOLEAN" => Boolean
    });

    let new_line = newline().repeated().at_least(1).ignored();

    let comment = whitespace().ignore_then(
        just("//")
            .ignored()
            .padded_by(indent().repeated())
            .then_ignore(filter(|char: &char| char != &'\n').repeated())
            .then_ignore(whitespace()),
    );

    let tokens = choice::<_, Simple<char>>((
        comment
            .repeated()
            .at_least(1)
            .ignored()
            .or(new_line)
            .to(Token::NewLine),
        number,
        string,
        data_types,
        boolean,
        conditionals,
        assign,
        operators,
        identifiers,
        colon,
        open_bracket,
        close_bracket,
    ));
    let ignored = indent().or(comment.ignored()).repeated().ignored();

    (ignored
        .ignore_then(tokens.repeated().at_least(1))
        .then_ignore(ignored))
    .repeated()
    .then_ignore(end())
    .map(|tokens| tokens.into_iter().flatten().collect())
}