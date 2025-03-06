use crate::adder;

pub fn multiplier(a: u32, b: u32) -> u32 {
    let mut result: u32 = 0;
    let mut multiplicand: u32 = a;
    let mut multiplier: u32 = b;

    while multiplier > 0 {
        if (multiplier & 1) == 1 {
            result = adder(result, multiplicand);
        }

        multiplicand = multiplicand << 1;
        multiplier = multiplier >> 1;
    }

    return result;
}
