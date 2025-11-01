use std::{
    env::args,
    fs::File,
    io::{self, Write},
};

fn main() -> io::Result<()> {
    let args: Vec<String> = args().collect();

    if args.len() != 2 {
        eprintln!("Usage: generate_ast <output directory>");
        std::process::exit(64);
    }

    let output_dir = args.get(1).cloned().unwrap();

    define_ast(
        output_dir,
        "Expr".to_string(),
        &[
            "Binary     : Expr left, Token operator, Expr right".to_string(),
            "Grouping   : Expr expression".to_string(),
            "Literal    : Object value".to_string(),
            "Unary      : Token operator, Expr right".to_string(),
        ],
    )
}

fn define_ast(output_dir: String, base_name: String, types: &[String]) -> io::Result<()> {
    let path = format!("{output_dir}/{}.rs", base_name.to_lowercase());
    let mut file = File::create(path)?;

    writeln!(file, "use crate::error::*;")?;
    writeln!(file, "use crate::token::*;")?;

    Ok(())
}
