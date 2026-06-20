use crate::{chunk::Chunk, error::LoxError, opcode::OpCode, value::Value};
use log::{Level, log_enabled};

pub struct VM<'a> {
    chunk: &'a Chunk,
    ip: usize,
    stack: Vec<Value>,
}

impl<'a> VM<'a> {
    pub fn new(chunk: &'a Chunk) -> Self {
        Self {
            chunk,
            ip: 0,
            stack: Vec::with_capacity(256),
        }
    }

    fn read_byte(&mut self) -> u8 {
        let b = self.chunk.code[self.ip];
        self.ip += 1;
        b
    }

    fn read_opcode(&mut self) -> OpCode {
        OpCode::read(self.read_byte())
    }

    fn read_constant(&mut self) -> Value {
        let idx = self.read_byte() as usize;
        self.chunk.constants[idx]
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        loop {
            if log_enabled!(Level::Debug) {
                println!("stack: {:?}  ip: {}", &self.stack, self.ip);
            }

            match self.read_opcode() {
                OpCode::Constant => {
                    let val = self.read_constant();
                    self.stack.push(val);
                }
                OpCode::Negate => {
                    let val = self.stack.pop().ok_or(LoxError::RuntimeError)?;
                    self.stack.push(-val);
                }
                OpCode::Add      => self.binary_op(|a, b| a + b)?,
                OpCode::Subtract => self.binary_op(|a, b| a - b)?,
                OpCode::Multiply => self.binary_op(|a, b| a * b)?,
                OpCode::Divide   => self.binary_op(|a, b| a / b)?,
                OpCode::Return => {
                    let val = self.stack.pop().ok_or(LoxError::RuntimeError)?;
                    println!("return {val:?}");
                    return Ok(());
                }
            }
        }
    }

    fn binary_op(&mut self, op: impl Fn(Value, Value) -> Value) -> anyhow::Result<()> {
        let rhs = self.stack.pop().ok_or(LoxError::RuntimeError)?;
        let lhs = self.stack.pop().ok_or(LoxError::RuntimeError)?;
        self.stack.push(op(lhs, rhs));
        Ok(())
    }
}
