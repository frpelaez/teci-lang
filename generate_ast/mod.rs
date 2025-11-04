use std::{
    fs::File,
    io::{self, Write},
};

#[derive(Debug)]
struct TreeType {
    base_class_name: String,
    class_name: String,
    fields: Vec<String>,
}

pub fn generate_ast(output_dir: &String) -> io::Result<()> {
    define_ast(
        output_dir,
        &"Expr".to_string(),
        &[
            "Binary     : Box<Expr> left, Token operator, Box<Expr> right".to_string(),
            "Grouping   : Box<Expr> expression".to_string(),
            "Literal    : Option<Object> value".to_string(),
            "Unary      : Token operator, Box<Expr> right".to_string(),
        ],
    )
}

fn define_ast(output_dir: &String, base_name: &String, types: &[String]) -> io::Result<()> {
    let path = format!("{output_dir}/{}.rs", base_name.to_lowercase());
    let mut file = File::create(path)?;
    let mut tree_types = Vec::new();

    writeln!(file, "use crate::error::*;")?;
    writeln!(file, "use crate::token::*;")?;
    writeln!(file, "use crate::object::*;")?;

    for ttype in types {
        let (base_class_name, args) = ttype.split_once(":").unwrap();
        let class_name = format!("{}{}", base_class_name.trim(), base_name); // BinaryExpr
        let args_split = args.split(",");
        let mut fields = Vec::new();
        for arg in args_split {
            let (typ, name) = arg.trim().split_once(" ").unwrap();
            fields.push(format!("{}: {}", name, typ));
        }
        tree_types.push(TreeType {
            base_class_name: base_class_name.to_string(),
            class_name,
            fields,
        });
    }

    writeln!(file, "\n#[derive(Clone)]")?;
    writeln!(file, "pub enum {base_name} {{")?;
    for t in &tree_types {
        writeln!(
            file,
            "    {}({}),",
            t.base_class_name.trim_end_matches(" "),
            t.class_name
        )?;
    }
    writeln!(file, "}}")?;

    writeln!(file, "\nimpl {} {{", base_name)?;
    writeln!(
        file,
        "    pub fn accept<T>(&self, visitor: &dyn {}Visitor<T>) -> Result<T, TeciError> {{",
        base_name
    )?;
    writeln!(file, "        match self {{")?;
    for t in &tree_types {
        writeln!(
            file,
            "            {base_name}::{}(exp) => exp.accept(visitor),",
            t.base_class_name.trim()
        )?;
    }
    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;

    for t in &tree_types {
        writeln!(file, "\n#[derive(Clone)]")?;
        writeln!(file, "pub struct {} {{", t.class_name)?;
        for field in &t.fields {
            writeln!(file, "    pub {},", field)?;
        }
        writeln!(file, "}}")?;
    }

    writeln!(file, "\npub trait ExprVisitor<T> {{")?;
    for t in &tree_types {
        writeln!(
            file,
            "    fn visit_{}_{}(&self, expr: &{}) -> Result<T, TeciError>;",
            t.base_class_name.trim().to_lowercase(),
            base_name.to_lowercase(),
            t.class_name
        )?;
    }
    writeln!(file, "}}")?;

    for t in &tree_types {
        writeln!(file, "\nimpl {} {{", t.class_name)?;
        writeln!(
            file,
            "    pub fn accept<T>(&self, visitor: &dyn {}Visitor<T>) -> Result<T, TeciError> {{",
            base_name,
        )?;
        writeln!(
            file,
            "        visitor.visit_{}_{}(self)",
            t.base_class_name.trim().to_lowercase(),
            base_name.to_lowercase()
        )?;
        writeln!(file, "    }}")?;
        writeln!(file, "}}")?;
    }

    Ok(())
}
