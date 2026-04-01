use std::{
    cell::RefCell,
    collections::hash_map::{Entry, HashMap},
    rc::Rc,
};

use crate::{error::TeciResult, object::Object, token::Token};

#[derive(Debug)]
pub struct Environment {
    values: HashMap<String, Object>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn with_enclosing(environment: Rc<RefCell<Environment>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(environment),
        }
    }

    pub fn define(&mut self, name: &str, value: Object) {
        self.values.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &Token) -> Result<Object, TeciResult> {
        if let Some(object) = self.values.get(&name.lexeme) {
            Ok(object.clone())
        } else if let Some(enc) = &self.enclosing {
            enc.borrow().get(name)
        } else {
            Err(TeciResult::runtime_error(
                name.clone(),
                format!("Undefined variable {}", name.lexeme).as_str(),
            ))
        }
    }

    pub fn assign(&mut self, name: &Token, value: Object) -> Result<(), TeciResult> {
        if let Entry::Occupied(mut object) = self.values.entry(name.lexeme.clone()) {
            object.insert(value);
            Ok(())
        } else if let Some(enc) = &self.enclosing {
            enc.borrow_mut().assign(name, value)
        } else {
            Err(TeciResult::runtime_error(
                name.clone(),
                &format!("Undefined variable '{}'", name.lexeme),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token_type::*;

    #[test]
    fn t_define_variable() {
        let mut e = Environment::new();
        e.define("a", Object::Bool(true));
        assert!(e.values.contains_key("a"));
        assert_eq!(e.values.get("a").unwrap(), &Object::Bool(true))
    }

    #[test]
    fn t_redefine_variable() {
        let mut e = Environment::new();
        e.define("a", Object::Bool(false));
        e.define("a", Object::Num(6.0));
        assert_eq!(e.values.get("a").unwrap(), &Object::Num(6.0))
    }

    #[test]
    fn t_lookup_variable() {
        let mut e = Environment::new();
        e.define("a", Object::Str("foo".to_string()));
        let tok = Token::new(TokenType::Identifier, "a".to_string(), None, 0);
        let tok_err = Token::new(TokenType::Identifier, "b".to_string(), None, 0);
        assert_eq!(e.get(&tok).unwrap(), Object::Str("foo".to_string()));
        assert!(e.get(&tok_err).is_err())
    }

    #[test]
    fn t_enclose_environment() {
        let e = Rc::new(RefCell::new(Environment::new()));
        let f = Environment::with_enclosing(Rc::clone(&e));
        assert_eq!(f.enclosing.unwrap().borrow().values, e.borrow().values)
    }

    #[test]
    fn t_lookup_from_enclosing() {
        let e = Rc::new(RefCell::new(Environment::new()));
        e.borrow_mut().define("a", Object::Num(1.0));
        let f = Environment::with_enclosing(Rc::clone(&e));
        let a_token = Token::new(TokenType::Identifier, "a".to_string(), None, 0);
        assert_eq!(f.get(&a_token).unwrap(), Object::Num(1.0))
    }

    #[test]
    fn t_assign_to_enclosing() {
        let e = Rc::new(RefCell::new(Environment::new()));
        e.borrow_mut().define("a", Object::Num(1.0));
        let mut f = Environment::with_enclosing(Rc::clone(&e));
        let a_token = Token::new(TokenType::Identifier, "a".to_string(), None, 0);
        assert!(f.assign(&a_token, Object::Num(2.0)).is_ok());
        assert_eq!(f.get(&a_token).unwrap(), Object::Num(2.0))
    }
}
