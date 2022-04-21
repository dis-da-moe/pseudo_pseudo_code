use crate::tokens::*;
use chumsky::{prelude::*};

pub fn parser() -> impl Parser<Token, Vec<Statement>, Error = Simple<Token>>{
    let identifier = select! {Token::Identifier(name) => name}
        .labelled("identifier");
    let data_type = select! {Token::DataType(r#type) => r#type.clone()}
        .labelled("data type");
    let int = select! {Token::Integer(int) => Expression::Value(Value::Integer(int))};
    
    let expression = recursive(|expr| {
        let atom = int
            .or(identifier.map(|name| Expression::Variable(name)));
        
        let unary = just(Token::Operator(Operators::Minus))
            .repeated().then(atom)
            .foldr(|_, right_side| Expression::Negative(Box::new(right_side)));
            
        let op = select! {Token::Operator(op) => op};
        
        let product = unary.clone()
            .then(
                op
                    .map(|op| {|x, y| Expression::Operate(op, x, y)})
                    .then(unary).repeated()
            ).foldl(|lhs, (op, rhs)| op(lhs, rhs));
            
        );
        
        atom
    });
    
    let declare = just(Token::Declare)
        .ignore_then(identifier)
        .then_ignore(just(Token::Colon))
        .then(data_type)
        .map(|(identifier, r#type)| Statement::Declare(identifier, r#type));

    let assigned = identifier
        .then_ignore(just(Token::Arrow)).then(expression)
        .map(|(identifier, expression)| Statement::Assign(identifier, expression));

    let out = just(Token::Out)
        .ignore_then(identifier)
        .map(|identifier| Statement::Out(identifier));
    
    
    
    let statements =
        choice((declare, out, assigned))
            .separated_by(just(Token::NewLine).ignored()).then_ignore(end());

    statements
}
