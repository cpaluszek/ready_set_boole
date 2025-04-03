use ready_set_boole::sat::sat;

#[test]
fn test_simple_sat() {
    assert_eq!(sat("AB|"), true);
    assert_eq!(sat("AB&"), true);
    assert_eq!(sat("A"), true);
    assert_eq!(sat("AB^"), true);
    assert_eq!(sat("AB>"), true);
    assert_eq!(sat("AB="), true);
}

#[test]
fn test_simple_unsat() {
    assert_eq!(sat("AA!&"), false);
    assert_eq!(sat("AA^"), false);
    assert_eq!(sat("A!A&"), false);
}

#[test]
fn test_complex_sat() {
    assert_eq!(sat("ABC||"), true);
    assert_eq!(sat("ABC&&"), true);
    assert_eq!(sat("AB|C&"), true);
    assert_eq!(sat("AB&C|"), true);
    assert_eq!(sat("AB&!C!|"), true);
    assert_eq!(sat("AB>C>"), true);
}

#[test]
fn test_tautologies() {
    assert_eq!(sat("AA|"), true);
    assert_eq!(sat("AA="), true);
    assert_eq!(sat("AA>"), true);
    assert_eq!(sat("AA!|"), true);
}

#[test]
fn test_contradictions() {
    assert_eq!(sat("AA!&"), false);
    assert_eq!(sat("AA!&BB!&&&"), false);
}

#[test]
fn test_more_variables() {
    assert_eq!(sat("ABCD&&&"), true);
    assert_eq!(sat("ABCD|||"), true);
    assert_eq!(sat("AB&CD&|"), true);
    assert_eq!(sat("AB&C&D&A!&"), false);
}

#[test]
fn test_edge_cases() {
    assert_eq!(sat(""), false);
    assert_eq!(sat("A!"), true);
    assert_eq!(sat("Z"), true);
}

