use std::{collections::HashSet, fmt};

use crate::symbol::LogicalSymbol;

#[derive(Debug, Clone)]
pub enum AstNode {
    Operator(LogicalSymbol, Option<Box<AstNode>>, Option<Box<AstNode>>),
    Operand(LogicalSymbol),
}

#[derive(Debug, Clone)]
pub struct Ast {
    pub root: Option<Box<AstNode>>,
}

impl Ast {
    pub fn new(root: AstNode) -> Self {
        Ast {
            root: Some(Box::new(root)),
        }
    }

    fn height(&self, node: &AstNode) -> usize {
        match node {
            AstNode::Operator(_, Some(left), Some(right)) => {
                let left_height = self.height(left);
                let right_height = self.height(right);
                1 + std::cmp::max(left_height, right_height)
            }
            AstNode::Operator(_, Some(left), None) => {
                let left_height = self.height(left);
                1 + left_height
            }
            AstNode::Operand(_) => 1,
            _ => 0,
        }
    }

    fn width(&self, node: &AstNode) -> usize {
        match node {
            AstNode::Operator(_, Some(left), Some(right)) => {
                let left_width = self.width(left);
                let right_width = self.width(right);
                left_width + right_width
            }
            AstNode::Operator(_, Some(left), None) => {
                let left_width = self.width(left);
                left_width
            }
            AstNode::Operand(_) => 1,
            _ => 0,
        }
    }

    pub fn variables(&self) -> HashSet<char> {
        let mut vars = HashSet::new();
        if let Some(root) = &self.root {
            self.collect_variables(&root, &mut vars);
        }
        return vars;
    }

    fn collect_variables(&self, node: &AstNode, vars: &mut HashSet<char>) {
        match node {
            AstNode::Operand(symbol) => {
                if symbol.is_variable() {
                    vars.insert(symbol.to_unicode());
                }
            },
            AstNode::Operator(_, left, right) => {
                if let Some(l) = left {
                    self.collect_variables(l, vars);
                }
                if let Some(r) = right {
                    self.collect_variables(r, vars);
                }
            }
        }
    }

    fn draw_tree(&self, node: &AstNode, buffer: &mut Vec<Vec<char>>, row: usize, col: usize) {
        match node {
            AstNode::Operator(op, Some(left), Some(right)) => {
                // Place operator
                buffer[row][col] = op.to_unicode();

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
            AstNode::Operator(op, Some(left), None) => {
                // Place operator
                buffer[row][col] = op.to_unicode();
                buffer[row + 1][col] = '|';

                // Draw the single child (for unary operators like negation)
                self.draw_tree(left, buffer, row + 2, col);
            },
            AstNode::Operand(val) => {
                buffer[row][col] = val.to_unicode();
            }
            _ => return,
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

    pub fn evaluate(&self, node: &AstNode, values: &HashSet<char>) -> bool {
        match node {
            AstNode::Operand(symbol) => {
                if symbol.is_variable() {
                    return values.contains(&symbol.to_unicode());
                } else {
                    return symbol == &LogicalSymbol::True;
                }
            },
            AstNode::Operator(symbol, left, right) => {
                let left_val = left.as_ref().map(|l| self.evaluate(l, values)).unwrap_or(false);
                let right_val = right.as_ref().map(|r| self.evaluate(r, values)).unwrap_or(false);
                match symbol {
                    LogicalSymbol::Negation => !left_val,
                    LogicalSymbol::Conjunction => left_val && right_val,
                    LogicalSymbol::Disjunction => left_val || right_val,
                    LogicalSymbol::ExclusiveOr => left_val ^ right_val,
                    LogicalSymbol::Implication => !left_val || right_val,
                    LogicalSymbol::Equivalence => left_val == right_val,
                    _ => unreachable!(),
                }
            }
        }
    }
}

impl fmt::Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.visualize_tree())
    }
}
