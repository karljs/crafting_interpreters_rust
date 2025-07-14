mod chunk;
mod debug;
mod error;
mod opcode;
mod value;

use chunk::Chunk;

fn main() {
    let chunk = Chunk::new("test chunk")
        .op_constant(1.2, 123)
        .op_return(123);
    chunk.disassemble();
}
