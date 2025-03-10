use ready_set_boole::multiplier;

#[test]
fn simple_multiplication() {
    for a in 0..u8::MAX as u32 {
        for b in 0..u8::MAX as u32 {
            assert_eq!(multiplier(a, b), a.wrapping_mul(b));
        }
    }

}

#[test]
fn max_multiplication() {
    let a = u32::MAX;
    let b = 1;
    let result = multiplier(a, b);
    assert_eq!(result, a * b);
}

