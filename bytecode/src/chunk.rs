use std::{collections::HashMap, ops::Range};

use crate::{debug::disassemble_instructions, opcode::OpCode, value::Value};

#[derive(Default)]
pub struct Chunk {
    code: Vec<u8>,
    values: Vec<Value>,
    lines: HashMap<u8, Range<u8>>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk::default()
    }

    pub fn write_op(&mut self, op: OpCode, line: u8) -> u8 {
        self.write_data(op.into(), line)
    }

    pub fn write_data(&mut self, data: u8, line: u8) -> u8 {
        let code_last_idx = self.code.len() as u8;
        self.code.push(data);

        self.lines
            .entry(line)
            .and_modify(|range| range.end = code_last_idx + 1)
            .or_insert(Range {
                start: code_last_idx,
                end: code_last_idx + 1,
            });
        code_last_idx
    }

    pub fn add_constant(&mut self, val: f64) -> u8 {
        self.values.push(val);
        (self.values.len() - 1) as u8
    }

    pub fn disassemble(&self, name: &str) {
        println!("{:?}", &self.lines);

        println!("== {} ==", name);
        disassemble_instructions(0, &self.code, &self.values, &self.lines);
    }
}
