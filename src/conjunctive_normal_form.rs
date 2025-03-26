// use crate::{ast::AstNode, to_nnf_recursive, LogicError};
// use crate::symbol::LogicalSymbol;
// use crate::evaluate::build_ast;
// use crate::ast::create_binary_op;
//
// // NOTE: use karnaugh map to simplify the formula?
// pub fn conjunctive_normal_form(formula: &str) -> String {
//     match build_ast(formula) {
//         Ok(ast) => {
//             if let Some(root) = ast.root {
//                 // convert to NNF
//                 match to_nnf_recursive(&root, false) {
//                     Ok(nnf_root) => {
//                         // Then apply CNF transformation
//                         match to_cnf_recursive(&nnf_root) {
//                             Ok(cnf) => {
//                                 format_cnf_to_rpn(&cnf)
//                             },
//                             Err(err) => {
//                                 eprintln!("Error occurred while evaluating: {}", err);
//                                 String::new()
//                             }
//                         }
//                     }
//                     Err(err) => {
//                         eprintln!("Error occurred while evaluating: {}", err);
//                         String::new()
//                     }
//                 }
//             } else {
//                 String::new()
//             }
//         }
//         Err(err) => {
//             eprintln!("Error occurred while evaluating: {}", err);
//             String::new()
//         }
//     }
// }
//
// fn format_cnf_to_rpn(node: &AstNode) -> String {
//     let mut literals = Vec::new();
//     let mut operators = Vec::new();
//
//     collect_cnf_components(node, &mut literals, &mut operators);
//
//     literals.into_iter().chain(operators.into_iter()).collect()
// }
//
// fn collect_cnf_components(node: &AstNode, literals: &mut Vec<String>, operators: &mut Vec<String>) {
//     match node {
//         AstNode::Operand(symbol) => {
//             literals.push(symbol.to_unicode().to_string());
//         },
//         AstNode::Negation(inner) => {
//             if let AstNode::Operand(symbol) = **inner {
//                 // For negated variables, add the variable and negation together
//                 literals.push(format!("{}{}", symbol.to_unicode(), LogicalSymbol::Negation.to_unicode()));
//             } else {
//                 // This shouldn't happen after NNF conversion
//                 eprintln!("Warning: Complex negation found in CNF formula");
//                 collect_cnf_components(inner, literals, operators);
//                 operators.push(LogicalSymbol::Negation.to_unicode().to_string());
//             }
//         },
//         AstNode::Operator(op, left, right) => {
//             // // Process the left and right nodes first
//             // collect_cnf_components(left, literals, operators);
//             // collect_cnf_components(right, literals, operators);
//             //
//             // // Add the operator after processing both operands
//             // operators.push(op.to_unicode_symbol().to_string());
//             match op {
//                 LogicalSymbol::Conjunction => {
//                     // Process operands first
//                     collect_cnf_components(left, literals, operators);
//                     collect_cnf_components(right, literals, operators);
//                     // Add conjunction at the end
//                     operators.push(op.to_unicode().to_string());
//                 },
//                 LogicalSymbol::Disjunction => {
//                     // Process operands and add disjunction in the middle
//                     collect_cnf_components(left, literals, operators);
//                     collect_cnf_components(right, literals, operators);
//                     // Add disjunction with literals
//                     operators.push(op.to_unicode().to_string());
//                 },
//                 _ => {
//                     // Shouldn't happen in CNF
//                     eprintln!("Warning: Unexpected operator in CNF formula");
//                     collect_cnf_components(left, literals, operators);
//                     collect_cnf_components(right, literals, operators);
//                     operators.push(op.to_unicode().to_string());
//                 }
//             }
//         }
//     }
// }
//
// fn to_cnf_recursive(node: &AstNode) -> Result<AstNode, LogicError> {
//     match node {
//         AstNode::Operand(_) | AstNode::Negation(_) => {
//             Ok(node.clone())
//         },
//         AstNode::Operator(op, left, right) => {
//             // Recursively convert the children to CNF
//             let left_cnf = to_cnf_recursive(left)?;
//             let right_cnf = to_cnf_recursive(right)?;
//
//             match op {
//                 LogicalSymbol::Conjunction => {
//                     // For A ∧ B, both A and B must be in CNF
//                     Ok(create_binary_op(LogicalSymbol::Conjunction, left_cnf, right_cnf))
//                 },
//                 LogicalSymbol::Disjunction => {
//                     // For A ∨ B, apply distributive law
//                     distribute_disjunction_over_conjunction(&left_cnf, &right_cnf)
//                 },
//                 _ => {
//                     // At this point, we should only have conjunctions and disjunctions
//                     // since we've already converted to NNF
//                     Err(LogicError::UnexpectedOperatorCNF)
//                 }
//             }
//         }
//     }
// }
//
// fn distribute_disjunction_over_conjunction(left: &AstNode, right: &AstNode) -> Result<AstNode, LogicError> {
//     match (left, right) {
//         // (A ∧ B) ∨ C → (A ∨ C) ∧ (B ∨ C)
//         (AstNode::Operator(LogicalSymbol::Conjunction, a, b), c) => {
//             let a_or_c = distribute_disjunction_over_conjunction(a, c)?;
//             let b_or_c = distribute_disjunction_over_conjunction(b, c)?;
//             Ok(create_binary_op(LogicalSymbol::Conjunction, a_or_c, b_or_c))
//         },
//         // C ∨ (A ∧ B) → (C ∨ A) ∧ (C ∨ B)
//         (c, AstNode::Operator(LogicalSymbol::Conjunction, a, b)) => {
//             let c_or_a = distribute_disjunction_over_conjunction(c, a)?;
//             let c_or_b = distribute_disjunction_over_conjunction(c, b)?;
//             Ok(create_binary_op(LogicalSymbol::Conjunction, c_or_a, c_or_b))
//         },
//         // For simple disjunctions, return them as is
//         (a, b) => {
//             Ok(create_binary_op(LogicalSymbol::Disjunction, a.clone(), b.clone()))
//         }
//     }
// }
