use crate::{ast::{Ast, AstNode}, LogicalSymbol};

enum EvalError {
    MissingArgument(char),
    UnrecognizedSymbol(char),
    IncompleteFormula(usize),
}

impl EvalError {
    fn description(&self) -> String {
        match *self {
            EvalError::MissingArgument(c) => format!("Missing argument for operator '{}'", c),
            EvalError::UnrecognizedSymbol(c) => format!("'{}' is not a recognized symbol.", c),
            EvalError::IncompleteFormula(len) => format!("Incomplete formula, stack contains {} elements instead of 1.", len),
        }
    }
}

pub fn eval_formula(formula: &str) -> bool {
    match try_eval_formula(formula) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Error occurred while evaluating: {}", err.description());
            false
        }
    }
}

fn try_eval_formula(formula: &str) -> Result<bool, EvalError> {
    let mut stack: Vec<bool> = Vec::new();
    for character in formula.chars() {
        if let Some(symbol) = LogicalSymbol::from_char(character) {
            if symbol.is_operand() {
                let value = match symbol {
                    LogicalSymbol::True => true,
                    LogicalSymbol::False => false,
                    _ => return Err(EvalError::UnrecognizedSymbol(character)),
                };
                stack.push(value);
            } else if symbol == LogicalSymbol::Negation {
                if stack.len() < 1 {
                    return Err(EvalError::MissingArgument(character));
                }
                let val = stack.pop().unwrap();
                stack.push(!val);
            } else {
                if stack.len() < 2 {
                    return Err(EvalError::MissingArgument(character));
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
            return Err(EvalError::UnrecognizedSymbol(character));
        }
    }

    if stack.len() != 1 {
        return Err(EvalError::IncompleteFormula(stack.len()));
    }

    return Ok(stack.pop().unwrap());
}

pub fn build_and_print_ast(formula: &str) {
    let mut stack: Vec<AstNode> = Vec::new();
    for character in formula.chars() {
        if let Some(symbol) = LogicalSymbol::from_char(character) {
            if symbol.is_operand() {
                stack.push(AstNode::Operand(symbol));
            } else if symbol == LogicalSymbol::Negation {
                if stack.len() < 1 {
                    return ;
                }
                let node = AstNode::Operator(symbol, Some(Box::new(stack.pop().unwrap())), None);
                stack.push(node);

            } else {
                if stack.len() < 2 {
                    return ;
                }
                let node = AstNode::Operator(symbol, Some(Box::new(stack.pop().unwrap())), Some(Box::new(stack.pop().unwrap())));
                stack.push(node);
            }
        }
        else {
            return ;
        }
    }

    if stack.len() != 1 {
        return ;
    }

    let formula_ast = Ast::new(stack.pop().unwrap());
    println!("{}", formula_ast);
}
