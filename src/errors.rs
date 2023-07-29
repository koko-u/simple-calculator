pub mod app_error;
pub mod interpreter_error;
pub mod lex_error;
pub mod parse_error;

mod print_error;

pub use interpreter_error::InterpreterError;
pub use interpreter_error::InterpreterErrorKind;
pub use lex_error::LexError;
pub use lex_error::LexErrorKind;
pub use parse_error::ParseError;

use std::io;
use std::io::Write;

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
