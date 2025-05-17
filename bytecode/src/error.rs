use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum LoxError {
    InvalidOpcode,
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoxError::InvalidOpcode => write!(f, "Invalid Opcode"),
        }
    }
}

impl Error for LoxError {}
