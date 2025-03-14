use std::{collections::HashSet, fmt};

use crate::symbol::LogicalSymbol;

#[derive(Debug, Clone)]
pub enum AstNode {
    Operator(LogicalSymbol, Box<AstNode>, Box<AstNode>),
    Operand(LogicalSymbol),
    Negation(Box<AstNode>),
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
            AstNode::Operator(_, left, right) => {
                let left_height = self.height(left);
                let right_height = self.height(right);
                1 + std::cmp::max(left_height, right_height)
            }
            AstNode::Negation(child) => {
                let child_height = self.height(child);
                1 + child_height
            }
            AstNode::Operand(_) => 1,
        }
    }

    fn width(&self, node: &AstNode) -> usize {
        match node {
            AstNode::Operator(_, left, right) => {
                let left_width = self.width(left);
                let right_width = self.width(right);
                left_width + right_width
            }
            AstNode::Negation(child) => {
                let child_height = self.height(child);
                child_height
            }
            AstNode::Operand(_) => 1,
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
                self.collect_variables(left, vars);
                self.collect_variables(right, vars);
            },
            AstNode::Negation(child) => {
                self.collect_variables(child, vars);
            }
        }
    }

    fn draw_tree(&self, node: &AstNode, buffer: &mut Vec<Vec<char>>, row: usize, col: usize) {
        match node {
            AstNode::Operator(op, left, right) => {
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
            AstNode::Negation(child) => {
                // Place operator
                buffer[row][col] = LogicalSymbol::Negation.to_unicode();
                buffer[row + 1][col] = '|';

                // Draw the single child (for unary operators like negation)
                self.draw_tree(child, buffer, row + 2, col);
            },
            AstNode::Operand(val) => {
                buffer[row][col] = val.to_unicode();
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
                let left_val = self.evaluate(left, values);
                let right_val = self.evaluate(right, values);
                match symbol {
                    LogicalSymbol::Negation => !left_val,
                    LogicalSymbol::Conjunction => left_val && right_val,
                    LogicalSymbol::Disjunction => left_val || right_val,
                    LogicalSymbol::ExclusiveOr => left_val ^ right_val,
                    LogicalSymbol::Implication => !left_val || right_val,
                    LogicalSymbol::Equivalence => left_val == right_val,
                    _ => unreachable!(),
                }
            },
            AstNode::Negation(child) => {
                !self.evaluate(child, values)
            }
        }
    }

    pub fn to_rpn(&self) -> String {
        if let Some(ref root) = self.root {
            self.node_to_rpn(root)
        } else {
            String::new()
        }
    }

    fn node_to_rpn(&self, node: &AstNode) -> String {
        match node {
            AstNode::Operand(symbol) => symbol.to_unicode_symbol().to_string(),
            AstNode::Negation(symbol) => format!("{}{}", self.node_to_rpn(symbol), LogicalSymbol::Negation.to_unicode_symbol()),
            AstNode::Operator(symbol, left, right) => {
                let left_str = self.node_to_rpn(left);
                let right_str = self.node_to_rpn(right);
                format!("{}{}{}", left_str, right_str, symbol.to_unicode_symbol())
            }
        }
    }
}

impl fmt::Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.visualize_tree())
    }
}

pub fn create_binary_op(op: LogicalSymbol, left: AstNode, right: AstNode) -> AstNode {
    AstNode::Operator(op, Box::new(left), Box::new(right))
}
