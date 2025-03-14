use std::collections::HashSet;
use crate::evaluate::build_ast;

pub fn sat(formula: &str) -> bool {
    match build_ast(formula) {
        Ok(ast) => {
            if let Some(ref root) = ast.root {
                let vars_set = ast.variables();
                let mut vars: Vec<char> = vars_set.into_iter().collect();
                vars.sort();

                let total_combinations = 1 << vars.len();
                for i in 0..total_combinations {
                    let mut values = HashSet::with_capacity(vars.len());

                    // Build the combination using bit operations
                    for (j, &var) in vars.iter().enumerate() {
                        if i & (1 << (vars.len() - 1 - j)) != 0 {
                            values.insert(var);
                        }
                    }

                    if ast.evaluate(&root, &values) {
                        return true;
                    }
                }
            }
        },
        Err(err) => eprintln!("Error occurred while evaluating: {}", err),
    }
    return false;
}
