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
                match to_nnf_recursive(&root, false) {
                    Ok(nnf) => Ast::new(nnf).to_rpn(),
                    Err(err) => {
                        eprintln!("Error occurred while evaluating: {}", err);
                        String::new()
                    }
                }
            } else {
                String::new()
            }
        }
        Err(err) => {
            eprintln!("Error occurred while evaluating: {}", err);
            String::new()
        }
    }
}

fn create_binary_op(op: LogicalSymbol, left: AstNode, right: AstNode) -> AstNode {
    AstNode::Operator(op, Box::new(left), Box::new(right))
}

fn to_nnf_recursive(node: &AstNode, negated: bool) -> Result<AstNode, LogicError> {
    // The negated parameter tracks if the current node is negated
    match node {
        AstNode::Operand(symbol) => handle_operand(symbol, negated),
        AstNode::Negation(node) => to_nnf_recursive(node, !negated),
        AstNode::Operator(op, left, right) => handle_operator(op, left, right, negated),
    }
}

fn handle_operand(symbol: &LogicalSymbol, negated: bool) -> Result<AstNode, LogicError> {
    match (symbol, negated) {
        (LogicalSymbol::True, true) => Ok(AstNode::Operand(LogicalSymbol::False)),
        (LogicalSymbol::False, true) => Ok(AstNode::Operand(LogicalSymbol::True)),
        (LogicalSymbol::Variable(v), true) => {
            Ok(AstNode::Negation(Box::new(AstNode::Operand(LogicalSymbol::Variable(*v)))))
        },
        _ => Ok(AstNode::Operand(symbol.clone())),
    }
}

fn handle_operator(op: &LogicalSymbol, left: &AstNode, right: &AstNode, negated: bool) -> Result<AstNode, LogicError> {
    match (op, negated) {
        // Implication: A ⇒ B ⇔ ¬A ∨ B
        (LogicalSymbol::Implication, false) => {
            // Convert A ⇒ B to ¬A ∨ B
            let not_a = to_nnf_recursive(left, true)?;
            let b = to_nnf_recursive(right, false)?;

            Ok(create_binary_op(LogicalSymbol::Disjunction, not_a, b))
        },
        // Negated implication: ¬(A ⇒ B) ⇔ A ∧ ¬B
        (LogicalSymbol::Implication, true) => {
            let a = to_nnf_recursive(left, false)?;
            let not_b = to_nnf_recursive(right, true)?;

            Ok(create_binary_op(LogicalSymbol::Conjunction, a, not_b))
        },
        // Equivalence: A ⇔ B ⇔ (A ⇒ B) ∧ (B ⇒ A) ⇔ (¬A ∨ B) ∧ (¬B ∨ A)
        (LogicalSymbol::Equivalence, false) => {
            // (¬B ∨ A)
            let not_a = to_nnf_recursive(left, true)?;
            let b = to_nnf_recursive(right, false)?;
            let left_disj = create_binary_op(LogicalSymbol::Disjunction, not_a, b);

            // (¬B ∨ A)
            let a = to_nnf_recursive(left, false)?;
            let not_b = to_nnf_recursive(right, true)?;
            let right_disj = create_binary_op(LogicalSymbol::Disjunction, not_b, a);

            Ok(create_binary_op(LogicalSymbol::Conjunction, left_disj, right_disj))
        }
        // Negated equivalence: ¬(A ⇔ B) ⇔ (A ∧ ¬B) ∨ (¬A ∧ B)
        (LogicalSymbol::Equivalence, true)=> {
            // (A ∧ ¬B) ∨ (¬A ∧ B)
            let a = to_nnf_recursive(left, false)?;
            let not_b = to_nnf_recursive(right, true)?;
            let left_conj = create_binary_op(LogicalSymbol::Conjunction, a, not_b);

            // (A ∧ ¬B) ∨ (¬A ∧ B)
            let not_a = to_nnf_recursive(left, true)?;
            let b = to_nnf_recursive(right, false)?;
            let right_conj = create_binary_op(LogicalSymbol::Conjunction, not_a, b);

            Ok(create_binary_op(LogicalSymbol::Disjunction, left_conj, right_conj))
        }
        // Conjunction with De Morgan's Law for negation
        (LogicalSymbol::Conjunction, true) => {
            // ¬(A ∧ B) ⇔ (¬A ∨ ¬B)
            let not_a = to_nnf_recursive(left, true)?;
            let not_b = to_nnf_recursive(right, true)?;

            Ok(create_binary_op(LogicalSymbol::Disjunction, not_a, not_b))
        }
        (LogicalSymbol::Conjunction, false) => {
            let a = to_nnf_recursive(left, false)?;
            let b = to_nnf_recursive(right, false)?;

            Ok(create_binary_op(LogicalSymbol::Conjunction, a, b))
        }
        // Disjunction with De Morgan's Law for negation
        (LogicalSymbol::Disjunction, true) => {
            let not_a = to_nnf_recursive(left, true)?;
            let not_b = to_nnf_recursive(right, true)?;

            Ok(create_binary_op(LogicalSymbol::Conjunction, not_a, not_b))
        },
        (LogicalSymbol::Disjunction, false) => {
            let a = to_nnf_recursive(left, false)?;
            let b = to_nnf_recursive(right, false)?;

            Ok(create_binary_op(LogicalSymbol::Disjunction, a, b))
        },
        // XOR: A ⊕ B ⇔ (A ∧ ¬B) ∨ (¬A ∧ B)
        (LogicalSymbol::ExclusiveOr, false) => {
            // (A ∧ ¬B)
            let a = to_nnf_recursive(left, false)?;
            let not_b = to_nnf_recursive(right, true)?;
            let first_part = create_binary_op(LogicalSymbol::Conjunction, a, not_b);

            // (¬A ∧ B)
            let not_a = to_nnf_recursive(left, true)?;
            let b = to_nnf_recursive(right, false)?;
            let second_part = create_binary_op(LogicalSymbol::Conjunction, not_a, b);

            Ok(create_binary_op(LogicalSymbol::Disjunction, first_part, second_part))
        }
        // Negated XOR: ¬(A ⊕ B) ⇔ (A ∧ B) ∨ (¬A ∧ ¬B)
        (LogicalSymbol::ExclusiveOr, true) => {
            // (A ∧ B)
            let a = to_nnf_recursive(left, false)?;
            let b = to_nnf_recursive(right, false)?;
            let first_part = create_binary_op(LogicalSymbol::Conjunction, a, b);

            // (¬A ∧ ¬B)
            let not_a = to_nnf_recursive(left, true)?;
            let not_b = to_nnf_recursive(right, true)?;
            let second_part = create_binary_op(LogicalSymbol::Conjunction, not_a, not_b);

            Ok(create_binary_op(LogicalSymbol::Disjunction, first_part, second_part))
        }
        // Default case: should never happen
        _ => Ok(create_binary_op(op.clone(), to_nnf_recursive(left, false)?, to_nnf_recursive(right, false)?))
    }
}

