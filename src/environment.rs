use crate::token::{Literal, Token};
use std::collections::HashMap;

pub struct Environment {
    values: HashMap<String, Literal>
}

impl Environment {
    pub fn build_envrionment() -> Environment {
        Environment {
            values: HashMap::new()
        }
    }

    fn get(&mut self, name: Token) -> Result<Literal, String> {
        if self.values.contains_key(&name.lexeme) {
            return Ok(self.values.get(&name.lexeme).unwrap().clone())
        } else {
            return Err(format!("Undefined variable {}.", name.lexeme))
        }
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }
}