use std::error::Error;
use std::fmt;

pub type Result<T> = std::result::Result<T, LoxError>;

#[derive(Debug)]
pub enum LoxError {
    CompileError,
    RuntimeError,
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = match self {
            LoxError::CompileError => "Compile Error",
            LoxError::RuntimeError => "Runtime Error",
        };
        write!(f, "{out}")
    }
}

impl Error for LoxError {}
