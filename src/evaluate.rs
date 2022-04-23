
use crate::tokens::*;
use std::{collections::HashMap, io::stdin};

#[derive(Debug, PartialEq)]
pub enum Execution {
    NotFound(String),
    NotAssigned(String),
    IncorrectType(DataTypes, DataTypes),
    //InvalidOperation(Operators, Expression),
    OperatorNotSupported(Ops, DataTypes),
}

fn match_type(a: DataTypes, b: DataTypes) -> Result<DataTypes, Execution> {
    match a == b {
        true => Ok(a),
        false => Err(Execution::IncorrectType(a, b)),
    }
}

fn evaluate_expression(
    expression: Expression,
    vars: &HashMap<String, Variable>,
) -> Result<Literal, Execution> {
    let eval = |expr: Box<Expression>| evaluate_expression(*expr, vars);

    match expression {
        Expression::Value(value) => Ok(value),
        Expression::Variable(identifier) => vars
            .get(&identifier)
            .ok_or_else(|| Execution::NotFound(identifier.clone()))?
            .value
            .clone()
            .ok_or_else(|| Execution::NotAssigned(identifier.clone())),
        Expression::Negative(expression) => negate(&eval(expression)?),
        Expression::Operate(op, a, b) => operate(op, &eval(a)?, &eval(b)?),
        Expression::Not(expression) => not(&eval(expression)?),
    }
}

fn negate(value: &Literal) -> Result<Literal, Execution> {
    match *value {
        Literal::Integer { value: number } => Ok(Literal::Integer { value: -number }),
        Literal::Real { value: number } => Ok(Literal::Real { value: -number }),
        _ => Err(Execution::OperatorNotSupported(
            Ops::Minus,
            DataTypes::from(value),
        )),
    }
}

fn not(value: &Literal) -> Result<Literal, Execution> {
    match *value {
        Literal::Boolean { value: boolean } => Ok(Literal::Boolean {value: !boolean}),
        _ => Err(Execution::OperatorNotSupported(
            Ops::Not,
            DataTypes::from(value),
        )),    
    }
}

fn int_operations(operation: Ops, a: i32, b: i32) -> Result<Literal, Execution> {
    match operation {
        Ops::Minus => Ok(Literal::Integer { value: a - b }),
        Ops::Plus => Ok(Literal::Integer { value: a + b }),
        Ops::Multiply => Ok(Literal::Integer { value: a * b }),
        Ops::Divide => Ok(Literal::Real {
            value: a as f64 / b as f64,
        }),
        Ops::Div => Ok(Literal::Integer { value: a / b }),
        Ops::Mod => Ok(Literal::Integer { value: a % b }),
        Ops::Equal => Ok(Literal::Boolean {value: a == b}),
        Ops::NotEqual => Ok(Literal::Boolean {value: a != b}),
        Ops::GreaterThanEqual => Ok(Literal::Boolean {value: a >= b}),
        Ops::LessThanEqual => Ok(Literal::Boolean {value: a <= b}),
        Ops::GreaterThan => Ok(Literal::Boolean {value: a > b}),
        Ops::LessThan => Ok(Literal::Boolean {value: a < b}),
        _ => Err(Execution::OperatorNotSupported(
            operation,
            DataTypes::Integer,
        )),
    }
}

fn float_operations(operation: Ops, a: f64, b: f64) -> Result<Literal, Execution> {
    match operation {
        Ops::Minus => Ok(Literal::Real { value: a - b }),
        Ops::Plus => Ok(Literal::Real { value: a + b }),
        Ops::Multiply => Ok(Literal::Real { value: a * b }),
        Ops::Divide => Ok(Literal::Real { value: a / b }),
        Ops::GreaterThan => Ok(Literal::Boolean {value: a > b}),
        Ops::LessThan => Ok(Literal::Boolean {value: a < b}),
        _ => Err(Execution::OperatorNotSupported(operation, DataTypes::Real)),
    }
}

