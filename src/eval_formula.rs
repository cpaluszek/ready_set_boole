use crate::{ast::{Ast, AstNode}, LogicalSymbol};

// NOTE: return a Result<> instead of bool?
pub fn eval_formula(formula: &str) -> bool {
    let mut stack: Vec<AstNode> = Vec::new();
    for character in formula.chars() {
        println!("char: {character}");
        if let Some(symbol) = LogicalSymbol::from_char(character) {
            if symbol.is_operand() {
                stack.push(AstNode::Operand(symbol));
            } else {
                if stack.len() >= 2 {
                    let node = AstNode::Operator(symbol, Box::new(stack.pop().unwrap()), Box::new(stack.pop().unwrap()));
                    stack.push(node);
                } else {
                    eprintln!("Operator '{}' encountered, but only {} operand(s) on stack. Expected 2", character, stack.len());
                    return false;
                }
            }
        }
        else {
            eprintln!("{character} is not a recognized symbol.");
            return false;
        }
    }

    // Stack should only contains one element
    if stack.len() == 1 {
        let formula_ast = Ast::new(stack.pop().unwrap());
        println!("{}", formula_ast);
    } else {
        eprintln!("Final stack contains {} elements instead of 1.", stack.len());
    }

    return false;
}
