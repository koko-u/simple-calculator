use std::error;

use crate::annotations::Annot;
use crate::annotations::WithAnnot;

/// lexer error kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, derive_more::Display)]
pub enum LexErrorKind {
    /// invalid character error
    #[display(fmt = "Invalid Character {}", _0)]
    InvalidChar(char),
    /// EOF
    #[display(fmt = "End of file")]
    Eof,
}

pub type LexError = Annot<LexErrorKind>;

impl WithAnnot for LexErrorKind {}

impl error::Error for LexError {}
