mod chunk;
mod debug;
mod error;
mod instruction;
mod value;
mod vm;

use chunk::Chunk;
use log::{Level, log_enabled};
use vm::VM;

fn main() -> Result<(), error::LoxError> {
    env_logger::init();

    let chunk = Chunk::new("test chunk")
        .op_constant(1.2, 123)
        .op_negate(123)
        .op_constant(5.0, 123)
        .op_subtract(123)
        .op_return(123);

    if log_enabled!(Level::Debug) {
        chunk.disassemble();
    }

    let mut vm = VM::new();
    vm.interpret(&chunk)
}
