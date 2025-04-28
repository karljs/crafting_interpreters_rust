use std::error;
use std::fmt;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum Error {
    ParseError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ParseError => write!(f, "Parse error!"),
        }
    }
}

impl error::Error for Error {}

pub fn report_error(line: usize, loc: String, message: String) {
    eprintln!("[line {}] Error {}: {}", line, loc, message);
}
