use ready_set_boole::evaluate::eval_formula;

#[test]
fn test_eval_formula() {
    assert_eq!(eval_formula("1"), true);
    assert_eq!(eval_formula("0"), false);
    assert_eq!(eval_formula("10&"), false);
    assert_eq!(eval_formula("10|"), true);
    assert_eq!(eval_formula("11>"), true);
    assert_eq!(eval_formula("10="), false);
    assert_eq!(eval_formula("1011||="), true);
}

#[test]
fn test_incorrect_formula() {
    assert_eq!(eval_formula("|"), false);
    assert_eq!(eval_formula("10"), false);
    assert_eq!(eval_formula("10x"), false);
}
