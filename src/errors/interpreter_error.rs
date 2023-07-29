use super::print_error::print_error;
use crate::annotations::Annot;
use crate::annotations::WithAnnot;

#[derive(Debug, Clone, Copy, PartialEq, Eq, derive_more::Display)]
pub enum InterpreterErrorKind {
    #[display(fmt = "Zero-division Error")]
    ZeroDivision,
    #[display(fmt = "Overflow Error")]
    Overflow,
}

pub type InterpreterError = Annot<InterpreterErrorKind>;

impl WithAnnot for InterpreterErrorKind {}
impl std::error::Error for InterpreterError {}

impl InterpreterError {
    pub fn show_diagnostic(&self, input: &str) {
        print_error(input).with(self.location());
    }
}