fn string_operations(operation: Ops, a: String, b: String) -> Result<Literal, Execution> {
    match operation {
        Ops::Concatenate => Ok(Literal::String { value: a + &b }),
        Ops::Equal => Ok(Literal::Boolean {value: a == b}),        
        Ops::NotEqual => Ok(Literal::Boolean {value: a != b}),
        _ => Err(Execution::OperatorNotSupported(
            operation,
            DataTypes::String,
        )),
    }
}

fn bool_operations(operation: Ops, a: bool, b: bool) -> Result<Literal, Execution> {
    match operation {
        Ops::And => {
            Ok(Literal::Boolean{value: a && b})
        },
        Ops::Or => {
            Ok(Literal::Boolean{value: a || b})
        },
        Ops::Equal => {
            Ok(Literal::Boolean{value: a == b})
        },
        Ops::NotEqual => {
            Ok(Literal::Boolean{value: a != b})
        },
        _ => Err(Execution::OperatorNotSupported(
            operation,
            DataTypes::Boolean,
        )),
    }
}

fn operate(operation: Ops, a: &Literal, b: &Literal) -> Result<Literal, Execution> {
    let data_type = match_type(a.into(), b.into())?;

    match data_type {
        DataTypes::Integer => {
            let a = i32::from(a);
            let b = i32::from(b);
            int_operations(operation, a, b)
        }
        DataTypes::Real => {
            let a = f64::from(a);
            let b = f64::from(b);
            float_operations(operation, a, b)
        }
        DataTypes::String => {
            let a = String::from(a);
            let b = String::from(b);
            string_operations(operation, a, b)
        }
        DataTypes::Boolean => {
            let a = bool::from(a);
            let b = bool::from(b);
            bool_operations(operation, a, b)
        } //_ => Err(Execution::OperatorNotSupported(operation, data_type)),
    }
}

pub fn evaluate(statements: Vec<Statement>, parent_scope: HashMap<String, Variable>) 
-> Result<HashMap<String, Variable>, Execution> {
    let mut variables: HashMap<String, Variable> = parent_scope;
    let mut local_variables: Vec<String> = vec![];

    for statement in statements {
        match statement {
            Statement::Declare(identifier, data_type) => {
                local_variables.push(identifier.clone());
                variables.insert(identifier, data_type.into());
            }

            Statement::Assign(identifier, expression) => {
                let value = evaluate_expression(expression, &variables)?;

                let variable = variables
                    .get_mut(identifier.as_str())
                    .ok_or_else(|| Execution::NotFound(identifier))?;

                match_type(DataTypes::from(&value), variable.data_type.clone())?;

                variable.value = Some(value);
            }

            Statement::Out(expression) => {
                let value = evaluate_expression(expression, &variables)?;

                /* No type checking until string conversion function is implemented 
                let data_type = DataTypes::from(&value);
                if data_type != DataTypes::String {
                    return Err(Execution::IncorrectType(DataTypes::String, data_type));
                }*/

                println!("{}", value);
            }

            Statement::In(identifier) => {
                let variable = variables
                    .get_mut(identifier.as_str())
                    .ok_or_else(|| Execution::NotFound(identifier))?;
                if variable.data_type != DataTypes::String {
                    return Err(Execution::IncorrectType(
                        DataTypes::String,
                        variable.data_type.clone(),
                    ));
                }
                let mut input = String::new();
                stdin().read_line(&mut input).unwrap();
                variable.value = Some(Literal::String { value: input.trim().to_string() });
            }

            Statement::If(conditional, if_branch, else_branch) => {
                let result = evaluate_expression(conditional, &variables)?;
                let data_type = DataTypes::from(&result);

                if data_type != DataTypes::Boolean {
                    return Err(Execution::IncorrectType(DataTypes::Boolean, data_type));
                }

                let condition = bool::from(&result); 

                if condition {
                    variables = evaluate(if_branch, variables)?;
                }
                else {
                    if let Some(statements) = else_branch {
                        variables = evaluate(statements, variables)?;
                    }
                }
            }
        }
    }

    for variable in local_variables {
        variables.remove(&variable);
    }

    Ok(variables)
}
