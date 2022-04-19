use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use chumsky::{prelude::*};
use chumsky::text::{newline};
use crate::ExecutionError::IncorrectAssignmentType;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DataTypes {
    Integer,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Token {
    Declare,
    Identifier(String),
    Arrow,
    Integer(u32),
    Out,
    Colon,
    DataType(DataTypes),
    NewLine
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Value {
    Integer(u32)
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(x) => f.write_str(&x.to_string())
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Variable {
    data_type: DataTypes,
    value: Option<Value>
}

impl From<DataTypes> for Variable {
    fn from(data_type: DataTypes) -> Self {
        Variable {
            data_type,
            value: None
        }
    }
}

impl From<&Value> for DataTypes {
    fn from(value: &Value) -> Self {
        match value {
            Value::Integer(_) => DataTypes::Integer
        }
    }
}

fn type_is_value(value: &Value, data_type: DataTypes) -> Result<(), ExecutionError> {
    if data_type == value.into() {
        Ok(())
    }
    else {
        Err(IncorrectAssignmentType(data_type, value.into()))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Statement {
    Declare(String, DataTypes),
    Out(String),
    Assign(String, Value)
}

#[derive(Debug)]
pub enum ExecutionError {
    VariableNotFound(String),
    VariableNotAssigned(String),
    IncorrectAssignmentType(DataTypes, DataTypes)
}

fn indent<E: chumsky::Error<char>>() -> impl Parser<char, (), Error = E> + Copy + Clone{
    choice((
        just(' '),
        just('\t')
    )).repeated().ignored()
}

pub fn lexer() -> impl Parser<char, Vec<Token>, Error = Simple<char>>{
    
    let number = text::digits(10).map(|int: String| Token::Integer(int.parse().unwrap()));
    
    let identifiers = text::ident().map(|identifier: String| match identifier.as_str() {
        "DECLARE" => Token::Declare,
        "OUT" => Token::Out,
        _ => Token::Identifier(identifier)
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
        )).padded_by(indent())
        .repeated().then_ignore(end())
}

pub fn parser() -> impl Parser<Token, Vec<Statement>, Error = Simple<Token>>{
    let identifier = select! {Token::Identifier(name) => name.clone()}
        .labelled("identifier");
    let data_type = select! {Token::DataType(r#type) => r#type.clone()}
        .labelled("data type");
    let value = select! {Token::Integer(int) => Value::Integer(int)};

    let declare = just(Token::Declare)
        .ignore_then(identifier)
        .then_ignore(just(Token::Colon))
        .then(data_type)
        .map(|(identifier, r#type)| Statement::Declare(identifier, r#type));

    let assigned = identifier
        .then_ignore(just(Token::Arrow)).then(value)
        .map(|(identifier, value)| Statement::Assign(identifier, value));

    let out = just(Token::Out)
        .ignore_then(identifier)
        .map(|identifier| Statement::Out(identifier));

    let statements =
        choice((declare, out, assigned))
        .separated_by(just(Token::NewLine).ignored()).then_ignore(end());

    statements
}

pub fn evaluate(statements: Vec<Statement>) -> Result<(), ExecutionError>{
    let mut variables: HashMap<String, Variable> = HashMap::new();

    for statement in statements {
        match statement {
            Statement::Declare(identifier, data_type) => {
                variables.insert(identifier, data_type.into());
            },
            Statement::Assign(identifier, value) => {
                match variables.get_mut(&identifier) {
                    Some(variable) => {
                        match type_is_value(&value, variable.data_type.clone()) {
                            Ok(_) => {
                                variable.value = Some(value);
                            },
                            Err(_) => {}
                        }
                    },
                    None => { return Err(ExecutionError::VariableNotFound(identifier)); }
                }
            },
            Statement::Out(identifier) => {
                match variables.get(&identifier) {
                    Some(variable) => {
                        match &variable.value {
                            None => {return Err(ExecutionError::VariableNotAssigned(identifier));},
                            Some(value) => println!("{}", value)
                        }
                    },
                    None => { return Err(ExecutionError::VariableNotFound(identifier)); }
                }
            },
        }
    }
    Ok(())
}