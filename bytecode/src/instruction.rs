use std::fmt::Display;

use crate::value::Value;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Constant(Value),
    Negate,
    Return,
}

impl Instruction {
    pub fn disassemble(&self) {
        match self {
            Instruction::Constant(val) => {
                print!("{:<16} {:>4}", "constant", val);
            }
            Instruction::Negate => print!("negate"),
            Instruction::Return => print!("return"),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = format!("{:?}", self);
        name.fmt(f)
    }
}
