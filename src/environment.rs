use crate::token::{Literal, Token};
use std::collections::HashMap;

pub struct Environment {
    instance: crate::Lox,
    values: HashMap<String, Literal>
}

impl Environment {
    pub fn build_envrionment(instance: crate::Lox) -> Environment {
        Environment {
            instance: instance,
            values: HashMap::new()
        }
    }

    // If there is an error send it here to report to the Lox instance
    fn error(&mut self, token: Token, message: String) {
        self.instance.interpreter_error(token, &*message);
    }

    pub fn get(&mut self, name: Token) -> Result<Literal, String> {
        if self.values.contains_key(&name.lexeme) {
            return Ok(self.values.get(&name.lexeme).unwrap().clone())
        } else {
            return Err(format!("Undefined variable {}.", name.lexeme))
        }
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: Token, value: Literal) {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme, value);
            return
        }
        
        self.error(name.clone(), format!("Undefined variable {}.", name.lexeme));
    }
}