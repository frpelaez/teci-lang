use std::rc::Rc;

use crate::{
    callable::TeciCallable,
    envirnoment::Environment,
    error::TeciResult,
    interpreter::Interpreter,
    object::Object,
    stmt::{FunctionStmt, Stmt},
    token::Token,
};

pub struct TeciFunction {
    name: Token,
    params: Rc<Vec<Token>>,
    body: Rc<Vec<Stmt>>,
}

impl TeciFunction {
    pub fn new(declaration: &FunctionStmt) -> Self {
        Self {
            name: declaration.name.clone(),
            params: Rc::clone(&declaration.params),
            body: Rc::clone(&declaration.body),
        }
    }
}

impl TeciCallable for TeciFunction {
    fn call(&self, interpreter: &Interpreter, args: Vec<Object>) -> Result<Object, TeciResult> {
        let mut env = Environment::with_enclosing(Rc::clone(&interpreter.globals));

        self.params.iter().zip(args).for_each(|(p, a)| {
            env.define(&p.lexeme, a);
        });

        if let Err(TeciResult::Return { _value }) = interpreter.execute_block(&self.body, env) {
            Ok(_value)
        } else {
            Ok(Object::Nil)
        }
    }

    fn arity(&self) -> usize {
        self.params.len()
    }

    fn to_string(&self) -> String {
        format!("<fun {}>", self.name.lexeme)
    }
}
