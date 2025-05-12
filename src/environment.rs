use std::collections::HashMap;

use crate::error::{Result, parse_error};
use crate::eval::ExprEval;

pub struct Scope {
    values: HashMap<String, Option<ExprEval>>,
}

pub struct Environment {
    scopes: Vec<Scope>,
}

impl Environment {
    pub fn new() -> Self {
        let mut env = Environment { scopes: Vec::new() };
        env.enter_scope();
        env
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(Scope {
            values: HashMap::new(),
        });
    }

    pub fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn define(&mut self, identifier: String, value: Option<ExprEval>) {
        // self.values.insert(identifier, value);
        self.scopes
            .last_mut()
            .unwrap()
            .values
            .insert(identifier, value);
    }

    pub fn assign(&mut self, identifier: String, value: Option<ExprEval>) -> bool {
        for scope in self.scopes.iter_mut().rev() {
            if scope.values.contains_key(&identifier) {
                scope.values.insert(identifier, value);
                return true;
            }
        }
        false
    }

    pub fn get(&self, identifier: &str) -> Result<&Option<ExprEval>> {
        for scope in self.scopes.iter().rev() {
            if let Some(val) = scope.values.get(identifier) {
                return Ok(val);
            }
        }
        Err(parse_error::<Option<ExprEval>>(&format!(
            "Undefined variable access {:?}",
            identifier,
        )))
    }

    pub fn debug_dump(&self) {
        for scope in self.scopes.iter().rev() {
            println!("scope");
            println!("-----");
            for (key, value) in &scope.values {
                println!("({:?} -> {:?})", key, value)
            }
            println!("-----");
        }
        println!("\n");
    }
}
