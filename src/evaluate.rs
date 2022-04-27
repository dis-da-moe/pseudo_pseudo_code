use crate::ast::*;
use std::{
    borrow::BorrowMut,
    collections::HashMap,
    io::stdin,
    ops::{Add, Div, Mul, Rem, Sub},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct State {
    pub functions: HashMap<String, Function>,
    pub scopes: Vec<Scope>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Scope {
    pub variables: HashMap<String, Variable>,
}

fn get_variable<'current>(
    state: &'current State,
    identifier: &str,
) -> Result<&'current Variable, Execution> {
    for scope in &state.scopes {
        if let Some(var) = scope.variables.get(identifier) {
            return Ok(var);
        }
    }

    Err(Execution::NotFound(String::from(identifier)))
}

fn get_literal<'current>(
    state: &'current State,
    identifier: &str,
) -> Result<&'current Literal, Execution> {
    for scope in &state.scopes {
        if let Some(var) = scope.variables.get(identifier) {
            match var {
                Variable::Literal { value, .. } => {
                    if let Some(value) = value {
                        return Ok(value);
                    } else {
                        return Err(Execution::NotAssigned(String::from(identifier)));
                    }
                }
                _ => return Err(Execution::LiteralToArray),
            }
        }
    }

    Err(Execution::NotFound(String::from(identifier)))
}

fn index_array<'current>(
    state: &'current State,
    identifier: &str,
    index: usize,
) -> Result<&'current Literal, Execution> {
    let variable = get_variable(state, identifier)?;
    match variable {
        Variable::Literal { .. } => {
            return Err(Execution::LiteralToArray);
        }
        Variable::Array {
            literal_type: _,
            bounds,
            values,
        } => {
            if index < bounds.lower || index > bounds.upper {
                return Err(Execution::OutOfBounds(String::from(identifier), index));
            }
            let vec_index = index - bounds.lower;
            if let Some(literal) = values.get(vec_index) {
                literal
                    .as_ref()
                    .ok_or_else(|| Execution::IndexNotAssigned(String::from(identifier), index))
            } else {
                Err(Execution::OutOfBounds(String::from(identifier), index))
            }
        }
    }
}

fn get_mut_variable<'state>(
    state: &'state mut State,
    identifier: &str,
) -> Result<&'state mut Variable, Execution> {
    for scope in state.scopes.iter_mut() {
        if let Some(var) = scope.variables.get_mut(identifier) {
            return Ok(var);
        }
    }

    return Err(Execution::NotFound(String::from(identifier)));
}

fn assign_array(
    mut state: State,
    identifier: &str,
    index: usize,
    to_assign: Literal,
) -> Result<State, Execution> {
    let variable = get_mut_variable(&mut state, identifier)?;

    let (array_type, bounds, values) = match variable {
        Variable::Array {
            literal_type,
            bounds,
            values,
        } => (literal_type, bounds, values),
        Variable::Literal { .. } => return Err(Execution::LiteralToArray),
    };

    let assign_type = LiteralType::from(&to_assign);

    if &assign_type != array_type {
        return Err(Execution::IncorrectType(array_type.clone(), assign_type));
    }

    if index < bounds.lower || index > bounds.upper {
        return Err(Execution::OutOfBounds(String::from(identifier), index));
    }
    values.remove(index - bounds.lower);
    values.insert(index - bounds.lower, Some(to_assign));

    Ok(state)
}

fn assign_literal(
    mut state: State,
    identifier: &str,
    literal: Literal,
) -> Result<State, Execution> {
    let variable = get_mut_variable(&mut state, identifier)?;

    match variable {
        Variable::Literal {
            literal_type,
            value,
            is_mutable,
        } => {
            if !*is_mutable {
                return Err(Execution::AssignToConstant(String::from(identifier)));
            }
            match_literal(&LiteralType::from(&literal), &literal_type.clone())?;
            *value = Some(literal);
            return Ok(state);
        }
        Variable::Array { .. } => return Err(Execution::LiteralToArray),
    }
}

macro_rules! eval {
    ($expression: expr, $context: expr) => {
        evaluate_expression($expression, $context)
    };
}

