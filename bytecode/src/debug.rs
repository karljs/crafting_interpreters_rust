use std::usize;

use crate::opcode::OpCode;

pub fn disassemble_instructions(mut addr: usize, bytes: &[u8], values: &[f64]) {
    while addr < bytes.len() {
        print!("{addr:0>4} ");
        match TryInto::<OpCode>::try_into(bytes[addr]) {
            Ok(opcode) if is_constant_instruction(&opcode) => {
                addr += disassemble_constant_instruction(&opcode, addr, bytes, values);
            }
            Ok(opcode) => {
                addr += disassemble_simple_instruction(&opcode);
            }
            Err(e) => panic!("{e}: Unable to parse opcode"),
        }
    }
}

fn disassemble_simple_instruction(opcode: &OpCode) -> usize {
    println!("{opcode}");
    1
}

fn disassemble_constant_instruction(
    opcode: &OpCode,
    addr: usize,
    bytes: &[u8],
    values: &[f64],
) -> usize {
    let constant_idx = bytes[addr + 1];
    print!("{opcode:<16} ");
    print!("{constant_idx:>4} ");
    println!("{}", values[constant_idx as usize]);
    2
}

fn is_constant_instruction(opcode: &OpCode) -> bool {
    match opcode {
        OpCode::Constant => true,
        _ => false,
    }
}
