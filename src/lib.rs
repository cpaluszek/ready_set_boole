// TODO: time and space complexity of each function

pub mod arithmetic;
pub mod evaluate;
pub mod ast;
pub mod symbol;
pub mod error;
pub mod truth_table;
pub mod negation_normal_form;

pub use arithmetic::*;
pub use evaluate::eval_formula;
pub use error::LogicError;
pub use truth_table::*;
pub use negation_normal_form::*;
