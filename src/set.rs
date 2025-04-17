use std::collections::HashSet;

use crate::{pop_from_stack, LogicError};

// A powerset of S is the set of all subsets of S
// including the empty set and S.
pub fn powerset(set: Vec<i32>) -> Vec<Vec<i32>> {
    let mut powerset = Vec::with_capacity(1 << set.len());

    for i in 0..1 << set.len() {
        let mut subset = Vec::new();
        for (j, val) in set.iter().enumerate() {
            if (i >> j) & 1 == 1 {
                subset.push(*val);
            }
        }
        powerset.push(subset);
    }

    powerset
}

pub fn has_duplicate(set: &Vec<i32>) -> bool {
    let mut seen = HashSet::with_capacity(set.len());
    for &val in set {
        if !seen.insert(val) {
            return true;
        }
    }
    false
}

fn union(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut result = a.clone();
    for value in &b {
        if !a.contains(value) {
            result.push(*value);
        }
    }
    result
}

fn xunion(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for value in &a {
        if !b.contains(value) {
            result.push(*value);
        }
    }
    for value in &b {
        if !a.contains(value) {
            result.push(*value);
        }
    }
    result
}

fn intersection(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for value in &b {
        if a.contains(value) {
            result.push(*value);
        }
    }
    result
}

fn complement(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for value in &a {
        if !b.contains(value) {
            result.push(*value);
        }
    }
    result
}

pub fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
    match try_eval_set(formula, sets) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Error occurred while evaluating: {err:?}");
            Vec::new()
        }
    }
}

fn try_eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Result<Vec<i32>, LogicError> {
    let mut stack: Vec<Vec<i32>> = Vec::with_capacity(formula.len());

    // The globally encompassing set is considered to be the union of all
    // the sets given as parameters
    let mut all = Vec::new();
    for set in &sets {
        all = union(all, set.to_vec());
    }

    for character in formula.chars() {
        match character {
            'A'..='Z' => {
                let index = character as usize - 'A' as usize;
                let set = sets.get(index).ok_or(LogicError::UnknownVar)?;
                stack.push(set.clone());
            }

            // Negation
            '!' => {
                let val = pop_from_stack(&mut stack)?;
                stack.push(complement(all.clone(), val));
            },

            // Binary operators
            '&' => {
                let right = pop_from_stack(&mut stack)?;
                let left = pop_from_stack(&mut stack)?;
                stack.push(intersection(right, left));
            },
            '|' => {
                let right = pop_from_stack(&mut stack)?;
                let left = pop_from_stack(&mut stack)?;
                stack.push(union(right, left));
            },
            '^' => {
                let right = pop_from_stack(&mut stack)?;
                let left = pop_from_stack(&mut stack)?;
                stack.push(xunion(right, left));
            },
            '>' => {
                let right = pop_from_stack(&mut stack)?;
                let left = pop_from_stack(&mut stack)?;
                stack.push(complement(all.clone(), complement(right, left)))
            },
            '=' => {
                let right = pop_from_stack(&mut stack)?;
                let left = pop_from_stack(&mut stack)?;
                stack.push(complement(all.clone(), intersection(left, right)));
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

