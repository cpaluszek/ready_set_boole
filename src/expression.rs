#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Operand(bool),
    Variable(char),
    Negation(Box<Expression>),
    Conjunction(Box<Expression>, Box<Expression>),
    Disjunction(Box<Expression>, Box<Expression>),
    ExclusiveOr(Box<Expression>, Box<Expression>),
    Implication(Box<Expression>, Box<Expression>),
    Equivalence(Box<Expression>, Box<Expression>),
}

impl Expression {
    pub fn to_unicode(&self) -> char {
        match self {
            Expression::Operand(true) => '1',
            Expression::Operand(false) => '0',
            Expression::Variable(c) => *c,
            Expression::Negation(_) => '!',
            Expression::Conjunction(_, _) => '&',
            Expression::Disjunction(_, _) => '|',
            Expression::ExclusiveOr(_, _) => '^',
            Expression::Implication(_, _) => '>',
            Expression::Equivalence(_, _) => '=',
        }
    }

    pub fn is_variable(&self) -> bool {
        matches!(self, Expression::Variable(_))
    }
}
