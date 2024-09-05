use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use std::process::exit;
use std::result::Result;

use crate::scanner::*;
use crate::parser::*;
mod scanner;
mod expr;
mod parser;

fn run_file(path: &str) -> Result<(), String> {
    match fs::read_to_string(path) {
        Ok(contents) => run(&contents),
        Err(_) => Err("ERROR: could not run file".to_string()),
    }
}

fn run(source: &str) -> Result<(), String> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;
    
    let mut parser = Parser::new(tokens);
    let parsed_expr = parser.parse()?;

    println!("{}", parsed_expr.to_string());
    Ok(())
}

fn run_prompt() -> Result<(), String> {
    loop {
        print!("> ");
        match io::stdout().flush() {
            Ok(_) => (),
            Err(_) => return Err("ERROR: could not flush stdout".to_string()),
        }

        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        match handle.read_line(&mut buffer) {
            Ok(line) => {
                if line <= 1 {
                    return Ok(());
                }
            }
            Err(_) => return Err("ERROR: could not read line".to_string()),
        }

        match run(&buffer) {
            Ok(_) => (),
            Err(msg) => println!("{}", msg),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        eprintln!("Usage: jlox [script]");
        exit(64);
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => exit(0),
            Err(err) => {
                eprintln!("ERROR:\n{}", err);
                exit(1);
            }
        }
    } else {
        match run_prompt() {
            Ok(_) => exit(0),
            Err(err) => {
                eprintln!("ERROR:\n{}", err);
                exit(1);
            }
        }
    }
}
