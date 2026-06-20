pub mod chunk;
pub mod compiler;
pub mod debug;
pub mod error;
pub mod opcode;
pub mod scanner;
pub mod token;
pub mod value;
pub mod vm;

use std::io::Read;

use log::{Level, log_enabled};

use crate::{chunk::Chunk, opcode::OpCode, vm::VM};

pub fn run_from_source(mut reader: Box<dyn Read>) -> anyhow::Result<()> {
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    println!("{}", contents);
    Ok(())
}

pub fn run_repl() -> anyhow::Result<()> {
    // println!("REPL");
    run()
}

pub fn run() -> anyhow::Result<()> {
    env_logger::init();

    let mut chunk = Chunk::new("test chunk");
    chunk.emit_constant(1.2, 123);
    chunk.emit_op(OpCode::Negate, 123);
    chunk.emit_constant(5.0, 123);
    chunk.emit_op(OpCode::Subtract, 123);
    chunk.emit_op(OpCode::Return, 124);

    if log_enabled!(Level::Debug) {
        chunk.disassemble();
    }

    VM::new(&chunk).run()
}
