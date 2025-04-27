use std::error;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub enum Error {}

pub fn report_error(line: usize, loc: String, message: String) {
    eprintln!("[line {}] Error {}: {}", line, loc, message);
}
