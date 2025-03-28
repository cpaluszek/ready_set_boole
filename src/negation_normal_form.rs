use crate::Expression;

pub fn negation_normal_form(formula: &str) -> String {
    let expression = match Expression::from_formula(formula) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Error occurred while evaluating: {err:?}");
            return String::new();
        }
    };

    normalize(&expression).to_rpn()
}

// Double negation: ¬¬A ⇔ A
// XOR: A ⊕ B ⇔ (A ∧ ¬B) ∨ (¬A ∧ B)
// Equivalence: A ⇔ B ⇔ (A ⇒ B) ∧ (B ⇒ A) ⇔ (¬A ∨ B) ∧ (¬B ∨ A)
// Implication: A ⇒ B ⇔ ¬A ∨ B
fn normalize(expr: &Expression) -> Expression {
    match expr {
        Expression::Val(_) | Expression::Var(_) => expr.clone(),
        Expression::Neg(e) => negate(e),
        Expression::And(a, b) => normalize(a) & normalize(b),
        Expression::Or(a, b) => normalize(a) | normalize(b),
        Expression::Xor(a, b) => (normalize(a) & negate(b)) | (negate(a) & normalize(b)),
        Expression::Implication(a, b) => negate(a) | normalize(b),
        Expression::Equivalence(a, b) => (negate(a) | normalize(b)) & (negate(b) | normalize(a)),
    }
}

// De Morgan's laws:
// ¬(A ∧ B) ⇔ ¬A ∨ ¬B
// ¬(A ∨ B) ⇔ ¬A ∧ ¬B
// Double negation: ¬¬A ⇔ A
// Negated XOR: ¬(A ⊕ B) ⇔ (A ∧ B) ∨ (¬A ∧ ¬B)
// Negated implication: ¬(A ⇒ B) ⇔ A ∧ ¬B
// Negated equivalence: ¬(A ⇔ B) ⇔ (A ∧ ¬B) ∨ (¬A ∧ B)
fn negate(expr: &Expression) -> Expression {
    match expr {
        Expression::Val(x) => Expression::val(!x),
        Expression::Var(x) => !Expression::var(*x),
        Expression::Neg(e) => normalize(e),
        Expression::And(a, b) => negate(a) | negate(b),
        Expression::Or(a, b) => negate(a) & negate(b),
        Expression::Xor(a, b) => (normalize(a) & normalize(b)) | (negate(a) & negate(b)),
        Expression::Implication(a, b) => normalize(a) & negate(b),
        Expression::Equivalence(a, b) => (normalize(a) & negate(b)) | (negate(a) & normalize(b)),
    }
}

