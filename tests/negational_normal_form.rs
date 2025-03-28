use ready_set_boole::negation_normal_form;

#[test]
fn test_double_negation() {
    let formula = "A!!";
    let nnf_ast = negation_normal_form(formula);
    assert_eq!(nnf_ast, "A");
}

#[test]
fn test_material_condition() {
    let formula = "AB>";
    let nnf_ast = negation_normal_form(formula);
    assert_eq!(nnf_ast, "A!B|");
}

#[test]
fn test_de_morgan_and() {
    let formula = "AB&!";
    let nnf_ast = negation_normal_form(formula);
    assert_eq!(nnf_ast, "A!B!|");
}

#[test]
fn test_de_morgan_or() {
    let formula = "AB|!";
    let nnf_ast = negation_normal_form(formula);
    assert_eq!(nnf_ast, "A!B!&");
}

#[test]
fn test_equivalence_to_nnf() {
    let formula = "AB=";
    let nnf_ast = negation_normal_form(formula);
    assert_eq!(nnf_ast, "A!B|B!A|&");
}

#[test]
fn test_xor_to_nnf() {
    let formula = "AB^";
    let nnf_ast = negation_normal_form(formula);
    assert_eq!(nnf_ast, "AB!&A!B&|");
}

