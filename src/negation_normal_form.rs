use crate::{ast::{Ast, AstNode}, evaluate::build_ast, symbol::LogicalSymbol, LogicError};

/*
- Double negation elimination: (¬¬A) ⇔ A
- Material condition: (A ⇒ B) ⇔ (¬A ∨ B)
- Equivalence: (A ⇔ B) ⇔ ((A ⇒ B) ∧ (B ⇒ A))
- De Morgan's laws: 
    - ¬(A ∨ B) ⇔ (¬A ∧ ¬B)
    - ¬(A ∧ B) ⇔ (¬A ∨ ¬B)
- Distributivity:
    - (A ∧ (B ∨ C)) ⇔ ((A ∧ B) ∨ (A ∧ C))
    - (A ∨ (B ∧ C)) ⇔ ((A ∨ B) ∧ (A ∨ C))
*/

pub fn negation_normal_form(formula: &str) -> String {
    match build_ast(formula) {
        Ok(ast) => {
            if let Some(root) = ast.root {
                let nnf_root = to_nnf_recursive(&root, false);
                match nnf_root {
                    Ok(nnf) => {
                        let nnf_ast = Ast::new(nnf);
                        return nnf_ast.to_rpn();
                    },
                    Err(err) => eprintln!("Error occurred while evaluating: {}", err),
                }
            }
        }
        Err(err) => eprintln!("Error occurred while evaluating: {}", err),
    }
    return String::new();
}

