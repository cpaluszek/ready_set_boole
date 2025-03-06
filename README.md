# Ready set boole

The only operations allowed to use are:
- & (bitwise AND)
- | (bitwise OR)
- ^ (bitwise XOR)
- << (left shift)
- >> (right shift)
- = (assignment)
- ==, !=, <, >, <=, >= (comparison operators)


## Ex00 - Adder

`fn adder(a: u32, b: u32) -> u32`

**Pseudocode:**
```python
while b != 0:
    carry = a & b
    a = a ^ b
    b = carry << 1
```

**Explanation:**
- AND: `a & b` finds the positions where both bits are 1, indicating a carry is needed
- XOR: `a ^ b` adds the bits wihout considering the carry
- Left Shift: `carry << 1` shifts the carry

## Ex01 - Multiplier

`fn multiplier(a: u32, b: u32) -> u32`

**Pseudocode:**
```python
result = 0
multiplicand = a
multiplier = b

while multiplier != 0:
    if multiplier & 1 == 1:
        result = result + multiplicand
    multiplicand = multiplicand << 1
    multiplier = multiplier >> 1

```

Explanation:
- Bitwise AND: multiplier & 1 checks if the least significant bit of the multiplier is 1. If it is, the current value of multiplicand is added to result.
- Addition: result = result + multiplicand accumulates the result by adding the multiplicand whenever the corresponding bit in the multiplier is 1.
- Left Shift: multiplicand << 1 shifts the multiplicand to the left by one bit, effectively multiplying it by 2.
- Right Shift: multiplier >> 1 shifts the multiplier to the right by one bit, effectively dividing it by 2.

![binary multiplication](https://mathmonks.com/wp-content/uploads/2024/02/Binary-Multiplication.jpg)
