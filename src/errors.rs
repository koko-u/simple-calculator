use std::io;
use std::io::Write;

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
