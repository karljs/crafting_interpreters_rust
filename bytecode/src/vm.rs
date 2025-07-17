use crate::{chunk::Chunk, error::Result, instruction::Instruction};
use log::{Level, log_enabled};

pub struct VM {}

impl VM {
    pub fn interpret(&self, chunk: &Chunk) -> Result<()> {
        for instruction in chunk {
            if log_enabled!(Level::Debug) {
                instruction.disassemble();
                println!()
            }

            match instruction {
                Instruction::Return => return Ok(()),
                Instruction::Constant(val) => {
                    println!("{val}");
                    continue;
                }
            }
        }
        Ok(())
    }
}
