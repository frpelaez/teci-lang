use crate::{callable::TeciCallable, error::TeciResult, interpreter::Interpreter, object::Object};

use std::time;

pub struct NativeClock;

impl TeciCallable for NativeClock {
    fn arity(&self) -> usize {
        0
    }

    fn call(&self, _interpreter: &Interpreter, _args: Vec<Object>) -> Result<Object, TeciResult> {
        match time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH) {
            Ok(dur) => Ok(Object::Num(dur.as_millis() as f64)),
            Err(e) => Err(TeciResult::system_error(&format!(
                "Clock returned invalid duration: {:?}",
                e.duration()
            ))),
        }
    }

    fn to_string(&self) -> String {
        "<fun native::clock>".to_string()
    }
}
