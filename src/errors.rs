use std::io;
use std::io::Write;

use crate::lexer::lex_errors::LexError;
use crate::parser::parse_errors::ParseError;
use crate::tokens::locations::Loc;

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

pub trait DisplayRecursively {
    fn print_recursive<W: io::Write>(&self, writer: W) -> io::Result<()>;
}

impl<E> DisplayRecursively for E
where
    E: std::error::Error,
{
    fn print_recursive<W: io::Write>(&self, writer: W) -> io::Result<()> {
        let mut writer = io::BufWriter::new(writer);
        writer.write_all(format!("{}\n", self).as_bytes())?;

        let mut source = self.source();
        while let Some(s) = source {
            writer.write_all(format!("{}\n", s).as_bytes())?;
            source = s.source();
        }

        Ok(())
    }
}
