use crate::{chunk::Chunk, error::LoxError, instruction::Instruction, value::Value};
use log::{Level, log_enabled};

pub const STACK_STARTING_CAPACITY: usize = 256;

pub struct VM {
    stack: Vec<Value>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            stack: Vec::with_capacity(STACK_STARTING_CAPACITY),
        }
    }

    pub fn interpret(&mut self, chunk: &Chunk) -> anyhow::Result<()> {
        for instruction in chunk {
            if log_enabled!(Level::Debug) {
                println!("Current stack: {:?}", &self.stack);
                print!("Instruction: ");
                instruction.disassemble();
                println!();
            }

            match instruction {
                Instruction::Constant(val) => {
                    self.push(*val);
                }
                Instruction::Negate => {
                    if let Some(val) = self.pop() {
                        self.push(-val);
                    } else {
                        return Err(LoxError::RuntimeError.into());
                    }
                }
                Instruction::Return => {
                    // intentionally incorrect implementation, for debugging
                    if let Some(val) = self.pop() {
                        println!("return {val:?}");
                        return Ok(());
                    } else {
                        return Err(LoxError::RuntimeError.into());
                    }
                }
                Instruction::Add => {
                    if let (Some(lhs), Some(rhs)) = (self.pop(), self.pop()) {
                        self.push(lhs + rhs)
                    } else {
                        return Err(LoxError::RuntimeError.into());
                    }
                }
                Instruction::Subtract => {
                    if let (Some(lhs), Some(rhs)) = (self.pop(), self.pop()) {
                        self.push(lhs - rhs)
                    } else {
                        return Err(LoxError::RuntimeError.into());
                    }
                }
                Instruction::Multiply => {
                    if let (Some(lhs), Some(rhs)) = (self.pop(), self.pop()) {
                        self.push(lhs * rhs)
                    } else {
                        return Err(LoxError::RuntimeError.into());
                    }
                }
                Instruction::Divide => {
                    if let (Some(lhs), Some(rhs)) = (self.pop(), self.pop()) {
                        self.push(lhs / rhs)
                    } else {
                        return Err(LoxError::RuntimeError.into());
                    }
                }
            }
        }
        Ok(())
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value)
    }

    fn pop(&mut self) -> Option<Value> {
        self.stack.pop()
    }
}