fn evaluate_expression(expression: &Expression, state: &State) -> Result<Literal, Execution> {
    match expression {
        Expression::Value(value) => Ok(value.clone()),
        Expression::Variable(identifier) => {
            get_literal(state, &identifier).map(|literal| literal.clone())
        }
        Expression::Negative(expression) => negate(&eval!(expression, state)?),
        Expression::Operate(op, a, b) => operate(op, &eval!(a, state)?, &eval!(b, state)?),
        Expression::Not(expression) => not(&eval!(expression, state)?),
        Expression::FunctionCall(name, args) => {
            
            if name == "LEN" {
                let identifier = match args.get(0) {
                    Some(Expression::Variable(identifier)) => identifier,
                    Some(_) => return Err(Execution::LiteralToArray),
                    None => return Err(Execution::IncorrectNumberArguments(String::from("LEN"), 1, 0))
                };
                let array = get_variable(state, identifier)?;
                match array {
                    Variable::Array { bounds, ..} => return Ok(Literal::Integer((bounds.upper - bounds.lower + 1).try_into().unwrap())),
                    Variable::Literal { .. } => return Err(Execution::LiteralToArray)
                }
            }
            
            let function = state
                .functions
                .get(name)
                .ok_or_else(|| Execution::NotFound(name.clone()))?;
            
            let call = match function {
                Function::BuiltIn(call) => call
            };

            let values: Vec<Literal> = args
                .iter()
                .map(|arg| eval!(arg, state))
                .collect::<Result<_, _>>()?;

            return call(values);
        }
        Expression::ArrayIndex(identifier, expression) => {
            let index = evaluate_expression(&*expression, state)?;
            let index = match index {
                Literal::Integer(value) => value,
                _ => {
                    return Err(Execution::IncorrectType(
                        LiteralType::Integer,
                        LiteralType::from(&index),
                    ))
                }
            };
            let index: usize = index
                .try_into()
                .map_err(|_| Execution::NegativeIndex(String::from(identifier), index))?;

            index_array(state, identifier, index).map(|literal| literal.clone())
        }
    }
}

fn negate(value: &Literal) -> Result<Literal, Execution> {
    match *value {
        Literal::Integer(value) => Ok(Literal::Integer(-value)),
        Literal::Real(value) => Ok(Literal::Real(-value)),
        _ => Err(Execution::UnaryNotSupported(
            Ops::Minus,
            LiteralType::from(value),
        )),
    }
}

fn not(value: &Literal) -> Result<Literal, Execution> {
    match *value {
        Literal::Bool(value) => Ok(Literal::Bool(!value)),
        _ => Err(Execution::UnaryNotSupported(
            Ops::Not,
            LiteralType::from(value),
        )),
    }
}

macro_rules! bool {
    ($value: expr) => {
        Ok(Literal::Bool($value))
    };
}

macro_rules! ops {
    ($left: expr, $right: expr, $not_found: ident, $($op: ident, $enum_a: ident, $enum_b: ident =>  $(($type: tt))? $enum_result: ident),+) => {
        match ($left, $right) {
            $((Literal::$enum_a(a), Literal::$enum_b(b)) => {
                Ok(Literal::$enum_result((a.clone() $(as $type)?).$op(b.clone() $(as $type)?)))
            },)+
            _ => Err($not_found())
        }
    };
    ($left: expr, $right: expr, $not_found: ident, $($enum_a: ident $op: tt  $enum_b: ident),+) => {
        match ($left, $right) {
            $((Literal::$enum_a(a), Literal::$enum_b(b)) => {
                bool!(*a $op *b)
            },)+
            _ => Err($not_found())
        }
    };
}

