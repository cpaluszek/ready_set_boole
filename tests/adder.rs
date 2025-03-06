use ready_set_boole::adder;

#[test]
fn simple_addition() {
    let a = 15;
    let b = 13;
    let result = adder(a, b);
    assert_eq!(result, a + b);
}


#[test]
fn zero_addition() {
    let a = 0;
    let b = 42;
    let result = adder(a, b);
    assert_eq!(result, a + b);
}

#[test]
fn max_value_addition() {
    let a = u32::MAX;
    let b = 1;
    let result = adder(a, b);
    assert_eq!(result, 0);
}

#[test]
fn identity_addition() {
    let a = 12345;
    let result = adder(a, a);
    assert_eq!(result, a + a);
}

#[test]
fn large_numbers_addition() {
    let a = 1_000_000_000;
    let b = 1_000_000_000;
    let result = adder(a, b);
    assert_eq!(result, a + b);
}

