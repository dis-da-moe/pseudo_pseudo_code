use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DataTypes {
    Literal(LiteralType),
    Array,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LiteralType {
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
    BuiltIn(String),
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
    Next,
    Then,
    Else,
    EndIf,
    Comma,
    Return,
    For,
    EndFor,
    To,
    While,
    Do,
    EndWhile,
    Repeat,
    Until,
    Of,
    OpenSquare,
    CloseSquare,
}

impl Default for Token {
    fn default() -> Self {
        Token::NewLine
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Integer(isize),
    Real(f64),
    String(String),
    Bool(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bounds {
    pub lower: usize,
    pub upper: usize,
}

impl Eq for Literal {}

macro_rules! to_value {
    ($x:tt, $type: ident, $name: expr) => {
        impl From<&Literal> for $type {
            fn from(value: &Literal) -> Self {
                match &*value {
                    Literal::$x(value) => value.clone(),
                    _ => panic!("Can not convert {} to {}", value, $name),
                }
            }
        }
    };
}

to_value!(Real, f64, "float");
to_value!(Integer, isize, "integer");
to_value!(String, String, "string");
to_value!(Bool, bool, "boolean");

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expression {
    Value(Literal),
    Variable(String),
    ArrayIndex(String, Box<Expression>),
    FunctionCall(String, Vec<Expression>),
    Negative(Box<Expression>),
    Operate(Ops, Box<Expression>, Box<Expression>),
    Not(Box<Expression>),
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Integer(value) => f.write_str(&*format!("{}", value)),
            Literal::Real(value) => f.write_str(&*format!("{}", value)),
            Literal::String(value) => f.write_str(&*format!("{}", value)),
            Literal::Bool(value) => f.write_str(&*format!("{}", value)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Variable {
    Literal {
        literal_type: LiteralType,
        value: Option<Literal>,
        is_mutable: bool,
    },
    Array {
        literal_type: LiteralType,
        bounds: Bounds,
        values: Vec<Option<Literal>>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Parameter {
    pub name: String,
    pub literal_type: LiteralType,
}

impl From<&LiteralType> for Variable {
    fn from(literal_type: &LiteralType) -> Self {
        Variable::Literal {
            value: None,
            literal_type: literal_type.clone(),
            is_mutable: true,
        }
    }
}

impl From<&Literal> for LiteralType {
    fn from(value: &Literal) -> Self {
        match value {
            Literal::Integer { .. } => LiteralType::Integer,
            Literal::Real { .. } => LiteralType::Real,
            Literal::String { .. } => LiteralType::String,
            Literal::Bool { .. } => LiteralType::Boolean,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Statement {
    Declare(Declare),
    Out(Vec<Expression>),
    Assign(Assign),
    In(String),
    If(Expression, Vec<Statement>, Option<Vec<Statement>>),
    ProcedureCall(String, Vec<Expression>),
    Return(Option<Expression>),
    For(String, Expression, Expression, Vec<Statement>),
    While(Expression, Vec<Statement>),
    Repeat(Vec<Statement>, Expression),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Declare {
    Literal(String, LiteralType),
    Array(String, Bounds, LiteralType),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Assign {
    Literal(String, Expression),
    Array(String, Expression, Expression),
}

#[derive(Debug, PartialEq)]
pub enum Execution {
    NotFound(String),
    NotAssigned(String),
    IncorrectType(LiteralType, LiteralType),
    BinaryNotSupported(Ops, LiteralType, LiteralType),
    UnaryNotSupported(Ops, LiteralType),
    AlreadyDeclared(String),
    IncorrectNumberArguments(String, usize, usize),
    CanNotCallReturn,
    CanNotParse(String),
    LiteralToArray,
    AssignToConstant(String),
    InvalidBounds(Bounds),
    OutOfBounds(String, usize),
    IndexNotAssigned(String, usize),
    NegativeIndex(String, isize),
    NegativeLoop(isize),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Function {
    BuiltIn(fn(Vec<Literal>) -> Result<Literal, Execution>)
}

pub fn match_literal<'a>(
    a: &'a LiteralType,
    b: &'a LiteralType,
) -> Result<&'a LiteralType, Execution> {
    match a == b {
        true => Ok(a),
        false => Err(Execution::IncorrectType(a.clone(), b.clone())),
    }
}
