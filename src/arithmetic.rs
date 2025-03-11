pub fn adder(a: u32, b: u32) -> u32 {
    let mut a: u32 = a;
    let mut b: u32 = b;
    let mut carry: u32;

    while b != 0 {
        // Compute the carry using AND
        carry = a & b;
        // Adds the bits without considering the carry using XOR
        a = a ^ b;
        // Shift the carry
        b = carry << 1;
    }
    return a;
}

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

pub fn gray_code(n: u32) -> u32 {
    n ^ (n >> 1)
}
