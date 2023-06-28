use std::io;

use crate::lexer::lex_errors::LexError;
use crate::parser::parse_errors::ParseError;

#[derive(Debug, derive_more::Display, derive_more::Error, derive_more::From)]
pub enum AppError {
    #[display(fmt = "IO Error {}", _0)]
    Io(#[error(source)] io::Error),
    #[display(fmt = "Lexer Error")]
    Lex(#[error(source)] LexError),
    #[display(fmt = "Parse Error")]
    Parse(#[error(source)] ParseError),
}
