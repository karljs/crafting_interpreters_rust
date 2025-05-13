use std::error;
use std::fmt;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum Error {
    RuntimeError(String),
    ParseError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::RuntimeError(msg) => write!(f, "Runtime error! {:?}", msg),
            Error::ParseError(msg) => write!(f, "Parse error! {:?}", msg),
        }
    }
}

impl error::Error for Error {}

pub fn parse_error<T>(msg: &str) -> Box<Error> {
    Box::new(Error::ParseError(msg.to_string()))
}

pub fn eof_parse_error<T>() -> Box<Error> {
    parse_error::<T>("Reached end of input while parsing")
}

pub fn runtime_error<T>(msg: &str) -> Box<Error> {
    Box::new(Error::RuntimeError(msg.to_string()))
}
