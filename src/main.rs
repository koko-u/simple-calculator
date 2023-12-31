use std::env;
use std::io;
use std::io::BufRead;
use std::io::Write;

use env_logger::Env;
use simple_calculator::AppError;
use simple_calculator::Ast;
use simple_calculator::DisplayRecursively;
use simple_calculator::Interpreter;

fn main() -> Result<(), AppError> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let stdin = io::stdin().lock();
    let reader = io::BufReader::new(stdin);
    let mut lines = reader.lines();

    let mut interpreter = Interpreter;

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
                e.show_diagnostic(&line);
                e.print_recursive(stderr)?;
                continue;
            }
        };

        let n = match interpreter.eval(&ast) {
            Ok(n) => n,
            Err(e) => {
                let stderr = io::stderr().lock();
                e.show_diagnostic(&line);
                e.print_recursive(stderr)?;
                continue;
            }
        };

        println!("{n}");
    }

    Ok(())
}

fn prompt(message: &str) -> io::Result<()> {
    let mut stdout = io::stdout().lock();
    stdout.write_all(message.as_bytes())?;
    stdout.flush()?;
    Ok(())
}
