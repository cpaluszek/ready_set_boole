use ready_set_boole::multiplier;

#[test]
fn simple_multiplication() {
    let a = 5;
    let b = 3;
    let result = multiplier(a, b);
    assert_eq!(result, a * b);
}

#[test]
fn zero_multiplication() {
    let a = 5;
    let b = 0;
    let result = multiplier(a, b);
    assert_eq!(result, a * b);
}

#[test]
fn large_multiplication() {
    let a = 1_234_567;
    let b = 2;
    let result = multiplier(a, b);
    assert_eq!(result, a * b);
}

#[test]
fn max_multiplication() {
    let a = u32::MAX;
    let b = 1;
    let result = multiplier(a, b);
    assert_eq!(result, a * b);
}
