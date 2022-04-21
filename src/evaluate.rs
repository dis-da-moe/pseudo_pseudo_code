use std::collections::HashMap;
use std::mem;
use crate::tokens::*;

#[derive(Debug, PartialEq)]
pub enum Execution {
    NotFound(String),
    NotAssigned(String),
    IncorrectType(DataTypes, DataTypes)
}

fn value_matches_type(value: &Value, data_type: DataTypes) -> Result<(), Execution> {
    let value_type = value.into();
    match data_type == value_type {
        true => Ok(()),
        false => Err(Execution::IncorrectType(data_type, value_type))
    }
}


pub fn evaluate(statements: Vec<Statement>) -> Result<(), Execution>{
    let mut variables: HashMap<String, Variable> = HashMap::new();

    for statement in statements {
        match statement {
            Statement::Declare(identifier, data_type) => {
                variables.insert(identifier, data_type.into());
            },
            
            Statement::Assign(identifier, value) => {
                let variable = variables.get_mut(identifier.as_str())
                    .ok_or_else(|| Execution::NotFound(identifier))?;

                value_matches_type(&value, variable.data_type.clone())?;

                variable.value = Some(value);
            },
            
            Statement::Out(identifier) => {
                let variable = variables.get(identifier.as_str())
                    .ok_or_else(|| Execution::NotFound(identifier.clone()))?;

                let value = variable.value.as_ref()
                    .ok_or_else(|| Execution::NotAssigned(identifier))?;

                println!("{}", value);
            },
        }
    }
    
    Ok(())
}