use std::collections::HashSet;

use crate::Expression;

// m = formula length
// v = variable count
// Space complexity: O(m + v)
// Time complexity: O(2^v . (v + m))
//   We generate 2^v rows - ex 2 variables -> 2^2 = 4 rows
//   For each row: determine variables O(v) & evaluate which is O(m)
pub fn print_truth_table(formula: &str) {
    let expression = match Expression::from_formula(formula) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Error occurred while evaluating: {err:?}");
            return;
        },
    };

    let vars_set = expression.variables();
    let mut vars: Vec<char> = vars_set.into_iter().collect();
    vars.sort();

    print_table_header(&vars);

    let mut values = HashSet::with_capacity(vars.len());

    let row_count = 1 << vars.len();
    for row in 0..row_count {
        values.clear();

        // Build the combination
        for (col, &var) in vars.iter().enumerate() {
            if row & (1 << (vars.len() - 1 - col)) != 0 {
                values.insert(var);
            }
        }

        let result = expression.evaluate(&values);

        print!("|");
        for v in &vars {
            print!(" {} |", if values.contains(v) { "1" } else { "0" });
        }
        println!(" {} |", if result { "1" } else { "0" });
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

