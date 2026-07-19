use crate::{opcode::OpCode, value::Value};

pub struct Chunk {
    name: String,
    code: Vec<u8>,
    constants: Vec<Value>,
    lines: Vec<(u32, usize)>,
}

impl Chunk {
    pub fn new(name: &str) -> Self {
        Chunk {
            name: name.to_string(),
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn emit_op(&mut self, op: OpCode, line: u32) {
        self.emit_byte(op as u8, line);
    }

    pub fn emit_byte(&mut self, byte: u8, line: u32) {
        self.code.push(byte);
        match self.lines.last_mut() {
            Some(entry) if entry.0 == line => entry.1 += 1,
            _ => self.lines.push((line, 1)),
        }
    }

    pub fn code(&self) -> &[u8] {
        &self.code
    }

    pub fn constant(&self, idx: usize) -> Value {
        self.constants[idx]
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn emit_constant(&mut self, value: Value, line: u32) {
        let idx = self.add_constant(value);
        if idx <= 0xFF {
            self.emit_op(OpCode::Constant, line);
            self.emit_byte(idx as u8, line);
        } else {
            panic!("too many constants in one chunk");
        }
    }

    pub fn disassemble(&self) {
        println!("== {} ==", self.name);
        let mut lines = self.line_bytes();
        let mut prev = None;
        let mut ip = 0;
        while ip < self.code.len() {
            let line = lines.next().unwrap();
            let next = self.disassemble_instruction(ip, line, prev);
            for _ in 0..(next - ip - 1) {
                lines.next();
            }
            prev = Some(line);
            ip = next;
        }
    }

    fn line_bytes(&self) -> impl Iterator<Item = u32> + '_ {
        self.lines
            .iter()
            .flat_map(|&(line, count)| std::iter::repeat_n(line, count))
    }

    fn disassemble_instruction(&self, ip: usize, line: u32, prev: Option<u32>) -> usize {
        if prev == Some(line) {
            print!("{ip:04}    | ");
        } else {
            print!("{ip:04} {line:>4} ");
        }
        let op = OpCode::read(self.code[ip]);
        match op {
            OpCode::Constant => {
                let idx = self.code[ip + 1] as usize;
                println!("{:<16} {:>4} '{}'", "constant", idx, self.constants[idx]);
                ip + 2
            }
            OpCode::Negate => {
                println!("negate");
                ip + 1
            }
            OpCode::Add => {
                println!("add");
                ip + 1
            }
            OpCode::Subtract => {
                println!("subtract");
                ip + 1
            }
            OpCode::Multiply => {
                println!("multiply");
                ip + 1
            }
            OpCode::Divide => {
                println!("divide");
                ip + 1
            }
            OpCode::Return => {
                println!("return");
                ip + 1
            }
        }
    }
}
