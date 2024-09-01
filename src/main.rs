use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use std::process::ExitCode;
use std::result;

type Result<T> = result::Result<T, ()>;

fn start() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        eprintln!("Usage: jlox [script]");
        Err(())
    } else if args.len() == 2 {
        run_file(&args[1]);
        Ok(())
    } else {
        run_prompt();
        Ok(())
    }
}

fn run_file(path: &str) {
    let content =
        fs::read_to_string(path).map_err(|err| eprintln!("ERROR: could not read file: {err}"));
}

fn run(source: &str) {
    todo!()
}

fn run_prompt() -> Result<()> {
    while true {
        print!("> ");
        let _ = io::stdout()
            .flush()
            .map_err(|err| eprintln!("ERROR: could not flush stdout: {err}"));

        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        match handle.read_line(&mut buffer) {
            Ok(line) => {
                if line <= 1 {
                    return Ok(());
                }
            }
            Err(_) => eprintln!("ERROR: could not read line"),
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    match start() {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => ExitCode::FAILURE,
    }
}
