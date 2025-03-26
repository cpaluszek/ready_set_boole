// use std::collections::HashSet;
//
// use crate::evaluate::build_ast;
//
// pub fn print_truth_table(formula: &str) {
//     match build_ast(formula) {
//         Ok(ast) => {
//             let vars_set = ast.variables();
//             let mut vars: Vec<char> = vars_set.into_iter().collect();
//             vars.sort();
//
//             // Print headers
//             print!("|");
//             for v in &vars {
//                 print!(" {} |", v)
//             }
//             println!(" = |");
//             println!("{}|", "|---".repeat(vars.len() + 1));
//
//             // Print rows
//             let total_combinations = 1 << vars.len();
//             for i in 0..total_combinations {
//                 let mut values = HashSet::with_capacity(vars.len());
//
//                 // Build the combination using bit operations
//                 for (j, &var) in vars.iter().enumerate() {
//                     if i & (1 << (vars.len() - 1 - j)) != 0 {
//                         values.insert(var);
//                     }
//                 }
//
//                 if let Some(ref root) = ast.root {
//                     let result = ast.evaluate(&root, &values);
//
//                     print!("|");
//                     for v in &vars {
//                         print!(" {} |", if values.contains(v) { "1" } else { "0" });
//                     }
//                     println!(" {} |", if result { "1" } else { "0" });
//                 }
//             }
//         },
//         Err(err) => eprintln!("Error occurred while evaluating: {}", err),
//     }
// }
