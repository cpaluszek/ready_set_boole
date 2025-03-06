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
