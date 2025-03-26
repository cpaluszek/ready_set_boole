use std::{collections::HashSet, fmt};

use crate::{expression::Expression, LogicError};

#[derive(Debug, Clone)]
pub struct Ast {
    pub root: Option<Box<Expression>>,
}

impl Ast {
    pub fn new(root: Expression) -> Self {
        Ast {
            root: Some(Box::new(root)),
        }
    }

    pub fn from_formula(formula: &str) -> Result<Self, LogicError> {
        build_ast(formula)
    }

    fn height(&self, expr: &Expression) -> usize {
        match expr {
            Expression::Conjunction(left, right) |
            Expression::Disjunction(left, right) |
            Expression::ExclusiveOr(left, right) |
            Expression::Implication(left, right) |
            Expression::Equivalence(left, right) => {
                let left_height = self.height(left);
                let right_height = self.height(right);
                1 + std::cmp::max(left_height, right_height)
            }
            Expression::Negation(child) => {
                1 + self.height(child)
            }
            Expression::Operand(_) | Expression::Variable(_) => 1,
        }
    }

    fn width(&self, expr: &Expression) -> usize {
        match expr {
            Expression::Conjunction(left, right) |
            Expression::Disjunction(left, right) |
            Expression::ExclusiveOr(left, right) |
            Expression::Implication(left, right) |
            Expression::Equivalence(left, right) => {
                let left_width = self.width(left);
                let right_width = self.width(right);
                left_width + right_width
            }
            Expression::Negation(child) => {
                self.height(child) // This is the same as before, using height for width
            }
            Expression::Operand(_) | Expression::Variable(_) => 1,
        }
    }

    pub fn variables(&self) -> HashSet<char> {
        let mut vars = HashSet::new();
        if let Some(root) = &self.root {
            self.collect_variables(&root, &mut vars);
        }
        return vars;
    }

    fn collect_variables(&self, expr: &Expression, vars: &mut HashSet<char>) {
        match expr {
            Expression::Variable(c) => {
                vars.insert(*c);
            },
            Expression::Conjunction(left, right) |
            Expression::Disjunction(left, right) |
            Expression::ExclusiveOr(left, right) |
            Expression::Implication(left, right) |
            Expression::Equivalence(left, right) => {
                self.collect_variables(left, vars);
                self.collect_variables(right, vars);
            },
            Expression::Negation(child) => {
                self.collect_variables(child, vars);
            },
            Expression::Operand(_) => {},
        }
    }

    fn draw_tree(&self, expr: &Expression, buffer: &mut Vec<Vec<char>>, row: usize, col: usize) {
        match expr {
            Expression::Conjunction(left, right) |
            Expression::Disjunction(left, right) |
            Expression::ExclusiveOr(left, right) |
            Expression::Implication(left, right) |
            Expression::Equivalence(left, right) => {
                // Place operator
                buffer[row][col] = expr.to_unicode();

                // Calculate positions for children
                let left_width = self.width(left);
                let right_width = self.width(right);

                let left_col = col - left_width / 2 - 1;
                let right_col = col + right_width / 2 + 1;

                // Draw connections
                if row + 1 < buffer.len() {
                    buffer[row + 1][col - 1] = '/';
                    buffer[row + 1][col + 1] = '\\';
                }

                // Draw children
                self.draw_tree(left, buffer, row + 2, left_col);
                self.draw_tree(right, buffer, row + 2, right_col);
            },
            Expression::Negation(child) => {
                // Place operator
                buffer[row][col] = expr.to_unicode();
                buffer[row + 1][col] = '|';

                // Draw the single child
                self.draw_tree(child, buffer, row + 2, col);
            },
            Expression::Operand(_) | Expression::Variable(_) => {
                buffer[row][col] = expr.to_unicode();
            }
        }
    }

    fn visualize_tree(&self) -> String {
        if let Some(root) = &self.root {
            let height = self.height(root) * 2;
            let width = self.width(root) * 4;

            let mut buffer = vec![vec![' '; width]; height];

            self.draw_tree(root, &mut buffer, 0, width / 2);

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
        } else {
            String::new()
        }
    }

    fn evaluate_binary_op(&self, left: &Expression, right: &Expression, 
        values: &HashSet<char>, 
        op: fn(bool, bool) -> bool) -> bool {
        let left_val = self.evaluate(left, values);
        let right_val = self.evaluate(right, values);
        op(left_val, right_val)
    }

    pub fn evaluate(&self, expr: &Expression, values: &HashSet<char>) -> bool {
        match expr {
            Expression::Operand(val) => *val,
            Expression::Variable(c) => values.contains(c),
            Expression::Negation(child) => !self.evaluate(child, values),
            Expression::Conjunction(left, right) => 
            self.evaluate_binary_op(left, right, values, |a, b| a && b),
            Expression::Disjunction(left, right) => 
            self.evaluate_binary_op(left, right, values, |a, b| a || b),
            Expression::ExclusiveOr(left, right) => 
            self.evaluate_binary_op(left, right, values, |a, b| a != b),
            Expression::Implication(left, right) => 
            self.evaluate_binary_op(left, right, values, |a, b| !a || b),
            Expression::Equivalence(left, right) => 
            self.evaluate_binary_op(left, right, values, |a, b| a == b),
        }
    }

    pub fn to_rpn(&self) -> String {
        if let Some(ref root) = self.root {
            self.expr_to_rpn(root)
        } else {
            String::new()
        }
    }

    fn expr_to_rpn(&self, expr: &Expression) -> String {
        match expr {
            Expression::Operand(_) | Expression::Variable(_) => expr.to_unicode().to_string(),
            Expression::Negation(child) => {
                let mut result = self.expr_to_rpn(child);
                result.push(Expression::Negation(Box::new(Expression::Operand(false))).to_unicode());
                result
            },
            Expression::Conjunction(left, right) |
            Expression::Disjunction(left, right) |
            Expression::ExclusiveOr(left, right) |
            Expression::Implication(left, right) |
            Expression::Equivalence(left, right) => {
                let mut result = self.expr_to_rpn(left);
                result.push_str(&self.expr_to_rpn(right));
                result.push(expr.to_unicode());
                result
            }
        }
    }
}

impl fmt::Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.visualize_tree())
    }
}

pub fn pop_from_stack<T>(stack: &mut Vec<T>) -> Result<T, LogicError> {
    stack.pop().ok_or(LogicError::MissingArgument)
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

pub fn build_ast(formula: &str) -> Result<Ast, LogicError> {
    let mut stack: Vec<Expression> = Vec::with_capacity(formula.len());

    for character in formula.chars() {
        match character {
            // Constants
            '1' => stack.push(Expression::Operand(true)),
            '0' => stack.push(Expression::Operand(false)),

            // Variables (a-z, A-Z)
            c if c.is_alphabetic() => stack.push(Expression::Variable(c)),

            // Negation
            '!' => {
                let operand = pop_from_stack(&mut stack)?;
                stack.push(Expression::Negation(Box::new(operand)));
            },

            // Binary operators - using the helper function
            '&' => handle_binary_op(&mut stack, Expression::Conjunction)?,
            '|' => handle_binary_op(&mut stack, Expression::Disjunction)?,
            '^' => handle_binary_op(&mut stack, Expression::ExclusiveOr)?,
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
    let formula_ast = Ast::new(root_expr);
    Ok(formula_ast)
}

pub fn build_and_print_ast(formula: &str) {
    match build_ast(formula) {
        Ok(ast) => println!("{}", ast),
        Err(_) => {},
    }
}