// NOTE: is the error type necessary?
fn to_nnf_recursive(node: &AstNode, negated: bool) -> Result<AstNode, LogicError> {
    // The negated parameter tracks if the current node is negated
    match node {
        AstNode::Operand(symbol) => {
            match symbol {
                LogicalSymbol::True if negated => Ok(AstNode::Operand(LogicalSymbol::False)),
                LogicalSymbol::False if negated => Ok(AstNode::Operand(LogicalSymbol::True)),
                LogicalSymbol::Variable(v) if negated => {
                    Ok(AstNode::Negation(Box::new(AstNode::Operand(LogicalSymbol::Variable(*v)))))
                },
                _ => Ok(AstNode::Operand(symbol.clone())),
            }
        },
        AstNode::Negation(node) => {
            to_nnf_recursive(node, !negated)
        },
        AstNode::Operator(op, left, right) => {
            match op {
                // Implication: A ⇒ B ⇔ ¬A ∨ B
                LogicalSymbol::Implication if !negated => {
                    // Convert A ⇒ B to ¬A ∨ B
                    let not_a = to_nnf_recursive(left, true)?;
                    let b = to_nnf_recursive(right, false)?;

                    Ok(AstNode::Operator(LogicalSymbol::Disjunction, Box::new(not_a), Box::new(b)))
                },
                // Negated implication: ¬(A ⇒ B) ⇔ A ∧ ¬B
                LogicalSymbol::Implication if negated => {
                    let a = to_nnf_recursive(left, false)?;
                    let not_b = to_nnf_recursive(right, true)?;

                    Ok(AstNode::Operator(LogicalSymbol::Conjunction, Box::new(a), Box::new(not_b)))
                },
                // Equivalence: A ⇔ B ⇔ (A ⇒ B) ∧ (B ⇒ A) ⇔ (¬A ∨ B) ∧ (¬B ∨ A)
                LogicalSymbol::Equivalence if !negated => {
                    // (¬B ∨ A)
                    let not_a = to_nnf_recursive(left, true)?;
                    let b = to_nnf_recursive(right, false)?;
                    let left_disj = AstNode::Operator(LogicalSymbol::Disjunction, Box::new(not_a), Box::new(b));
 
                    // (¬B ∨ A)
                    let a = to_nnf_recursive(left, false)?;
                    let not_b = to_nnf_recursive(right, true)?;
                    let right_disj = AstNode::Operator(LogicalSymbol::Disjunction, Box::new(not_b), Box::new(a));

                    Ok(AstNode::Operator(LogicalSymbol::Conjunction, Box::new(left_disj), Box::new(right_disj)))
                }
                // Negated equivalence: ¬(A ⇔ B) ⇔ (A ∧ ¬B) ∨ (¬A ∧ B)
                LogicalSymbol::Equivalence if negated => {
                    // (A ∧ ¬B) ∨ (¬A ∧ B)
                    let a = to_nnf_recursive(left, false)?;
                    let not_b = to_nnf_recursive(right, true)?;
                    let left_conj = AstNode::Operator(LogicalSymbol::Conjunction, Box::new(a), Box::new(not_b));
 
                    // (A ∧ ¬B) ∨ (¬A ∧ B)
                    let not_a = to_nnf_recursive(left, true)?;
                    let b = to_nnf_recursive(right, false)?;
                    let right_conj = AstNode::Operator(LogicalSymbol::Conjunction, Box::new(not_a), Box::new(b));

                    Ok(AstNode::Operator(LogicalSymbol::Disjunction, Box::new(left_conj), Box::new(right_conj)))
                }
                // Conjunction with De Morgan's Law for negation
                LogicalSymbol::Conjunction => {
                    // ¬(A ∧ B) ⇔ (¬A ∨ ¬B)
                    if negated {
                        let not_a = to_nnf_recursive(left, true)?;
                        let not_b = to_nnf_recursive(right, true)?;

                        Ok(AstNode::Operator(LogicalSymbol::Disjunction, Box::new(not_a), Box::new(not_b)))
                    } else {
                        let a = to_nnf_recursive(left, false)?;
                        let b = to_nnf_recursive(right, false)?;

                        Ok(AstNode::Operator(LogicalSymbol::Conjunction, Box::new(a), Box::new(b)))
                    }
                }
                // Disjunction with De Morgan's Law for negation
                LogicalSymbol::Disjunction => {
                    if negated {
                        let not_a = to_nnf_recursive(left, true)?;
                        let not_b = to_nnf_recursive(right, true)?;

                        Ok(AstNode::Operator(LogicalSymbol::Conjunction, Box::new(not_a), Box::new(not_b)))
                    } else {
                        let a = to_nnf_recursive(left, false)?;
                        let b = to_nnf_recursive(right, false)?;

                        Ok(AstNode::Operator(LogicalSymbol::Disjunction, Box::new(a), Box::new(b)))
                    }
                },
                // XOR: A ⊕ B ⇔ (A ∧ ¬B) ∨ (¬A ∧ B)
                LogicalSymbol::ExclusiveOr if !negated => {
                    // (A ∧ ¬B)
                    let a = to_nnf_recursive(left, false)?;
                    let not_b = to_nnf_recursive(right, true)?;
                    let first_part = AstNode::Operator(LogicalSymbol::Conjunction, Box::new(a), Box::new(not_b));

                    // (¬A ∧ B)
                    let not_a = to_nnf_recursive(left, true)?;
                    let b = to_nnf_recursive(right, false)?;
                    let second_part = AstNode::Operator(LogicalSymbol::Conjunction, Box::new(not_a), Box::new(b));

                    Ok(AstNode::Operator(LogicalSymbol::Disjunction, Box::new(first_part), Box::new(second_part)))
                }
                // Negated XOR: ¬(A ⊕ B) ⇔ (A ∧ B) ∨ (¬A ∧ ¬B)
                LogicalSymbol::ExclusiveOr if negated => {
                    // (A ∧ B)
                    let a = to_nnf_recursive(left, false)?;
                    let b = to_nnf_recursive(right, false)?;
                    let first_part = AstNode::Operator(LogicalSymbol::Conjunction, Box::new(a), Box::new(b));

                    // (¬A ∧ ¬B)
                    let not_a = to_nnf_recursive(left, true)?;
                    let not_b = to_nnf_recursive(right, true)?;
                    let second_part = AstNode::Operator(LogicalSymbol::Conjunction, Box::new(not_a), Box::new(not_b));

                    Ok(AstNode::Operator(LogicalSymbol::Disjunction, Box::new(first_part), Box::new(second_part)))
                }
                // Default case: should never happen
                _ => Ok(node.clone())
            }
        }
    }
}
