use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DataTypes {
    Integer,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Operators {
    Plus,
    Minus,
    Divide,
    Multiply
}

pub const ALL_OPS: &'static str = "+-/*";

pub const PRODUCTS: [Operators; 2] = [Operators::Plus, Operators::Minus];

pub const SUMS: [Operators; 2] = [Operators::Minus, Operators::Plus];

impl From<char> for Operators {
    fn from(char: char) -> Self {
        match char {
            '+' => Operators::Plus,
            '-' => Operators::Minus,
            '/' => Operators::Divide,
            '*' => Operators::Multiply,
            _ => panic!("Forgot to match an operator"),
        }    
    }

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
    NewLine,
    Operator(Operators)
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Value {
    Integer(u32)
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Expression {
    Value(Value),
    Variable(String),
    Negative(Box<Expression>),
    Operate(Operators, Box<Expression>, Box<Expression>),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(x) => f.write_str(&x.to_string())
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Variable {
    pub data_type: DataTypes,
    pub value: Option<Value>
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Statement {
    Declare(String, DataTypes),
    Out(String),
    Assign(String, Expression)
}