use crate::{normalize, Expression};

// Space complexity: O(2^m):
//      Expression tree - O(m)
//      NNF Expression - O(m)
//      Call stack - O(m)
//      CNF expression - O(2^m) - can be exponentially larger than the input
// Time complexity:  O(2^m)
//   Expression parsing: O(m)
//   NNF Conversion: O(m)
//   CNF conversion: O(2^m)
//      AND op: 2 recursive calls
//      OR op: distributing can lead to exponential growth
//      Flattening: O(m)
pub fn conjunctive_normal_form(formula: &str) -> String {
    let expression = match Expression::from_formula(formula) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Error occurred while evaluating: {err:?}");
            return String::new();
        }
    };
    // First convert to nnf
    let nnf = normalize(&expression);
    let cnf = to_cnf(&nnf);
    flatten_expression(&cnf).to_rpn()
}

fn to_cnf(expr: &Expression) -> Expression {
    match expr {
        Expression::Val(_) | Expression::Var(_) | Expression::Neg(_) => expr.clone(),
        Expression::And(a, b) => to_cnf(a) & to_cnf(b),
        Expression::Or(a, b) => {
            let a_cnf = to_cnf(a);
            let b_cnf = to_cnf(b);

            // Apply distributive law: A ∨ (B ∧ C) ⇔ (A ∨ B) ∧ (A ∨ C)
            match(&a_cnf, &b_cnf) {
                // if right side is AND, distributive: A ∨ (B ∧ C) ⇔ (A ∨ B) ∧ (A ∨ C)
                (_, Expression::And(b1, b2)) => {
                    // use to_cnf on both sides ?
                    to_cnf(  &((a_cnf.clone() | to_cnf(b1)) & (a_cnf | to_cnf(b2))) )
                },
                // If left side is AND, distribute: (A ∧ B) ∨ C ⇔ (A ∨ C) ∧ (B ∨ C)
                (Expression::And(a1, a2), _) => {
                    to_cnf( &((to_cnf(a1) | b_cnf.clone()) & (to_cnf(a2) | b_cnf)))
                },
                // Base case: both sides are litterals in CNF form
                _ => a_cnf | b_cnf,
            }
        },
        // These expressions should not occur after calling normalize()
        Expression::Xor(_, _) => panic!("Unexpected XOR in CNF conversion"),
        Expression::Implication(_, _) => panic!("Unexpected IMPLICATION in CNF conversion"),
        Expression::Equivalence(_, _) => panic!("Unexpected EQUIVALENCE in CNF conversion"),
    }
}

fn flatten_expression(expr: &Expression) -> Expression {
    match expr {
        Expression::And(_, _) => {
            let mut operands = Vec::new();
            collect_operands(expr, &mut operands, true);

            // create a righ-associative tree of AND
            let mut result = operands.pop().unwrap();
            while !operands.is_empty() {
                result = operands.pop().unwrap() & result;
            }
            result
        },
        Expression::Or(_, _) => {
            let mut operands = Vec::new();
            collect_operands(expr, &mut operands, false);

            // create a righ-associative tree of OR
            let mut result = operands.pop().unwrap();
            while !operands.is_empty() {
                result = operands.pop().unwrap() | result;
            }
            result
        }
        _ => expr.clone(),
    }
}

fn collect_operands(expr: &Expression, operands: &mut Vec<Expression>, is_and: bool) {
    match (expr, is_and) {
        (Expression::And(a, b), true) => {
            collect_operands(a, operands, is_and);
            collect_operands(b, operands, is_and);
        },
        (Expression::Or(a, b), false) => {
            collect_operands(a, operands, is_and);
            collect_operands(b, operands, is_and);
        },
        _ => operands.push(flatten_expression(expr)),
    }
}
