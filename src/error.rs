use std::fmt;

#[derive(Debug)]
pub enum LogicError {
    MissingArgument(char),
    UnrecognizedSymbol(char),
    IncompleteFormula{
        expected: usize,
        actual: usize,
    },
    UnexpectedOperatorCNF,
}

impl fmt::Display for LogicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogicError::MissingArgument(c) => write!(f, "Missing argument for operator '{}'", c),
            LogicError::UnrecognizedSymbol(c) => write!(f, "'{}' is not a recognized symbol", c),
            LogicError::IncompleteFormula { expected, actual } => 
                write!(f, "Incomplete formula, stack contains {} elements instead of {}", actual, expected),
            LogicError::UnexpectedOperatorCNF => write!(f, "Unexpected operator in CNF conversion"),
        }
    }
}

impl std::error::Error for LogicError {}
