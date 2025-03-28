// use ready_set_boole::{ast::AstNode, conjunctive_normal_form, evaluate::build_ast, symbol::LogicalSymbol};
//
// #[test]
// fn test_conjunctive_normal_form() {
//     let formula = "AB&!";
//     let cnf = conjunctive_normal_form(&formula);
//     let root_cnf = build_ast(&cnf);
//     assert!(is_cnf(&root_cnf.unwrap().root.unwrap()));
// }
//
// #[test]
// fn test_conjunctive_normal_form2() {
//     let formula = "AB|!";
//     let cnf = conjunctive_normal_form(&formula);
//     let root_cnf = build_ast(&cnf);
//     assert!(is_cnf(&root_cnf.unwrap().root.unwrap()));
// }
//
// #[test]
// fn test_conjunctive_normal_form3() {
//     let formula = "AB|C&";
//     let cnf = conjunctive_normal_form(&formula);
//     let root_cnf = build_ast(&cnf);
//     assert!(is_cnf(&root_cnf.unwrap().root.unwrap()));
// }
//
// #[test]
// fn test_conjunctive_normal_form4() {
//     let formula = "AB|C|D|";
//     let cnf = conjunctive_normal_form(&formula);
//     println!("CNF: {cnf}");
//     let root_cnf = build_ast(&cnf);
//     assert!(is_cnf(&root_cnf.unwrap().root.unwrap()));
// }
//
// #[test]
// fn test_conjunctive_normal_form5() {
//     let formula = "AB&C&D&";
//     let cnf = conjunctive_normal_form(&formula);
//     let root_cnf = build_ast(&cnf);
//     assert!(is_cnf(&root_cnf.unwrap().root.unwrap()));
// }
//
// #[test]
// fn test_conjunctive_normal_form6() {
//     let formula = "AB&!C!|";
//     let cnf = conjunctive_normal_form(&formula);
//     let root_cnf = build_ast(&cnf);
//     assert!(is_cnf(&root_cnf.unwrap().root.unwrap()));
// }
//
// #[test]
// fn test_conjunctive_normal_form7() {
//     let formula = "AB|!C!&";
//     let cnf = conjunctive_normal_form(&formula);
//     let root_cnf = build_ast(&cnf);
//     assert!(is_cnf(&root_cnf.unwrap().root.unwrap()));
// }
//
//
// fn is_cnf(node: &AstNode) -> bool {
//     match node {
//         AstNode::Operand(_) | AstNode::Negation(_) => true,
//         AstNode::Operator(op, left, right) => {
//             match op {
//                 LogicalSymbol::Conjunction => {
//                     is_cnf(left) && is_cnf(right)
//                 },
//                 LogicalSymbol::Disjunction => {
//                     is_clause(left) && is_clause(right)
//                 }
//                 _ => false,
//             }
//         }
//     }
// }
//
// fn is_clause(node: &AstNode) -> bool {
//     match node {
//         AstNode::Operand(_) | AstNode::Negation(_) => true,
//         AstNode::Operator(op, left, right) => {
//             match op {
//                 LogicalSymbol::Disjunction => {
//                     is_literal(left) && is_clause(right)
//                 }
//                 _ => false,
//             }
//         }
//     }
// }
//
// fn is_literal(node: &AstNode) -> bool {
//     match node {
//         AstNode::Operand(_) => true,
//         AstNode::Negation(inner) => {
//             match **inner {
//                 AstNode::Operand(_) => true,
//                 _ => false,
//             }
//         },
//         _ => false,
//     }
// }
