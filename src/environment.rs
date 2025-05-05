use std::collections::HashMap;

use crate::error::{Result, parse_error};
use crate::eval::ExprEval;

pub struct Environment {
    values: HashMap<String, Option<ExprEval>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, identifier: String, value: Option<ExprEval>) {
        self.values.insert(identifier, value);
    }

    pub fn get(&self, identifier: &str) -> Result<&Option<ExprEval>> {
        match self.values.get(identifier) {
            Some(rhs) => Ok(rhs),
            None => Err(parse_error::<Option<ExprEval>>(&format!(
                "Undefined variable access {:?}",
                identifier,
            ))),
        }
    }

    pub fn debug_dump(&self) {
        for (key, value) in &self.values {
            println!("({:?} -> {:?})", key, value)
        }
    }
}
