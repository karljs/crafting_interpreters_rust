pub mod chunk;
pub mod compiler;
pub mod debug;
pub mod error;
pub mod instruction;
pub mod scanner;
pub mod token;
pub mod value;
pub mod vm;

use std::io::Read;

use log::{Level, log_enabled};

use crate::{chunk::Chunk, vm::VM};

pub fn run_from_source(mut reader: Box<dyn Read>) -> anyhow::Result<()> {
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    println!("{}", contents);
    Ok(())
}

pub fn run_repl() -> anyhow::Result<()> {
    println!("REPL");
    Ok(())
}

pub fn run() -> anyhow::Result<()> {
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
    vm.run_chunk(&chunk)
}
