use std::fmt::Display;

use crate::value::Value;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Return,
    Constant(u8),
}

impl Instruction {
    pub fn disassemble(&self, values: &[Value]) {
        match self {
            Instruction::Return => print!("return"),
            Instruction::Constant(idx) => {
                print!("{:<16} {:>4} {}", "constant", idx, values[*idx as usize]);
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn assert_instruction_size() {
        assert!(size_of::<Instruction>() <= 4);
    }
}
