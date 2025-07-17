mod chunk;
mod debug;
mod error;
mod instruction;
mod value;
mod vm;

use chunk::Chunk;
use vm::VM;

fn main() {
    env_logger::init();

    let chunk = Chunk::new("test chunk")
        .op_constant(1.2, 123)
        .op_return(123);
    // chunk.disassemble();

    let vm = VM {};
    _ = vm.interpret(&chunk);
}
