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

## Ex02 - Gray code
- [Gray code - wikipedia](https://en.wikipedia.org/wiki/Gray_code#Constructing_an_n-bit_Gray_code)
Is an ordering of binary numbers where two successive values differ in only one bit, useful for reducing state transitions.

**Binary to Gray conversion**
1. MSB (Most Significant Bit): The most significant bit (MSB) remains the same in both binary and Gray code.
2. Other Bits: For each subsequent bit, the Gray code bit is determined by the XOR of the current bit and the previous bit in the binary representation.

![Binary to gray](https://media.geeksforgeeks.org/wp-content/uploads/20220420085103/Screenshot695-300x191.png)

## Ex03 - Boolean Evaluation

Evaluates propositional formulas in Reverse Polish Notation (RPN) using an Abstract Syntax Tree (AST) for visualization and binary tree representation.
- [Reverse Polish Notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation)
- [Abstract Syntax Tree](https://en.wikipedia.org/wiki/Abstract_syntax_tree)

| Symbol | Mathematical equivalent | Description           |
|--------|-------------------------|-----------------------|
|  `0`   | `⊥`                     | false                 |
|  `1`   | `⊤`                     | true                  |
|  `!`   | `¬`                     | Negation              |
|  `&`   | `∧`                     | Conjunction           |
|  `│`   | `∨`                     | Disjunction           |
|  `ˆ`   | `⊕`                     | Exclusive disjunction |
|  `>`   | `⇒`                     | Material condition    |
|  `=`   | `⇔`                     | Logical equivalence   |


## Ex04 - Truth Table

Generates a truth table for a given Boolean expression. A truth table lists all possible input values and their corresponding output values based on logical operations.

**Example**

Input Expression: `(A ∧ B) ∨ C`

| A | B | C | = |
|---|---|---|---|
| 0 | 0 | 0 | 0 |
| 0 | 0 | 1 | 1 |
| 0 | 1 | 0 | 0 |
| 0 | 1 | 1 | 1 |
| 1 | 0 | 0 | 0 |
| 1 | 0 | 1 | 1 |
| 1 | 1 | 0 | 1 |
| 1 | 1 | 1 | 1 |

Each row represents a unique combination of variable values, and the columns show intermediate and final results.

## Ex05 - Negation Normal Form
**WHAT IS THE PURPOSE OF THIS FORM??**

- [Negation Normal Form](https://en.wikipedia.org/wiki/Negation_normal_form)

A formula is in NNF if the negation operator is only applied to variables or conjuction (∧, AND).
Every formula can be brought into this form by:
- replacing implications and equivalences by their definitions
- using [De Morgan's laws](https://en.wikipedia.org/wiki/De_Morgan%27s_laws)
- eliminating double negations

This process can be represented using the following rewrite rules:
- Double negation elimination: (¬¬A) ⇔ A
- Material condition: (A ⇒ B) ⇔ (¬A ∨ B)
- Equivalence: (A ⇔ B) ⇔ ((A ⇒ B) ∧ (B ⇒ A))
- De Morgan's laws: 
    - ¬(A ∨ B) ⇔ (¬A ∧ ¬B)
    - ¬(A ∧ B) ⇔ (¬A ∨ ¬B)
- Distributivity:
    - (A ∧ (B ∨ C)) ⇔ ((A ∧ B) ∨ (A ∧ C))
    - (A ∨ (B ∧ C)) ⇔ ((A ∨ B) ∧ (A ∨ C))
