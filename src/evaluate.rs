use crate::{pop_from_stack, LogicError};

pub fn eval_formula(formula: &str) -> bool {
    match try_eval_formula(formula) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Error occurred while evaluating: {err:?}");
            false
        }
    }
}

fn try_eval_formula(formula: &str) -> Result<bool, LogicError> {
    let mut stack: Vec<bool> = Vec::with_capacity(formula.len());

    for character in formula.chars() {
        match character {
            // Constants
            '1' => stack.push(true),
            '0' => stack.push(false),

            // Negation
            '!' => {
                let val = pop_from_stack(&mut stack)?;
                stack.push(!val);
            },

            // Binary operators
            '&' => {
                let right = pop_from_stack(&mut stack)?;
                let left = pop_from_stack(&mut stack)?;
                stack.push(left && right);
            },
            '|' => {
                let right = pop_from_stack(&mut stack)?;
                let left = pop_from_stack(&mut stack)?;
                stack.push(left || right);
            },
            '^' => {
                let right = pop_from_stack(&mut stack)?;
                let left = pop_from_stack(&mut stack)?;
                stack.push(left != right);
            },
            '>' => {
                let right = pop_from_stack(&mut stack)?;
                let left = pop_from_stack(&mut stack)?;
                stack.push(!left || right);
            },
            '=' => {
                let right = pop_from_stack(&mut stack)?;
                let left = pop_from_stack(&mut stack)?;
                stack.push(left == right);
            },

            // Unrecognized character
            _ => return Err(LogicError::UnrecognizedSymbol),
        }
    }

    if stack.len() != 1 {
        return Err(LogicError::IncompleteFormula);
    }

    // Safe because we just checked stack.len() == 1
    Ok(stack.pop().unwrap()) 
}

