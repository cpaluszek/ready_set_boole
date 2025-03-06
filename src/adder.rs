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
