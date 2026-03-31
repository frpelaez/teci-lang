use std::fmt;
use std::fmt::Debug;
use std::rc::Rc;

use crate::{error::TeciResult, interpreter::Interpreter, object::Object};

pub trait TeciCallable {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &Interpreter, args: Vec<Object>) -> Result<Object, TeciResult>;
}

#[derive(Clone)]
pub struct Callable {
    pub func: Rc<dyn TeciCallable>,
}

impl Debug for Callable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "callable")
    }
}

impl PartialEq for Callable {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.func, &other.func)
    }
}

impl TeciCallable for Callable {
    fn arity(&self) -> usize {
        self.func.arity()
    }

    fn call(&self, interpreter: &Interpreter, args: Vec<Object>) -> Result<Object, TeciResult> {
        self.func.call(interpreter, args)
    }
}
