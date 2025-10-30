mod error;
mod scanner;
mod token;
mod token_type;

use crate::error::TeciError;
use crate::scanner::Scanner;
use std::{env::args, io};

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() > 2 {
        println!("Usage: teci-lang [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        run_script(&args[1]).unwrap_or_else(|_| panic!("Could not run script {}", &args[1]));
    } else {
        run_prompt();
    }
}

fn run_script(path: &String) -> io::Result<()> {
    let buf = std::fs::read_to_string(path)?;
    match run(buf) {
        Ok(_) => {}
        Err(m) => {
            m.report("".to_string());
            std::process::exit(65);
        }
    }
    Ok(())
}

fn run_prompt() {
    let stdin = io::stdin();
    print!(">> ");
    for line in stdin.lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            match run(line) {
                Ok(_) => {}
                Err(m) => {
                    m.report("".to_string());
                }
            }
        } else {
            break;
        }
    }
}

fn run(source: String) -> Result<(), TeciError> {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;
    for token in tokens {
        print!("{:?}", token);
    }
    Ok(())
}
