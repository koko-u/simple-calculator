use std::io;

use crate::locations::Loc;

use super::lex_error::LexError;
use super::parse_error::ParseError;

#[derive(Debug, derive_more::Display, derive_more::Error, derive_more::From)]
pub enum AppError {
    #[display(fmt = "IO Error {}", _0)]
    Io(#[error(source)] io::Error),
    #[display(fmt = "Lexer Error")]
    Lex(#[error(source)] LexError),
    #[display(fmt = "Parse Error")]
    Parse(#[error(source)] ParseError),
}

impl AppError {
    pub fn show_diagnostic(&self, input: &str) {
        match self {
            AppError::Io(e) => eprintln!("{e:?}"),
            AppError::Lex(e) => {
                print_annot(input, e.location());
            }
            AppError::Parse(ParseError::Eof) => {
                print_annot(input, Loc(input.len(), input.len() + 1));
            }
            AppError::Parse(ParseError::RedundantExpression(token)) => {
                let Loc(from, _) = token.location();
                print_annot(input, Loc(from, input.len()));
            }
            AppError::Parse(ParseError::InvalidExpression(token))
            | AppError::Parse(ParseError::InvalidOperator(token))
            | AppError::Parse(ParseError::UnexpectedToken(token))
            | AppError::Parse(ParseError::UnclosedParen(token)) => {
                print_annot(input, token.location());
            }
        }
    }
}

fn print_annot(input: &str, Loc(from, to): Loc) {
    eprintln!("{input}");
    eprintln!("{}{}", " ".repeat(from), "^".repeat(to - from))
}
