use crate::ast::*;
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

macro_rules! function_call {
    ($identifier: expr, $args: expr) => {
        $identifier.clone().then(
            $args
                .separated_by(just(Token::Comma))
                .delimited_by(just(Token::OpenBracket), just(Token::CloseBracket)),
        )
    };
}

pub fn parser() -> impl Parser<Token, Vec<Statement>, Error = Simple<Token>> {
    let identifier = select! {Token::Identifier(name) => name}.labelled("identifier");
    //let data_type = select! {Token::DataType(r#type) => r#type.clone()}.labelled("data type");
    let literal_type =
        select! {Token::DataType(DataTypes::Literal(literal_type)) => literal_type.clone()}
            .labelled("literal type");

    let int = select! {Token::Integer(int) => int.parse().unwrap()};

    let literal = select! {
        Token::Integer(int) => Expression::Value(Literal::Integer(int.parse().unwrap())),
        Token::Real(float) => Expression::Value(Literal::Real(float.parse().unwrap())),
        Token::String(string) => Expression::Value(Literal::String(string)),
        Token::Boolean(boolean) => Expression::Value(Literal::Bool(boolean))
    }
    .labelled("literal");

    let built_in = select! {Token::BuiltIn(name) => name};

    let expression = recursive(|expr| {
        let index_array = identifier
            .clone()
            .then(
                expr.clone()
                    .delimited_by(just(Token::OpenSquare), just(Token::CloseSquare)),
            )
            .map(|(identifier, expression): (String, Expression)| {
                Expression::ArrayIndex(identifier, Box::new(expression))
            });

        let atom = index_array
            .or(literal)
            .or(function_call!(identifier.or(built_in), expr.clone())
                .map(|(name, args)| Expression::FunctionCall(name, args)))
            .or(identifier.clone().map(|name| Expression::Variable(name)))
            .or(expr
                .clone()
                .delimited_by(just(Token::OpenBracket), just(Token::CloseBracket)));

        let unary = just(Token::Operator(Ops::Minus))
            .repeated()
            .then(atom.clone())
            .foldr(|_, right_side| Expression::Negative(Box::new(right_side)))
            .or(just(Token::Operator(Ops::Not))
                .repeated()
                .then(expr)
                .foldr(|_, right_side| Expression::Not(Box::new(right_side))));

        let products = operator!(&PRODUCTS, unary);

        let sums = operator!(&SUMS, products);

        operator!(&COMPARE, sums)
    });

    let newline = |at_least| just(Token::NewLine).repeated().at_least(at_least).ignored();

    let statement = recursive(|stat| {
        let declare = just(Token::Declare)
            .ignore_then(identifier)
            .then_ignore(just(Token::Colon));

        let declare_literal = declare
            .clone()
            .then(literal_type.clone())
            .map(|(identifier, literal_type)| Declare::Literal(identifier, literal_type));

        let declare_array = declare
            .then_ignore(just(Token::DataType(DataTypes::Array)))
            .then(
                int.clone()
                    .separated_by(just(Token::Colon))
                    .exactly(2)
                    .delimited_by(just(Token::OpenSquare), just(Token::CloseSquare)),
            )
            .then_ignore(just(Token::Of))
            .then(literal_type)
            .map(|((identifier, bounds), literal_type)| {
                Declare::Array(
                    identifier,
                    Bounds {
                        lower: *bounds.get(0).unwrap(),
                        upper: *bounds.get(1).unwrap(),
                    },
                    literal_type,
                )
            });

        let declare = declare_array
            .or(declare_literal)
            .map(|declare| Statement::Declare(declare));

        let assign_literal = identifier
            .clone()
            .then_ignore(just(Token::Arrow))
            .then(expression.clone())
            .map(|(identifier, expression)| Assign::Literal(identifier, expression));

        let assign_array = identifier
            .clone()
            .then(
                expression
                    .clone()
                    .delimited_by(just(Token::OpenSquare), just(Token::CloseSquare)),
            )
            .then_ignore(just(Token::Arrow))
            .then(expression.clone())
            .map(|((identifier, index), assign)| Assign::Array(identifier, index, assign));

        let assign = assign_array
            .or(assign_literal)
            .map(|assign| Statement::Assign(assign));

        let out = just(Token::Out)
            .ignore_then(
                expression
                    .clone()
                    .separated_by(just(Token::Comma))
                    .at_least(1),
            )
            .map(|expressions| Statement::Out(expressions));

        let in_ = just(Token::In)
            .ignore_then(identifier)
            .map(|name| Statement::In(name));

        let if_ = just(Token::If)
            .ignore_then(expression.clone())
            .then_ignore(just(Token::NewLine).repeated().ignored())
            .then_ignore(just(Token::Then).then_ignore(newline(0)))
            .then(stat.clone().repeated().at_least(1))
            .then(
                just(Token::Else)
                    .ignore_then(stat.clone().repeated().at_least(1).padded_by(newline(0)))
                    .or_not(),
            )
            .map(|((conditional, if_branch), else_branch)| {
                Statement::If(conditional, if_branch, else_branch)
            })
            .then_ignore(just(Token::EndIf));

        let procedure = function_call!(identifier.clone(), expression.clone())
            .map(|(name, args)| Statement::ProcedureCall(name, args));

        let return_ = just(Token::Return)
            .ignore_then(expression.clone().or_not())
            .map(|expression| Statement::Return(expression));

        let for_ = just(Token::For)
            .ignore_then(identifier)
            .then_ignore(just(Token::Arrow))
            .then(expression.clone())
            .then_ignore(just(Token::To))
            .then(expression.clone())
            .then_ignore(newline(1))
            .then(stat.clone().repeated().at_least(1))
            .map(|(((identifier, start), end), statements)| {
                Statement::For(identifier, start, end, statements)
            })
            .then_ignore(just(Token::EndFor).or(just(Token::Next)))
            .then_ignore(identifier.or_not());

        let while_ = just(Token::While)
            .ignore_then(expression.clone())
            .then_ignore(just(Token::Do))
            .then_ignore(newline(1))
            .then(stat.clone().repeated().at_least(1))
            .map(|(expression, statements)| Statement::While(expression, statements))
            .then_ignore(just(Token::EndWhile));

        let repeat = just(Token::Repeat)
            .ignore_then(newline(1))
            .ignore_then(stat.clone().repeated().at_least(1))
            .then_ignore(just(Token::Until))
            .then(expression.clone())
            .map(|(statements, expression)| Statement::Repeat(statements, expression));

        choice((
            declare, assign, out, in_, if_, procedure, return_, for_, while_, repeat,
        ))
        .then_ignore(newline(1).or(end().rewind()))
    });

    let statements = statement.repeated().then_ignore(end());

    statements
}
