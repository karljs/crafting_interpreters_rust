use std::fmt::Display;

use crate::error::LoxError;

#[repr(u8)]
#[derive(Debug)]
pub enum OpCode {
    Return = 1,
    Constant = 2,
}

// Surely there's a less verbose way to do this
impl TryFrom<u8> for OpCode {
    type Error = LoxError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(OpCode::Return),
            2 => Ok(OpCode::Constant),
            _ => Err(LoxError::InvalidOpcode),
        }
    }
}

impl Into<u8> for OpCode {
    fn into(self) -> u8 {
        self as u8
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = format!("{:?}", self);
        name.fmt(f)
    }
}
