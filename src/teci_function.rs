#![allow(unused_variables)]
use std::rc::Rc;

use crate::{
    callable::TeciCallable, envirnoment::Environment, error::TeciResult, interpreter::Interpreter,
    object::Object, stmt::FunctionStmt,
};

pub struct TeciFunction {
    declaration: FunctionStmt,
}

impl TeciFunction {
    pub fn new(declaration: FunctionStmt) -> Self {
        Self { declaration }
    }
}

impl TeciCallable for TeciFunction {
    fn call(&self, interpreter: &Interpreter, args: Vec<Object>) -> Result<Object, TeciResult> {
        let mut env = Environment::with_environment(Rc::clone(&interpreter.globals));

        self.declaration.params.iter().zip(args).for_each(|(p, a)| {
            env.define(&p.lexeme, a);
        });

        interpreter.execute_block(&self.declaration.body, env)?;

        Ok(Object::Nil)
    }

    fn arity(&self) -> usize {
        self.declaration.params.len()
    }

    fn to_string(&self) -> String {
        format!("<fun {}>", self.declaration.name.lexeme)
    }
}
