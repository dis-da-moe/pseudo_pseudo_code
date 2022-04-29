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
    Any,
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
pub type Span = std::ops::Range<usize>;
pub type Spanned<T> = (T, Span);

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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
    ArrayIndex(String, Box<Spanned<Expression>>),
    FunctionCall(String, Vec<Spanned<Expression>>),
    Negative(Box<Spanned<Expression>>),
    Operate(Ops, Box<Spanned<Expression>>, Box<Spanned<Expression>>),
    Not(Box<Spanned<Expression>>),
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
    Out(Vec<Spanned<Expression>>),
    Assign(Assign),
    In(String),
    If(
        Spanned<Expression>,
        Vec<Spanned<Statement>>,
        Option<Vec<Spanned<Statement>>>,
    ),
    ProcedureCall(String, Vec<Spanned<Expression>>),
    Return(Option<Spanned<Expression>>),
    For(
        String,
        Spanned<Expression>,
        Spanned<Expression>,
        Vec<Spanned<Statement>>,
    ),
    While(Spanned<Expression>, Vec<Spanned<Statement>>),
    Repeat(Vec<Spanned<Statement>>, Spanned<Expression>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Declare {
    Literal(String, LiteralType),
    Array(String, Bounds, LiteralType),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Assign {
    Literal(String, Spanned<Expression>),
    Array(String, Spanned<Expression>, Spanned<Expression>),
}

#[derive(Debug, PartialEq, Hash, Eq)]
pub enum Execution {
    NotFound(String),
    NotAssigned(String),
    IncorrectType(DataTypes, DataTypes),
    BinaryNotSupported(Ops, DataTypes, DataTypes),
    UnaryNotSupported(Ops, DataTypes),
    AlreadyDeclared(String),
    IncorrectNumberArguments(String, usize, usize),
    CanNotCallReturn,
    CanNotParse(String),
    AssignToConstant(String),
    InvalidBounds(Bounds),
    OutOfBounds(String, usize),
    NegativeIndex(String, isize),
    IndexNotAssigned(String, usize),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Function {
    BuiltIn(fn(Vec<Literal>) -> Result<Literal, Execution>),
}

impl From<&LiteralType> for DataTypes {
    fn from(literal_type: &LiteralType) -> Self {
        DataTypes::Literal(literal_type.clone())
    }
}

impl From<LiteralType> for DataTypes {
    fn from(literal_type: LiteralType) -> Self {
        DataTypes::Literal(literal_type)
    }
}

impl Display for LiteralType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use LiteralType::*;
        f.write_str(match self{
            Integer => "integer",
            Real => "real",
            String => "string",
            Boolean => "boolean",
            Any => "any"
        })
    }
}

impl Display for DataTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DataTypes::Literal(literal_type) => f.write_str(&format!("{} literal", literal_type)),
            DataTypes::Array => f.write_str("array"),
        }
    }
}

impl Display for Ops{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self{
            Ops::Plus => "plus",
            Ops::Minus => "minues",
            Ops::Divide => "divide",
            Ops::Multiply => "multiply",
            Ops::Concatenate => "concatenate",
            Ops::GreaterThan => "greater than",
            Ops::LessThan => "less than",
            Ops::GreaterThanEqual => "greater than or equal",
            Ops::LessThanEqual => "less than or equal",
            Ops::Equal => "equal",
            Ops::NotEqual => "not equal",
            Ops::Mod => "modulo",
            Ops::Div => "integer divide",
            Ops::And => "and",
            Ops::Or => "or",
            Ops::Not => "not",
        })
    }
}

impl Display for Bounds{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("lower: {}, upper: {}", self.lower, self.upper))
    }
}

impl Display for Execution{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Execution::*;

        let message = match self {
            NotFound(identifier) => format!("variable {} not found", identifier),
            NotAssigned(identifier) => format!("variable {} not assigned", identifier),
            IncorrectType(expected, received) => format!("Incorrect type, expected {} but received {}", expected, received),
            BinaryNotSupported(op, a, b) => format!("Binary operator \"{}\" is not supported between types {} and {}", op, a, b),
            UnaryNotSupported(op, a) => format!("Unary operator \"{}\" is not supported on type {}", op, a),
            AlreadyDeclared(identifier) => format!("Variable {} is already declared", identifier),
            IncorrectNumberArguments(identifier, expected, received) => 
            format!("Incorrect numer of arguments for \"{}\", expected {} but received {}", identifier, expected, received),
            CanNotCallReturn => format!("Can not call return outside of a function or procedure"),
            CanNotParse(string) => format!("Can not parse string {} as number", string),
            AssignToConstant(identifier) => format!("Can not assign value to constant \"{}\"", identifier),
            InvalidBounds(bounds) => format!("Invalid bounds {}", bounds),
            OutOfBounds(identifier, index) => format!("Index {} is out of bounds for array {}", identifier, index),
            IndexNotAssigned(identifier, index) => format!("Index {} not assigned for array {}", identifier, index),
            NegativeIndex(identifier, index) => format!("Index {} is out of bounds for array {}", identifier, index),
        };
        write!(f, "{}", message)
    }
}

pub fn match_literal<'a>(
    a: &'a LiteralType,
    b: &'a LiteralType,
) -> Result<&'a LiteralType, Execution> {
    match a == b {
        true => Ok(a),
        false => Err(Execution::IncorrectType(a.into(), b.into())),
    }
}
