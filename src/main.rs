mod error;
mod expr_hand;
mod pretty_printer;
mod scanner;
mod token;
mod token_type;

use expr_hand::*;
use pretty_printer::AstPrinter;
use token::Token;

use crate::error::TeciError;
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
        if args.get(1).unwrap() == "test" {
            run_test();
        } else {
            run_script(&args[1]).unwrap_or_else(|_| panic!("Could not run script {}", &args[1]));
        }
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
    for token in tokens {
        println!("{:?}", token);
    }
    Ok(())
}

fn run_test() -> Result<(), TeciError> {
    let expression = Expr::Binary(BinaryExpr {
        left: Box::new(Expr::Literal(LiteralExpr {
            value: Some(token::Object::Num(1.0)),
        })),
        operator: Token {
            ttype: token_type::TokenType::Plus,
            lexeme: "+".to_string(),
            literal: None,
            line: 1,
        },
        right: Box::new(Expr::Literal(LiteralExpr {
            value: Some(token::Object::Num(2.0)),
        })),
    });

    let printer = AstPrinter {};
    println!("{}", printer.print(&expression)?);
    Ok(())
}
