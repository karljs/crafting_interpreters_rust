use crate::{debug::disassemble_instructions, opcode::OpCode, value::Value};

pub struct Chunk {
    code: Vec<u8>,
    values: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            values: Vec::new(),
        }
    }

    pub fn write_op(&mut self, op: OpCode) -> u8 {
        self.code.push(op.into());
        (self.code.len() - 1) as u8
    }

    pub fn write_data(&mut self, data: u8) -> u8 {
        self.code.push(data);
        (self.code.len() - 1) as u8
    }

    pub fn add_constant(&mut self, val: f64) -> u8 {
        self.values.push(val);
        (self.values.len() - 1) as u8
    }

    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);
        disassemble_instructions(0, &self.code, &self.values);
    }
}
