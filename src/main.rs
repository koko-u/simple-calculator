use std::io;
use std::io::BufRead;
use std::io::Write;

use simple_calculator::lexer::lex;
use simple_calculator::lexer::lex_errors::LexError;
use simple_calculator::parser::parse;
use simple_calculator::parser::parse_errors::ParseError;

fn main() -> Result<(), AppError> {
    let stdin = io::stdin().lock();
    let reader = io::BufReader::new(stdin);
    let mut lines = reader.lines();

    loop {
        prompt("> ")?;
        let Some(line) = lines.next() else {
            break;
        };
        let line = line?;
        let tokens = lex(&line)?;
        let ast = parse(tokens)?;

        println!("{ast:#?}");
    }

    Ok(())
}

#[derive(Debug, derive_more::Display, derive_more::Error, derive_more::From)]
enum AppError {
    #[display(fmt = "IO Error {}", _0)]
    Io(#[error(source)] io::Error),
    #[display(fmt = "Lexer Error")]
    Lex(#[error(source)] LexError),
    #[display(fmt = "Parse Error")]
    Parse(#[error(source)] ParseError),
}

fn prompt(message: &str) -> io::Result<()> {
    let mut stdout = io::stdout().lock();
    stdout.write_all(message.as_bytes())?;
    stdout.flush()?;
    Ok(())
}
