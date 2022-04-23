use crate::tokens::*;
use chumsky::prelude::*;

macro_rules! operator {
    ($allowed: expr, $base: expr) => {
        $base
            .clone()
            .then(
                select! {Token::Operator(op) if $allowed.contains(&op) => op}
                    .map(|op| |x, y| Expression::Operate(op, x, y))
                    .then($base)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)))
    };
}

pub fn parser() -> impl Parser<Token, Vec<Statement>, Error = Simple<Token>> {
    let identifier = select! {Token::Identifier(name) => name}.labelled("identifier");
    let data_type = select! {Token::DataType(r#type) => r#type.clone()}.labelled("data type");

    let literal = select! {
        Token::Integer(int) => Expression::Value(Literal::Integer { value: int.parse().unwrap() }),
        Token::Real(float) => Expression::Value(Literal::Real { value: float.parse().unwrap() }),
        Token::String(string) => Expression::Value(Literal::String {value: string}),
        Token::Boolean(boolean) => Expression::Value(Literal::Boolean {value: boolean})
    }
    .labelled("literal");

    let expression = recursive(|expr| {
        let atom = literal
            .or(identifier.map(|name| Expression::Variable(name)))
            .or(expr.clone().delimited_by(just(Token::OpenBracket), just(Token::CloseBracket)));

        let unary = just(Token::Operator(Ops::Minus))
            .repeated()
            .then(atom.clone())
            .foldr(|_, right_side| Expression::Negative(Box::new(right_side)))
            .or(
                just(Token::Operator(Ops::Not)).repeated()
                .then(expr)
                .foldr(|_, right_side| Expression::Not(Box::new(right_side)))
            );

        let products = operator!(&PRODUCTS, unary);

        let sums = operator!(&SUMS, products);

        operator!(&COMPARE, sums)
    });

    let newline  = |at_least| just(Token::NewLine).repeated().at_least(at_least).ignored();

    let statement = recursive(|stat| {
        let declare = just(Token::Declare)
        .ignore_then(identifier)
        .then_ignore(just(Token::Colon))
        .then(data_type)
        .map(|(identifier, r#type)| Statement::Declare(identifier, r#type));

        let assigned = identifier
            .then_ignore(just(Token::Arrow))
            .then(expression.clone())
            .map(|(identifier, expression)| Statement::Assign(identifier, expression));

        let out = just(Token::Out)
            .ignore_then(expression.clone())
            .map(|expression| Statement::Out(expression));

        let r#in = just(Token::In)
            .ignore_then(identifier)
            .map(|name| Statement::In(name));
        
        let r#if = just(Token::If)
        .ignore_then(expression)
        .then_ignore(just(Token::NewLine).repeated().ignored())
        .then_ignore(just(Token::Then).then_ignore(newline(0)))
        .then(stat.clone().repeated().at_least(1)).then(
            just(Token::Else)
            .ignore_then(stat.repeated().at_least(1).padded_by(newline(0)))
            .or_not()
        ).map(
            |((conditional, if_branch), else_branch)| 
            Statement::If(conditional, if_branch, else_branch)
        ).then_ignore(just(Token::EndIf));

        choice((declare, assigned, out, r#in, r#if))
        .then_ignore(newline(1).or(end().rewind()))
    });

    let statements = statement.repeated()
    .then_ignore(end());

    statements
}
