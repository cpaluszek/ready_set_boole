
#[derive(Debug)]
pub enum LogicError {
    MissingArgument,
    UnrecognizedSymbol,
    IncompleteFormula,
    UnexpectedOperatorCNF,
    UnknownVar,
}

