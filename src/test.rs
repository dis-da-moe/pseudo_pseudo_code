

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use crate::evaluate::*;
    use crate::tokens::*;
    
    fn equivalent(expected: &Result<(), Execution>, statements: Vec<&Statement>){
        let result = 
            &evaluate(statements.iter().map(|x| x.deref().clone()).collect());
        assert_eq!(
            result,
            expected, 
            "\nStatements: {:?} \nResulted in: {:?} \nBut Expected: {:?}",
            statements,
            result,
            expected
        )
    }
    
    #[test]
    fn assign_test(){
        let x = String::from("x");
        let y = String::from("y");
        
        use Statement::*;
        use Execution::*;
        let assign_x = Assign(x.clone(), Value::Integer(10));
        let declare_x = Declare(x.clone(), DataTypes::Integer);
        let assign_y = Assign(y.clone(), Value::Integer(10));
        let declare_y = Declare(y.clone(), DataTypes::Integer);
        let out_x = Out(x.clone());
        let out_y = Out(y.clone());
        
        let not_found_statements = vec![
            vec![&assign_x],
            vec![&declare_y, &assign_x],
            vec![&out_x]
        ];
        
        let not_assigned = vec![
            vec![&declare_x, &out_x]
        ];
        
        let ok_statements = vec![
            vec![&declare_x, &assign_x, &out_x],
            vec![&declare_x, &declare_y, &assign_x, &assign_y, &out_x, &out_y]
        ];
        
        let tests = [
            (Ok(()), ok_statements),
            (Err(NotFound(x.clone())), not_found_statements),
            (Err(NotAssigned(x.clone())), not_assigned)
        ];
        
        for test in tests {
            for statements in test.1 {
                equivalent(&test.0, statements);
            }
        }
    }
}