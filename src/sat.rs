use std::collections::HashSet;

use crate::Expression;

pub fn sat(formula: &str) -> bool {
    let expression = match Expression::from_formula(formula) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Error occurred while evaluating: {err:?}");
            return false;
        }
    };

    let vars_set = expression.variables();
    let vars: Vec<char> = vars_set.into_iter().collect();

    let mut values = HashSet::with_capacity(vars.len());
    let combination_count = 1 << vars.len();
    for i in 0..combination_count {
        values.clear();

        for (j, &var) in vars.iter().enumerate() {
            if i & (1 << (vars.len() - 1 - j)) != 0 {
                values.insert(var);
            }
        }

        if expression.evaluate(&values) {
            return true;
        }
    }
    false
}
