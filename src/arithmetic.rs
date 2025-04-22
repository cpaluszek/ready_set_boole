// Space complexity: O(1) - 3 variables regardless of the input size
// Time complexity: O(log N)
//   The number of iteration depends on how many times we shift the carry
//   At most 32 operations
//   For any number N, its binary representation has roughly log2(N) bits
//   Worst case: 0xFFFFFFFF + 0x1

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

// Space complexity: O(1)
// Time complexity: O((log N)^2) - The loop runs log2(b) times
//   Each loop calls adder which is also log(N)
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

// Space complexity: O(1)
// Time complexity: O(1)
pub fn gray_code(n: u32) -> u32 {
    n ^ (n >> 1)
}
