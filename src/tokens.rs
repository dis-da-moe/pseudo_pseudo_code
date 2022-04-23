use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DataTypes {
    Integer,
    Real,
    String,
    Boolean,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Ops {
    Plus,
    Minus,
    Divide,
    Multiply,
    Concatenate,
    GreaterThan,
    LessThan,
    GreaterThanEqual,
    LessThanEqual,
    Equal,
    NotEqual,
    Mod,
    Div,
    And,
    Or,
    Not,
}

pub const PRODUCTS: [Ops; 4] = [Ops::Multiply, Ops::Divide, Ops::Div, Ops::Mod];

pub const COMPARE: [Ops; 9] = [
    Ops::GreaterThan,
    Ops::LessThan,
    Ops::GreaterThanEqual,
    Ops::LessThanEqual,
    Ops::Equal,
    Ops::NotEqual,
    Ops::And,
    Ops::Or,
    Ops::Not,
];

pub const SUMS: [Ops; 3] = [Ops::Minus, Ops::Plus, Ops::Concatenate];

impl From<&str> for Ops {
    fn from(string: &str) -> Self {
        match string {
            "+" => Ops::Plus,
            "-" => Ops::Minus,
            "/" => Ops::Divide,
            "*" => Ops::Multiply,
            "&" => Ops::Concatenate,
            ">" => Ops::GreaterThan,
            "<" => Ops::LessThan,
            ">=" => Ops::GreaterThanEqual,
            "<=" => Ops::LessThanEqual,
            "=" => Ops::Equal,
            "<>" => Ops::NotEqual,
            _ => panic!("Forgot to match an operator"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Token {
    Declare,
    Identifier(String),
    Arrow,
    Integer(String),
    Real(String),
    Out,
    In,
    Colon,
    DataType(DataTypes),
    NewLine,
    String(String),
    Operator(Ops),
    OpenBracket,
    CloseBracket,
    Boolean(bool),
    If,
    Then,
    Else,
    EndIf,
    Comma
}

impl Default for Token {
    fn default() -> Self {
        Token::NewLine
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Integer { value: i32 },
    Real { value: f64 },
    String { value: String },
    Boolean { value: bool },
}

impl Eq for Literal {}

macro_rules! to_value {
    ($x:tt, $type: ident, $name: expr) => {
        impl From<&Literal> for $type {
            fn from(value: &Literal) -> Self {
                match &*value {
                    Literal::$x { value } => value.clone(),
                    _ => panic!("Can not convert {} to {}", value, $name),
                }
            }
        }
    };
}

to_value!(Real, f64, "float");
to_value!(Integer, i32, "integer");
to_value!(String, String, "string");
to_value!(Boolean, bool, "boolean");

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expression {
    Value(Literal),
    Variable(String),
    Negative(Box<Expression>),
    Operate(Ops, Box<Expression>, Box<Expression>),
    Not(Box<Expression>)
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Integer { value: number } => f.write_str(&*format!("{}", number)),
            Literal::Real { value: number } => f.write_str(&*format!("{}", number)),
            Literal::String { value: string } => f.write_str(&*format!("{}", string)),
            Literal::Boolean { value: boolean } => f.write_str(&*format!("{}", boolean)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variable {
    pub data_type: DataTypes,
    pub value: Option<Literal>,
}

impl From<DataTypes> for Variable {
    fn from(data_type: DataTypes) -> Self {
        Variable {
            value: None,
            data_type,
        }
    }
}

impl From<&Literal> for DataTypes {
    fn from(value: &Literal) -> Self {
        match value {
            Literal::Integer { .. } => DataTypes::Integer,
            Literal::Real { .. } => DataTypes::Real,
            Literal::String { .. } => DataTypes::String,
            Literal::Boolean { .. } => DataTypes::Boolean,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Statement {
    Declare(String, DataTypes),
    Out(Vec<Expression>),
    Assign(String, Expression),
    In(String),
    If(Expression, Vec<Statement>, Option<Vec<Statement>>)
}
