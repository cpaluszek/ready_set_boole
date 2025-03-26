#[derive(Debug, Clone, PartialEq, Copy)]
pub enum LogicalSymbol {
    // Operands
    False,  // 0, ⊥
    True,   // 1, ⊤
    Variable(char),

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
            'A'..='Z' => Some(LogicalSymbol::Variable(c)),
            _ => None,
        }
    }

    pub fn to_unicode(&self) -> char {
        match self {
            LogicalSymbol::False => '0',
            LogicalSymbol::True => '1',
            LogicalSymbol::Negation => '!',
            LogicalSymbol::Conjunction => '&',
            LogicalSymbol::Disjunction => '|',
            LogicalSymbol::ExclusiveOr => '^',
            LogicalSymbol::Implication => '>',
            LogicalSymbol::Equivalence => '=',
            LogicalSymbol::Variable(c) => *c,
        }
    }

    pub fn is_operand(&self) -> bool {
        matches!(self, LogicalSymbol::True | LogicalSymbol::False | LogicalSymbol::Variable(_))
    }

    pub fn is_variable(&self) -> bool {
        matches!(self, LogicalSymbol::Variable(_))
    }

    pub fn is_operator(&self) -> bool {
        !self.is_operand()
    }
}
