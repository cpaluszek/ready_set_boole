use ready_set_boole::conjunctive_normal_form;

#[test]
fn test_negation_of_conjunction() {
    // Test for !(A & B) = !A | !B (De Morgan's law)
    let formula = "AB&!";
    let expected = "A!B!|";
    assert_eq!(conjunctive_normal_form(formula), expected);
}

#[test]
fn test_negation_of_disjunction() {
    // Test for !(A | B) = !A & !B (De Morgan's law)
    let formula = "AB|!";
    let expected = "A!B!&";
    assert_eq!(conjunctive_normal_form(formula), expected);
}

#[test]
fn test_disjunction_with_conjunction() {
    // Test for (A | B) & C = (A | B) & C (already in CNF)
    let formula = "AB|C&";
    let expected = "AB|C&";
    assert_eq!(conjunctive_normal_form(formula), expected);
}

#[test]
fn test_multiple_disjunctions() {
    // Test for A | B | C | D = A | B | C | D (already in CNF)
    let formula = "AB|C|D|";
    let expected = "ABCD|||";
    assert_eq!(conjunctive_normal_form(formula), expected);
}

#[test]
fn test_multiple_conjunctions() {
    // Test for A & B & C & D = A & B & C & D (already in CNF)
    let formula = "AB&C&D&";
    let expected = "ABCD&&&";
    assert_eq!(conjunctive_normal_form(formula), expected);
}

#[test]
fn test_negated_conjunction_with_negated_var() {
    // Test for !(A & B) | !C = !A | !B | !C (distributive law)
    let formula = "AB&!C!|";
    let expected = "A!B!C!||";
    assert_eq!(conjunctive_normal_form(formula), expected);
}

#[test]
fn test_negated_disjunction_with_negated_var() {
    // Test for !(A | B) & !C = !A & !B & !C
    let formula = "AB|!C!&";
    let expected = "A!B!C!&&";
    assert_eq!(conjunctive_normal_form(formula), expected);
}

#[test]
fn test_implication() {
    // Test for A => B = !A | B
    let formula = "AB>";
    let expected = "A!B|";
    assert_eq!(conjunctive_normal_form(formula), expected);
}

#[test]
fn test_equivalence() {
    // Test for A <=> B = (!A | B) & (!B | A) 
    let formula = "AB=";
    let expected = "A!B|B!A|&";
    assert_eq!(conjunctive_normal_form(formula), expected);
}

#[test]
fn test_complex_distribution() {
    // Test for A | (B & C) = (A | B) & (A | C) (distributive law)
    let formula = "ABC&|";
    let expected = "AB|AC|&";
    assert_eq!(conjunctive_normal_form(formula), expected);
}

#[test]
fn test_double_negation() {
    // Test for !!A = A
    let formula = "A!!";
    let expected = "A";
    assert_eq!(conjunctive_normal_form(formula), expected);
}