fn operate(operation: &Ops, a: &Literal, b: &Literal) -> Result<Literal, Execution> {
    let not_found = || {
        Execution::BinaryNotSupported(
            operation.clone(),
            LiteralType::from(a),
            LiteralType::from(b),
        )
    };
    use Literal::*;
    match operation {
        Ops::Plus => ops!(
            a, b, not_found,
            add, Integer, Integer => Integer,
            add, Real, Real => Real
        ),
        Ops::Minus => ops!(
            a, b, not_found,
            sub, Integer, Integer => Integer,
            sub, Real, Real => Real
        ),
        Ops::Divide => ops!(
            a, b, not_found,
            div, Integer, Integer => (f64) Real,
            div, Real, Real => Real
        ),
        Ops::Multiply => ops!(
            a, b, not_found,
            mul, Integer, Integer => Integer,
            mul, Real, Real => Real
        ),
        Ops::Mod => ops!(
            a, b, not_found,
            rem, Integer, Integer => Integer
        ),
        Ops::Div => ops!(
            a, b, not_found,
            div, Integer, Integer => Integer,
            div, Real, Real => (isize) Integer
        ),
        Ops::GreaterThan => ops!(
            a,
            b,
            not_found,
            Integer > Integer,
            Bool > Bool,
            String > String
        ),
        Ops::LessThan => ops!(
            a,
            b,
            not_found,
            Integer < Integer,
            Bool < Bool,
            String < String
        ),
        Ops::Equal => ops!(
            a,
            b,
            not_found,
            Integer == Integer,
            Bool == Bool,
            String == String
        ),
        Ops::And => ops!(a, b, not_found, Bool && Bool),
        Ops::Or => ops!(a, b, not_found, Bool || Bool),
        Ops::NotEqual => ops!(
            a,
            b,
            not_found,
            Integer != Integer,
            Bool != Bool,
            String != String
        ),
        Ops::GreaterThanEqual => ops!(a, b, not_found, Integer >= Integer),
        Ops::LessThanEqual => ops!(a, b, not_found, Integer <= Integer),
        Ops::Concatenate => match (a, b) {
            (String(a), String(b)) => Ok(String(a.to_owned() + b)),
            _ => Err(not_found()),
        },
        Ops::Not => Err(not_found()),
    }
}

