use std::collections::HashSet;

use crate::ast::Ast;

pub fn print_truth_table(formula: &str) {
    let ast = match Ast::from_formula(formula) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Error occurred while evaluating: {err:?}");
            return;
        },
    };

    let vars_set = ast.variables();
    let mut vars: Vec<char> = vars_set.into_iter().collect();
    vars.sort();

    print_table_header(&vars);

    let mut values = HashSet::with_capacity(vars.len());

    // Print rows
    let row_count = 1 << vars.len();
    if let Some(ref root) = ast.root {
        for row in 0..row_count {
            values.clear();

            // Build the combination
            for (col, &var) in vars.iter().enumerate() {
                if row & (1 << (vars.len() - 1 - col)) != 0 {
                    values.insert(var);
                }
            }

            let result = ast.evaluate(&root, &values);

            print!("|");
            for v in &vars {
                print!(" {} |", if values.contains(v) { "1" } else { "0" });
            }
            println!(" {} |", if result { "1" } else { "0" });
        }
    }
}

fn print_table_header(vars: &[char]) {
    print!("|");
    for v in vars {
        print!(" {} |", v)
    }
    println!(" = |");
    println!("{}|", "|---".repeat(vars.len() + 1));
}

