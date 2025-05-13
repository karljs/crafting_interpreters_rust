use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;

mod lox;
use lox::Lox;
mod environment;
mod error;
mod eval;
mod lexer;
mod parser;
mod program;
mod scanner;

fn main() -> error::Result<()> {
    let args: Vec<String> = env::args().collect();
    match &args[..] {
        [_] => run_prompt(),
        [_, script] => run_file(script),
        _ => {
            print_usage();
            std::process::exit(1);
        }
    }
}

fn print_usage() {
    println!("Usage: lox1 [script]");
}

fn run_file(script: &String) -> error::Result<()> {
    let mut file = File::open(script)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    run(buffer)
}

fn run_prompt() -> error::Result<()> {
    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut buffer = String::new();
        stdin.read_line(&mut buffer)?;
        buffer.pop();
        if buffer.is_empty() {
            break;
        }
        if let Err(e) = run(buffer) {
            println!("Error: {:?}", e);
        }
    }
    Ok(())
}

fn run(source: String) -> error::Result<()> {
    match Lox::new_from_input(source) {
        Ok(mut lox) => lox.eval(),
        Err(e) => Err(e),
    }
}
