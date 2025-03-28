use std::{collections::HashSet, fmt};

use crate::{pop_from_stack, LogicError};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Val(bool),
    Var(char),
    Neg(Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Xor(Box<Expression>, Box<Expression>),
    Implication(Box<Expression>, Box<Expression>),
    Equivalence(Box<Expression>, Box<Expression>),
}

impl Expression {
    pub fn from_formula(formula: &str) -> Result<Self, LogicError> {
        let mut stack: Vec<Expression> = Vec::with_capacity(formula.len());

        for character in formula.chars() {
            match character {
                // Constants
                '1' => stack.push(Expression::Val(true)),
                '0' => stack.push(Expression::Val(false)),

                // Variables
                'A'..'Z' => stack.push(Expression::Var(character)),

                // Negation
                '!' => {
                    let operand = pop_from_stack(&mut stack)?;
                    stack.push(!operand);
                },

                // Binary operators - using the helper function
                '&' => handle_binary_op(&mut stack, Expression::And)?,
                '|' => handle_binary_op(&mut stack, Expression::Or)?,
                '^' => handle_binary_op(&mut stack, Expression::Xor)?,
                '>' => handle_binary_op(&mut stack, Expression::Implication)?,
                '=' => handle_binary_op(&mut stack, Expression::Equivalence)?,

                // Unrecognized character
                _ => return Err(LogicError::UnrecognizedSymbol),
            }
        }

        // Make sure we have exactly one element in the stack (the full expression)
        if stack.len() != 1 {
            return Err(LogicError::IncompleteFormula);
        }

        let root_expr = stack.pop().unwrap(); // Safe because we just checked stack.len() == 1
        Ok(root_expr)
    }

    pub fn evaluate(&self, values: &HashSet<char>) -> bool {
        match self {
            Expression::Val(val) => *val,
            Expression::Var(c) => values.contains(c),
            Expression::Neg(child) => !child.evaluate(values),
            Expression::And(left, right) => left.evaluate(values) && right.evaluate(values),
            Expression::Or(left, right) => left.evaluate(values) || right.evaluate(values),
            Expression::Xor(left, right) => left.evaluate(values) != right.evaluate(values),
            Expression::Implication(left, right) => !left.evaluate(values) || right.evaluate(values),
            Expression::Equivalence(left, right) => left.evaluate(values) == right.evaluate(values),
        }
    }

    pub fn to_unicode(&self) -> char {
        match self {
            Expression::Val(true) => '1',
            Expression::Val(false) => '0',
            Expression::Var(c) => *c,
            Expression::Neg(_) => '!',
            Expression::And(_, _) => '&',
            Expression::Or(_, _) => '|',
            Expression::Xor(_, _) => '^',
            Expression::Implication(_, _) => '>',
            Expression::Equivalence(_, _) => '=',
        }
    }

    pub fn is_variable(&self) -> bool {
        matches!(self, Expression::Var(_))
    }

    // Construction Functions
    // TODO: check for this constructor usage in truth_table + eval + ast
    pub fn var(c: char) -> Expression {
        Expression::Var(c)
    }

    pub fn val(b: bool) -> Expression {
        Expression::Val(b)
    }

    pub fn not(expr: impl Into<Expression>) -> Expression {
        Expression::Neg(Box::new(expr.into()))
    }

    pub fn and(a: impl Into<Expression>, b: impl Into<Expression>) -> Expression {
        Expression::And(Box::new(a.into()), Box::new(b.into()))
    }
 
    pub fn or(a: impl Into<Expression>, b: impl Into<Expression>) -> Expression {
        Expression::Or(Box::new(a.into()), Box::new(b.into()))
    }

    pub fn xor(a: impl Into<Expression>, b: impl Into<Expression>) -> Expression {
        Expression::Xor(Box::new(a.into()), Box::new(b.into()))
    }

    pub fn implies(a: impl Into<Expression>, b: impl Into<Expression>) -> Expression {
        Expression::Implication(Box::new(a.into()), Box::new(b.into()))
    }

    pub fn equiv(a: impl Into<Expression>, b: impl Into<Expression>) -> Expression {
        Expression::Equivalence(Box::new(a.into()), Box::new(b.into()))
    }

    pub fn to_rpn(&self) -> String {
        match self {
            Expression::Val(_) | Expression::Var(_) => self.to_unicode().to_string(),
            Expression::Neg(child) => {
                let mut result = child.to_rpn();
                result.push(Expression::Neg(Box::new(Expression::Val(false))).to_unicode());
                result
            },
            Expression::And(left, right) |
            Expression::Or(left, right) |
            Expression::Xor(left, right) |
            Expression::Implication(left, right) |
            Expression::Equivalence(left, right) => {
                let mut result = left.to_rpn();
                result.push_str(&right.to_rpn());
                result.push(self.to_unicode());
                result
            }
        }
    }

