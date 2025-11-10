use std::{
    cell::RefCell,
    collections::hash_map::{Entry, HashMap},
    rc::Rc,
};

use crate::{error::TeciError, object::Object, token::Token};

#[derive(Debug)]
pub struct Envirnoment {
    values: HashMap<String, Object>,
    enclosing: Option<Rc<RefCell<Envirnoment>>>,
}

impl Envirnoment {
    pub fn new() -> Rc<RefCell<Envirnoment>> {
        Rc::new(RefCell::new(Envirnoment {
            values: HashMap::new(),
            enclosing: None,
        }))
    }

    pub fn with_enclosing(enclosing: Rc<RefCell<Envirnoment>>) -> Envirnoment {
        Envirnoment {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<Object, TeciError> {
        if let Some(object) = self.values.get(&name.lexeme) {
            Ok(object.clone())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow().get(name)
        } else {
            Err(TeciError::runtime_error(
                name.clone(),
                format!("Undefined variable {}", name.lexeme).as_str(),
            ))
        }
    }

    pub fn assign(&mut self, name: &Token, value: Object) -> Result<(), TeciError> {
        if let Entry::Occupied(mut object) = self.values.entry(name.lexeme.clone()) {
            object.insert(value);
            Ok(())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow_mut().assign(name, value)
        } else {
            Err(TeciError::runtime_error(
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
        let e = Envirnoment::new();
        e.borrow_mut().define("a".to_string(), Object::Bool(true));
        assert!(e.borrow().values.contains_key("a"));
        assert_eq!(e.borrow().values.get("a").unwrap(), &Object::Bool(true))
    }

    #[test]
    fn t_redefine_variable() {
        let e = Envirnoment::new();
        e.borrow_mut().define("a".to_string(), Object::Bool(false));
        e.borrow_mut().define("a".to_string(), Object::Num(6.0));
        assert_eq!(e.borrow().values.get("a").unwrap(), &Object::Num(6.0))
    }

    #[test]
    fn t_lookup_variable() {
        let e = Envirnoment::new();
        e.borrow_mut()
            .define("a".to_string(), Object::Str("foo".to_string()));
        let tok = Token::new(TokenType::Identifier, "a".to_string(), None, 0);
        let tok_err = Token::new(TokenType::Identifier, "b".to_string(), None, 0);
        assert_eq!(
            e.borrow().get(&tok).unwrap(),
            Object::Str("foo".to_string())
        );
        assert!(e.borrow().get(&tok_err).is_err())
    }

    #[test]
    fn t_enclose_envirnoment() {
        let e = Envirnoment::new();
        let f = Envirnoment::with_enclosing(Rc::clone(&e));
        assert_eq!(
            e.borrow().values.clone(),
            f.enclosing.unwrap().borrow().values
        )
    }

    #[test]
    fn t_read_from_eclosing() {
        let e = Envirnoment::new();
        e.borrow_mut().define("a".to_string(), Object::Num(1.0));
        let f = Envirnoment::with_enclosing(Rc::clone(&e));
        assert_eq!(
            f.get(&Token::new(TokenType::Identifier, "a".to_string(), None, 0))
                .unwrap(),
            Object::Num(1.0)
        )
    }

    #[test]
    fn t_assign_to_enclosing() {
        let e = Envirnoment::new();
        e.borrow_mut().define("a".to_string(), Object::Num(1.0));
        let mut f = Envirnoment::with_enclosing(Rc::clone(&e));
        let tok = Token::new(TokenType::Identifier, "a".to_string(), None, 0);
        assert!(f.assign(&tok, Object::Bool(false)).is_ok());
        assert_eq!(f.get(&tok).unwrap(), Object::Bool(false))
    }
}
