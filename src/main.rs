use std::io;
use std::io::BufRead;
use std::io::Write;

use simple_calculator::AppError;
use simple_calculator::Ast;
use simple_calculator::DisplayRecursively;

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

        let ast = match line.parse::<Ast>() {
            Ok(ast) => ast,
            Err(e) => {
                let stderr = io::stderr().lock();
                e.print_recursive(stderr)?;
                continue;
            }
        };

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
