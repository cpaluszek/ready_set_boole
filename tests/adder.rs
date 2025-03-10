use std::u8;
use ready_set_boole::adder;

#[test]
fn simple_addition() {
    for a in 0..u8::MAX as u32 {
        for b in 0..u8::MAX as u32 {
            assert_eq!(adder(a, b), a.wrapping_add(b));
        }
    }
}

#[test]
fn max_value_addition() {
    let a = u32::MAX;
    let b = 1;
    let result = adder(a, b);
    assert_eq!(result, 0);
}

