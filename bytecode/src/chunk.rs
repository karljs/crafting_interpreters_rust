use crate::{opcode::Instruction, value::Value};

#[derive(Default)]
pub struct Chunk {
    name: String,
    instructions: Vec<Instruction>,
    values: Vec<Value>,
    lines: Vec<usize>,
}

impl Chunk {
    pub fn new(name: &str) -> Self {
        Chunk {
            name: name.to_string(),
            ..Chunk::default()
        }
    }

    pub fn op_return(mut self, line: usize) -> Chunk {
        self.instructions.push(Instruction::Return);
        self.lines.push(line);
        self
    }

    pub fn op_constant(mut self, constant: Value, line: usize) -> Chunk {
        let value_idx = self.values.len() as u8;
        self.instructions.push(Instruction::Constant(value_idx));
        self.values.push(constant);
        self.lines.push(line);
        self
    }

    pub fn disassemble(&self) {
        println!("== {} ==", self.name);

        let instruction_size = std::mem::size_of::<Instruction>();
        let mut previous_line = None;
        for (idx, instruction) in self.instructions.iter().enumerate() {
            print!("{addr:0>4} ", addr = idx * instruction_size);
            let line = self.lines[idx as usize];
            if previous_line.is_some_and(|prev: usize| prev == line) {
                print!("   | ");
            } else {
                print!("{line:>4} ");
                previous_line = Some(line);
            }
            instruction.disassemble(&self.values);
            println!();
        }
    }
}
