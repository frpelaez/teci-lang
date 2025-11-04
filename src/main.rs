mod error;
mod expr;
mod parser;
mod pretty_printer;
mod scanner;
mod token;
mod token_type;

use crate::error::TeciError;
use crate::parser::Parser;
use crate::pretty_printer::AstPrinter;
use crate::scanner::Scanner;

use std::{
    env::args,
    io::{self, Write, stdout},
};

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
        Err(_m) => {
            // Error already reported
            std::process::exit(65);
        }
    }
    Ok(())
}

fn run_prompt() {
    let stdin = io::stdin();
    print!(">> ");
    let _ = stdout().flush();
    for line in stdin.lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            match run(line) {
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

fn run(source: String) -> Result<(), TeciError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;
    // for token in &tokens {
    //     println!("{:?}", token);
    // }

    let mut parser = Parser::new(tokens);
    let expr = parser.parse();

    let printer = AstPrinter {};
    if let Some(expr) = expr {
        println!("{}", printer.print(&expr)?);
    }

    Ok(())
}