    pub fn variables(&self) -> HashSet<char> {
        let mut vars = HashSet::new();
        self.collect_variables(&mut vars);
        return vars;
    }

    fn collect_variables(&self, vars: &mut HashSet<char>) {
        match self {
            Expression::Var(c) => {
                vars.insert(*c);
            },
            Expression::And(left, right) |
            Expression::Or(left, right) |
            Expression::Xor(left, right) |
            Expression::Implication(left, right) |
            Expression::Equivalence(left, right) => {
                left.collect_variables(vars);
                right.collect_variables(vars);
            },
            Expression::Neg(child) => {
                child.collect_variables(vars);
            },
            Expression::Val(_) => {},
        }
    }

    fn height(&self) -> usize {
        match self {
            Expression::And(left, right) |
            Expression::Or(left, right) |
            Expression::Xor(left, right) |
            Expression::Implication(left, right) |
            Expression::Equivalence(left, right) => {
                let left_height = left.height();
                let right_height = right.height();
                1 + std::cmp::max(left_height, right_height)
            }
            Expression::Neg(child) => {
                1 + child.height()
            }
            Expression::Val(_) | Expression::Var(_) => 1,
        }
    }

    fn width(&self) -> usize {
        match self {
            Expression::And(left, right) |
            Expression::Or(left, right) |
            Expression::Xor(left, right) |
            Expression::Implication(left, right) |
            Expression::Equivalence(left, right) => {
                let left_width = left.width();
                let right_width = right.width();
                left_width + right_width
            }
            Expression::Neg(child) => {
                child.height() // This is the same as before, using height for width
            }
            Expression::Val(_) | Expression::Var(_) => 1,
        }
    }

    fn draw_tree(&self, buffer: &mut Vec<Vec<char>>, row: usize, col: usize) {
        match self {
            Expression::And(left, right) |
            Expression::Or(left, right) |
            Expression::Xor(left, right) |
            Expression::Implication(left, right) |
            Expression::Equivalence(left, right) => {
                // Place operator
                buffer[row][col] = self.to_unicode();

                // Calculate positions for children
                let left_width = left.width();
                let right_width = right.width();

                let left_col = col - left_width / 2 - 1;
                let right_col = col + right_width / 2 + 1;

                // Draw connections
                if row + 1 < buffer.len() {
                    buffer[row + 1][col - 1] = '/';
                    buffer[row + 1][col + 1] = '\\';
                }

                // Draw children
                left.draw_tree(buffer, row + 2, left_col);
                right.draw_tree(buffer, row + 2, right_col);
            },
            Expression::Neg(child) => {
                // Place operator
                buffer[row][col] = self.to_unicode();
                buffer[row + 1][col] = '|';

                // Draw the single child
                child.draw_tree(buffer, row + 2, col);
            },
            Expression::Val(_) | Expression::Var(_) => {
                buffer[row][col] = self.to_unicode();
            }
        }
    }

    fn visualize_tree(&self) -> String {
        let height = self.height() * 2;
        let width = self.width() * 4;

        let mut buffer = vec![vec![' '; width]; height];

        self.draw_tree(&mut buffer, 0, width / 2);

        let mut result = String::with_capacity(height * (width + 1));
        for row in buffer {
            // Find the last non-space character
            let last_non_space = row.iter().rposition(|&c| c != ' ').unwrap_or(0);

            // Only append up to the last non-space character
            for i in 0..=last_non_space {
                result.push(row[i]);
            }
            result.push('\n');
        }

        result
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.visualize_tree())
    }
}

pub fn build_and_print_expression(formula: &str) {
    match Expression::from_formula(formula) {
        Ok(expr) => println!("{}", expr),
        Err(_) => {},
    }
}

impl std::ops::BitAnd for Expression {
    type Output = Self;
 
    fn bitand(self, rhs: Self) -> Self {
        Expression::And(Box::new(self), Box::new(rhs))
    }
}

impl std::ops::BitOr for Expression {
    type Output = Self;
 
    fn bitor(self, rhs: Self) -> Self {
        Expression::Or(Box::new(self), Box::new(rhs))
    }
}

impl std::ops::Not for Expression {
    type Output = Self;
 
    fn not(self) -> Self {
        Expression::Neg(Box::new(self))
    }
}

fn handle_binary_op(
    stack: &mut Vec<Expression>, 
    constructor: fn(Box<Expression>, Box<Expression>) -> Expression
) -> Result<(), LogicError> {
    let right = pop_from_stack(stack)?;
    let left = pop_from_stack(stack)?;
    stack.push(constructor(Box::new(left), Box::new(right)));
    Ok(())
}