pub fn evaluate<'a>(
    statements: &Vec<Statement>,
    mut state: State,
    as_function: bool,
) -> Result<State, Execution> {
    state.scopes.push(Scope {
        variables: HashMap::new(),
    });

    for statement in statements {
        match statement {
            Statement::Declare(Declare::Literal(identifier, literal_type)) => {
                if get_variable(&state, identifier.as_str()).is_ok() {
                    return Err(Execution::AlreadyDeclared(String::from(identifier)));
                }
                let variables = state.scopes.last_mut().unwrap().variables.borrow_mut();

                variables.insert(String::from(identifier), literal_type.into());
            }

            Statement::Declare(Declare::Array(identifier, bounds, literal_type)) => {
                if get_variable(&state, identifier.as_str()).is_ok() {
                    return Err(Execution::AlreadyDeclared(String::from(identifier)));
                }
                if bounds.lower >= bounds.upper {
                    return Err(Execution::InvalidBounds(bounds.clone()));
                }

                let variables = state.scopes.last_mut().unwrap().variables.borrow_mut();

                let array = Variable::Array {
                    literal_type: literal_type.clone(),
                    bounds: bounds.clone(),
                    values: vec![None; (bounds.upper - bounds.lower) + 1],
                };

                variables.insert(String::from(identifier), array);
            }

            Statement::Assign(Assign::Literal(identifier, expression)) => {
                let value = evaluate_expression(expression, &state)?;

                state = assign_literal(state, identifier.as_str(), value)?;
            }

            Statement::Assign(Assign::Array(identifier, index, expression)) => {
                let index = evaluate_expression(index, &state)?;
                let index = match index {
                    Literal::Integer(value) => value,
                    _ => {
                        return Err(Execution::IncorrectType(
                            LiteralType::Integer,
                            LiteralType::from(&index),
                        ))
                    }
                };
                let index: usize = index
                    .try_into()
                    .map_err(|_| Execution::NegativeIndex(String::from(identifier), index))?;

                let to_assign = evaluate_expression(expression, &state)?;

                state = assign_array(state, identifier, index, to_assign)?;
            }

            Statement::Out(expressions) => {
                let values: Vec<Literal> = expressions
                    .iter()
                    .map(|expression| evaluate_expression(expression, &state))
                    .collect::<Result<Vec<Literal>, Execution>>()?;

                /* No type checking until string conversion function is implemented
                let data_type = DataTypes::from(&value);
                if data_type != DataTypes::String {
                    return Err(Execution::IncorrectType(DataTypes::String, data_type));
                }*/

                println!(
                    "{}",
                    values
                        .iter()
                        .map(|literal| format!("{}", literal))
                        .collect::<Vec<_>>()
                        .join("")
                );
            }

            Statement::In(identifier) => {
                let variable = get_variable(&state, identifier.as_str())?;

                let literal_type = match variable {
                    Variable::Literal { literal_type, .. } => literal_type,
                    _ => return Err(Execution::LiteralToArray),
                };

                if literal_type != &LiteralType::String {
                    return Err(Execution::IncorrectType(
                        LiteralType::String,
                        literal_type.clone(),
                    ));
                }

                let mut input = String::new();
                stdin().read_line(&mut input).unwrap();
                state = assign_literal(
                    state,
                    &identifier,
                    Literal::String(input.trim().to_string()),
                )?;
            }

            Statement::If(conditional, if_branch, else_branch) => {
                let result = evaluate_expression(conditional, &state)?;
                let data_type = LiteralType::from(&result);

                if data_type != LiteralType::Boolean {
                    return Err(Execution::IncorrectType(LiteralType::Boolean, data_type));
                }

                let condition = bool::from(&result);

                if condition {
                    state = evaluate(if_branch, state, false)?;
                } else {
                    if let Some(statements) = else_branch {
                        state = evaluate(statements, state, false)?;
                    }
                }
            }

            Statement::Return(return_value) => {
                if !as_function {
                    return Err(Execution::CanNotCallReturn);
                }

                if let Some(expression) = return_value {
                    let value = evaluate_expression(expression, &state)?;
                    let mut scope = state.scopes.pop().unwrap();
                    scope.variables.insert(
                        String::from("RETURN"),
                        Variable::Literal {
                            literal_type: LiteralType::from(&value),
                            value: Some(value),
                            is_mutable: true,
                        },
                    );
                }

                return Ok(state);
            }

            Statement::For(identifier, start, end, statements) => {
                let iterator = get_variable(&state, identifier)?;
                match iterator {
                    Variable::Literal {
                        literal_type,
                        is_mutable,
                        ..
                    } => {
                        if literal_type != &LiteralType::Integer {
                            return Err(Execution::IncorrectType(
                                LiteralType::Integer,
                                literal_type.clone(),
                            ));
                        }
                        if !*is_mutable {
                            return Err(Execution::AssignToConstant(String::from(identifier)));
                        }
                    }
                    Variable::Array { .. } => {
                        return Err(Execution::LiteralToArray);
                    }
                }

                let start_literal = evaluate_expression(start, &state)?;
                let start = match start_literal {
                    Literal::Integer(value) => value,
                    _ => {
                        return Err(Execution::IncorrectType(
                            LiteralType::Integer,
                            LiteralType::from(&start_literal),
                        ))
                    }
                };
                if start < 0 {
                    return Err(Execution::NegativeLoop(start));
                }
                let end_literal = evaluate_expression(end, &state)?;
                let end = match end_literal {
                    Literal::Integer(value) => value,
                    _ => {
                        return Err(Execution::IncorrectType(
                            LiteralType::Integer,
                            LiteralType::from(&end_literal),
                        ))
                    }
                };
                if end < 0 {
                    return Err(Execution::NegativeLoop(end));
                }

                if start <= end {
                    let range = start..=end;
                    for n in range {
                        state = assign_literal(state, identifier, Literal::Integer(n))?;
                        state = evaluate(statements, state, as_function)?;
                    }
                }
            }

            Statement::While(expression, statements) => {
                loop {
                    let condition = evaluate_expression(expression, &state)?;
                    let continue_loop = match condition {
                        Literal::Bool(value) => value,
                        _ => {
                            return Err(Execution::IncorrectType(
                                LiteralType::Boolean,
                                LiteralType::from(&condition),
                            ))
                        }
                    };
                    if continue_loop {
                        state = evaluate(&statements, state, as_function)?;
                    } else {
                        break;
                    }
                }
            }

            Statement::Repeat(statements, expression) => loop {
                state = evaluate(statements, state, as_function)?;
                let condition = evaluate_expression(expression, &state)?;
                match condition {
                    Literal::Bool(value) => {
                        if value {
                            break;
                        };
                    }
                    _ => {
                        return Err(Execution::IncorrectType(
                            LiteralType::Boolean,
                            LiteralType::from(&condition),
                        ))
                    }
                }
            },

            Statement::ProcedureCall(_, _) => {
                todo!()
            }
        }
    }
    state.scopes.pop();
    Ok(state)
}
