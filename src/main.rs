mod envirnoment;
mod error;
mod expr;
mod interpreter;
mod object;
mod parser;
mod scanner;
mod stmt;
mod token;
mod token_type;

use crate::error::TeciError;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;

use std::{
    env::args,
    io::{self, Write, stdout},
};

fn main() {
    let args: Vec<String> = args().collect();
    let teci = Teci::new();
    match args.len() {
        1 => teci.run_prompt(),
        2 => teci
            .run_script(&args[1])
            .unwrap_or_else(|_| panic!("Could not run script {}", &args[1])),
        _ => {
            println!("Usage: teci-lang [script]");
            std::process::exit(64)
        }
    }
}

struct Teci {
    interpreter: Interpreter,
}

impl Teci {
    pub fn new() -> Self {
        Self {
            interpreter: Interpreter::new(),
        }
    }

    fn run_script(&self, path: &String) -> io::Result<()> {
        let buf = std::fs::read_to_string(path)?;
        if self.run(buf).is_err() {
            std::process::exit(65);
        }
        Ok(())
    }

    fn run_prompt(&self) {
        let stdin = io::stdin();
        print!(">> ");
        let _ = stdout().flush();
        for line in stdin.lines() {
            if let Ok(line) = line {
                if line.is_empty() {
                    break;
                }
                match self.run(line) {
                    Ok(_) => {}
                    Err(_) => {
                        // already reported
                    }
                }
            } else {
                break;
            }
            print!(">> ");
            let _ = stdout().flush();
        }
    }

    fn run(&self, source: String) -> Result<(), TeciError> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens()?;

        let mut parser = Parser::new(tokens);
        let statements = parser.parse()?;

        if parser.succeded() {
            if let Some(()) = self.interpreter.interpret(&statements) {}
        }

        Ok(())
    }
}
