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

pub fn generate_ast(output_dir: &str) -> io::Result<()> {
    define_ast(
        output_dir,
        "Expr",
        &["token", "object", "error"],
        &[
            "Assign     : Token name, Box<Expr> value",
            "Binary     : Box<Expr> left, Token operator, Box<Expr> right",
            "Grouping   : Box<Expr> expression",
            "Literal    : Option<Object> value",
            "Unary      : Token operator, Box<Expr> right",
            "Variable   : Token name",
        ],
    )?;

    define_ast(
        output_dir,
        "Stmt",
        &["error", "expr", "token"],
        &[
            "Expression : Expr expression",
            "Print      : Expr expression",
            "Let        : Token name, Option<Expr> initializer",
        ],
    )
}

fn define_ast(
    output_dir: &str,
    base_name: &str,
    imports: &[&str],
    types: &[&str],
) -> io::Result<()> {
    let path = format!("{output_dir}/{}.rs", base_name.to_lowercase());
    let mut file = File::create(path)?;
    let mut tree_types = Vec::new();

    for module in imports {
        writeln!(file, "use crate::{}::*;", module)?;
    }

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

    writeln!(file, "\npub trait {}Visitor<T> {{", base_name)?;
    for t in &tree_types {
        writeln!(
            file,
            "    fn visit_{}_{}(&self, {}: &{}) -> Result<T, TeciError>;",
            t.base_class_name.trim().to_lowercase(),
            base_name.to_lowercase(),
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
