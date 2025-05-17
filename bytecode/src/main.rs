#![allow(overflowing_literals)]

mod chunk;
mod debug;
mod error;
mod opcode;
mod value;

use chunk::Chunk;
use opcode::OpCode;

fn main() {
    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(1.2);
    chunk.write_op(OpCode::Constant);
    chunk.write_data(constant);

    chunk.write_op(OpCode::Return);

    chunk.disassemble("test chunk");
}
