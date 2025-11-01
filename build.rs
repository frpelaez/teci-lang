mod generate_ast;
use std::io;

use generate_ast::*;

fn main() -> io::Result<()> {
    generate_ast(&"src".to_string())
}
