use std::collections::hash_map::HashMap;

use crate::{error::TeciError, object::Object, token::Token};

pub struct Envirnoment {
    values: HashMap<String, Object>,
}

impl Envirnoment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<Object, TeciError> {
        if let Some(object) = self.values.get(&name.lexeme) {
            Ok(object.clone())
        } else {
            Err(TeciError::runtime_error(
                name.clone(),
                format!("Undefined variable {}", name.lexeme).as_str(),
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
        let mut e = Envirnoment::new();
        e.define("a".to_string(), Object::Bool(true));
        assert!(e.values.contains_key("a"));
        assert_eq!(e.values.get("a").unwrap(), &Object::Bool(true))
    }

    #[test]
    fn t_redefine_variable() {
        let mut e = Envirnoment::new();
        e.define("a".to_string(), Object::Bool(false));
        e.define("a".to_string(), Object::Num(6.0));
        assert_eq!(e.values.get("a").unwrap(), &Object::Num(6.0))
    }

    #[test]
    fn t_lookup_variable() {
        let mut e = Envirnoment::new();
        e.define("a".to_string(), Object::Str("foo".to_string()));
        let tok = Token::new(TokenType::Identifier, "a".to_string(), None, 0);
        let tok_err = Token::new(TokenType::Identifier, "b".to_string(), None, 0);
        assert_eq!(e.get(&tok).unwrap(), Object::Str("foo".to_string()));
        assert!(e.get(&tok_err).is_err())
    }
}
