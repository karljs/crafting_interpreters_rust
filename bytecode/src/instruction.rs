use std::fmt::Display;

use crate::value::Value;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Return,
    Constant(Value),
}

impl Instruction {
    pub fn disassemble(&self) {
        match self {
            Instruction::Return => print!("return"),
            Instruction::Constant(val) => {
                print!("{:<16} {:>4}", "constant", val);
            }
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = format!("{:?}", self);
        name.fmt(f)
    }
}
