// TODO: time and space complexity of each function

pub mod arithmetic;
pub mod evaluate;
pub mod error;
pub mod truth_table;
pub mod negation_normal_form;
pub mod conjunctive_normal_form;
pub mod sat;
pub mod expression;

pub use arithmetic::*;
pub use evaluate::eval_formula;
pub use error::LogicError;
// pub use truth_table::*;
pub use negation_normal_form::*;
pub use conjunctive_normal_form::*;
// pub use sat::*;
pub use expression::*;

pub fn pop_from_stack<T>(stack: &mut Vec<T>) -> Result<T, LogicError> {
    stack.pop().ok_or(LogicError::MissingArgument)
}
