use std::error;
use std::fmt;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum Error {
    ParseError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
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

pub fn report_error(line: usize, loc: String, message: String) {
    eprintln!("[line {}] Error {}: {}", line, loc, message);
}
