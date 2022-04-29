use crate::ast::*;
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

pub fn lexer<'a>() -> impl Parser<char, Vec<Spanned<Token>>, Error = Simple<char>> {
    let number = text::digits(10)
        .then(just('.').ignore_then(text::digits(10)).or_not())
        .map(|(whole, integral): (String, Option<String>)| {
            if let Some(integral) = integral {
                let joined = whole + "." + &integral;
                Token::Real(joined)
            } else {
                Token::Integer(whole)
            }
        })
        .boxed()
        .labelled("number");

    let boolean = text::keyword("TRUE")
        .to(Token::Boolean(true))
        .or(text::keyword("FALSE").to(Token::Boolean(false)))
        .labelled("boolean value");

    let open_bracket = just('(').to(Token::OpenBracket);
    let close_bracket = just(')').to(Token::CloseBracket);

    let open_square = just('[').to(Token::OpenSquare);
    let close_square = just(']').to(Token::CloseSquare);

    let identifiers = text::ident().map(|name: String| {
        use Token::*;
        match name.as_str() {
            "DECLARE" => Declare,
            "OUTPUT" => Out,
            "INPUT" => In,
            "RETURN" => Return,
            "IF" => If,
            "ENDIF" => EndIf,
            "THEN" => Then,
            "ELSE" => Else,
            "FOR" => For,
            "ENDFOR" => EndFor,
            "TO" => To,
            "WHILE" => While,
            "ENDWHILE" => EndWhile,
            "DO" => Do,
            "REPEAT" => Repeat,
            "UNTIL" => Until,
            "OF" => Of,
            "NEXT" => Next,
            "INTEGER" => DataType(DataTypes::Literal(LiteralType::Integer)),
            "REAL" => DataType(DataTypes::Literal(LiteralType::Real)),
            "STRING" => DataType(DataTypes::Literal(LiteralType::String)),
            "BOOLEAN" => DataType(DataTypes::Literal(LiteralType::Boolean)),
            "ARRAY" => DataType(DataTypes::Array),
            _ => {
                let chars: Vec<char> = name.chars().collect();
                if chars.first().unwrap().is_alphabetic()
                    && chars.iter().all(|char| char.is_alphanumeric())
                {
                    Token::Identifier(name)
                } else {
                    Token::BuiltIn(name)
                }
            }
        }
    });

    let string = filter(|char: &char| char != &'\n' && char != &'\"')
        .repeated()
        .map(|vec| Token::String(vec.into_iter().collect()))
        .delimited_by(just('\"'), just('\"'))
        .labelled("string literal");

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
    })
    .labelled("operator")
    .boxed();

    let assign = just("<-").or(just("←")).to(Token::Arrow).labelled("arrow");

    let colon = just(':').to(Token::Colon).labelled("colon");

    let new_line = newline()
        .repeated()
        .at_least(1)
        .ignored()
        .labelled("new line");

    let comment = just("//").then(take_until(just('\n'))).padded_by(whitespace())
        .labelled("comment")
        .boxed();

    let comma = just(',').to(Token::Comma);

    let tokens = choice::<_, Simple<char>>((
        comment
            .clone()
            .repeated()
            .at_least(1)
            .ignored()
            .or(new_line)
            .to(Token::NewLine)
            .labelled("comment or new line")
            .boxed(),
        number,
        string,
        boolean,
        assign,
        operators,
        identifiers,
        colon,
        open_bracket,
        close_bracket,
        open_square,
        close_square,
        comma,
    ))
    .map_with_span(|token, span| (token, span))
    .boxed();

    let ignored = indent().or(comment.ignored()).repeated().ignored();

    (tokens.repeated().at_least(1).padded_by(ignored))
        .repeated()
        .then_ignore(end())
        .map(|tokens| tokens.into_iter().flatten().collect())
}
