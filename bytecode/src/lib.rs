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

use crate::{chunk::Chunk, opcode::OpCode, scanner::Scanner, vm::VM};

pub fn run_from_source(mut reader: impl Read) -> anyhow::Result<()> {
    let mut source = String::new();
    reader.read_to_string(&mut source)?;

    let scanner = Scanner::new(source);
    for token in scanner.lexemes() {
        println!("{token:?}");
    }

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
    chunk.emit_op(OpCode::Negate, 124);
    chunk.emit_constant(5.0, 124);
    chunk.emit_op(OpCode::Subtract, 124);
    chunk.emit_op(OpCode::Return, 125);

    if log_enabled!(Level::Debug) {
        chunk.disassemble();
    }

    VM::new(&chunk).run()
}
