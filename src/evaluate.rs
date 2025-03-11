use crate::symbol::LogicalSymbol;

use crate::ast::{Ast, AstNode};
use crate::LogicError;

pub fn eval_formula(formula: &str) -> bool {
    match try_eval_formula(formula) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Error occurred while evaluating: {}", err);
            false
        }
    }
}

fn try_eval_formula(formula: &str) -> Result<bool, LogicError> {
    let mut stack: Vec<bool> = Vec::with_capacity(formula.len());
    for character in formula.chars() {
        if let Some(symbol) = LogicalSymbol::from_char(character) {
            if symbol.is_operand() {
                let value = match symbol {
                    LogicalSymbol::True => true,
                    LogicalSymbol::False => false,
                    _ => return Err(LogicError::UnrecognizedSymbol(character)),
                };
                stack.push(value);
            } else if symbol == LogicalSymbol::Negation {
                if stack.is_empty() {
                    return Err(LogicError::MissingArgument(character));
                }
                let val = stack.pop().unwrap();
                stack.push(!val);
            } else {
                if stack.len() < 2 {
                    return Err(LogicError::MissingArgument(character));
                }
                let left = stack.pop().unwrap();
                let right = stack.pop().unwrap();

                let result = match symbol {
                    LogicalSymbol::Conjunction => left && right,
                    LogicalSymbol::Disjunction => left || right,
                    LogicalSymbol::ExclusiveOr => left != right,
                    LogicalSymbol::Implication => !left || right,
                    LogicalSymbol::Equivalence => left == right,
                    _ => unreachable!()
                };

                stack.push(result);
            }
        }
        else {
            return Err(LogicError::UnrecognizedSymbol(character));
        }
    }

    if stack.len() != 1 {
        return Err(LogicError::IncompleteFormula{
            expected: 1,
            actual: stack.len()
        });
    }

    return Ok(stack.pop().unwrap());
}

pub fn build_and_print_ast(formula: &str) {
    match build_ast(formula) {
        Ok(ast) => println!("{}", ast),
        Err(err) => eprintln!("Error occurred while evaluating: {}", err),
    }
}

pub fn build_ast(formula: &str) -> Result<Ast, LogicError> {
    let mut stack: Vec<AstNode> = Vec::with_capacity(formula.len());

    for character in formula.chars() {
        if let Some(symbol) = LogicalSymbol::from_char(character) {
            if symbol.is_operand() {
                stack.push(AstNode::Operand(symbol));
            } else if symbol == LogicalSymbol::Negation {
                if stack.is_empty() {
                    return Err(LogicError::MissingArgument(character));
                }
                let operand = stack.pop().unwrap();
                stack.push(AstNode::Operator(symbol, Some(Box::new(operand)), None));
            } else {
                if stack.len() < 2 {
                    return Err(LogicError::MissingArgument(character));
                }

                let left = stack.pop().unwrap();
                let right = stack.pop().unwrap();
                let node = AstNode::Operator(symbol, Some(Box::new(left)), Some(Box::new(right)));
                stack.push(node);
            }
        }
        else {
            return Err(LogicError::UnrecognizedSymbol(character));
        }
    }

    if stack.len() != 1 {
        return Err(LogicError::IncompleteFormula{
            expected: 1,
            actual: stack.len()
        });
    }

    let formula_ast = Ast::new(stack.pop().unwrap());
    return Ok(formula_ast);
}

