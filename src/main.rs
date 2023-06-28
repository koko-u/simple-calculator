use std::io;
use std::io::BufRead;
use std::io::Write;

use simple_calculator::errors::AppError;
use simple_calculator::lexer::lex;
use simple_calculator::parser::parse;

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

fn prompt(message: &str) -> io::Result<()> {
    let mut stdout = io::stdout().lock();
    stdout.write_all(message.as_bytes())?;
    stdout.flush()?;
    Ok(())
}
