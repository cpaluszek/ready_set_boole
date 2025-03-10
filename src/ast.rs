use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum LogicalSymbol {
    // Operands
    False,  // 0, ⊥
    True,   // 1, ⊤

    // Operators
    Negation,      // !, ¬
    Conjunction,   // &, ∧
    Disjunction,   // |, ∨
    ExclusiveOr,   // ^, ⊕
    Implication,   // >, ⇒
    Equivalence,   // =, ⇔
}

impl LogicalSymbol {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '0' | '⊥' => Some(LogicalSymbol::False),
            '1' | '⊤' => Some(LogicalSymbol::True),
            '!' | '¬' => Some(LogicalSymbol::Negation),
            '&' | '∧' => Some(LogicalSymbol::Conjunction),
            '|' | '∨' => Some(LogicalSymbol::Disjunction),
            '^' | '⊕' => Some(LogicalSymbol::ExclusiveOr),
            '>' | '⇒' => Some(LogicalSymbol::Implication),
            '=' | '⇔' => Some(LogicalSymbol::Equivalence),
            _ => None,
        }
    }

    pub fn to_unicode(&self) -> char {
        match self {
            LogicalSymbol::False => '⊥',
            LogicalSymbol::True => '⊤',
            LogicalSymbol::Negation => '¬',
            LogicalSymbol::Conjunction => '∧',
            LogicalSymbol::Disjunction => '∨',
            LogicalSymbol::ExclusiveOr => '⊕',
            LogicalSymbol::Implication => '⇒',
            LogicalSymbol::Equivalence => '⇔',
        }
    }

    pub fn is_operand(&self) -> bool {
        matches!(self, LogicalSymbol::True | LogicalSymbol::False)
    }

    pub fn is_operator(&self) -> bool {
        !self.is_operand()
    }
}

#[derive(Debug, Clone)]
pub enum AstNode {
    Operator(LogicalSymbol, Box<AstNode>, Box<AstNode>),
    Operand(LogicalSymbol),
}

#[derive(Debug, Clone)]
pub struct Ast {
    root: Option<Box<AstNode>>,
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
            AstNode::Operand(_) => 1,
        }
    }

    fn width(&self, node: &AstNode) -> usize {
        match node {
            AstNode::Operator(_, left, right) => {
                let left_width = self.width(left);
                let right_width = self.width(right);
                left_width + right_width + 1
            }
            AstNode::Operand(_) => 1,
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
                buffer[row + 1][col - 1] = '/';
                buffer[row + 1][col + 1] = '\\';

                // Draw children
                self.draw_tree(left, buffer, row + 2, left_col);
                self.draw_tree(right, buffer, row + 2, right_col);
            },
            AstNode::Operand(val) => {
                buffer[row][col] = val.to_unicode();
            }
        }
    }

    fn visualize_tree(&self) -> String {
        if let Some(root) = &self.root {
            let height = self.height(root) * 2;
            let width = self.width(root) * 2;

            let mut buffer = vec![vec![' '; width]; height];

            self.draw_tree(root, &mut buffer, 0, width / 2);

            let mut result = String::with_capacity(height * width);
            for row in buffer {
                let line: String = row.into_iter().collect();
                let trimmed = line.trim_end();
                result.push_str(trimmed);
                result.push('\n');
            }

            result
        } else {
            String::new()
        }
    }
}

impl fmt::Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.visualize_tree())
    }
}
