use std::{collections::HashMap, ops::Range, usize};

use crate::opcode::OpCode;

pub fn disassemble_instructions(
    mut addr: usize,
    bytes: &[u8],
    values: &[f64],
    lines: &HashMap<u8, Range<u8>>,
) {
    let mut line_for_addr = 0;
    while addr < bytes.len() {
        print!("{addr:0>4} ");

        print_line_info(addr, lines, &mut line_for_addr);

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

fn print_line_info(addr: usize, lines: &HashMap<u8, Range<u8>>, line_for_addr: &mut u8) {
    let line_for_next_addr = get_line_for_addr(addr, lines);
    if *line_for_addr == line_for_next_addr {
        print!("   | ");
    } else {
        print!("{line_for_next_addr:>4} ");
    }
    *line_for_addr = line_for_next_addr;
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
    println!(
        "{opcode:<16} {constant_idx:>4} {}",
        values[constant_idx as usize]
    );
    2
}

fn is_constant_instruction(opcode: &OpCode) -> bool {
    match opcode {
        OpCode::Constant => true,
        _ => false,
    }
}

fn get_line_for_addr(addr: usize, lines: &HashMap<u8, Range<u8>>) -> u8 {
    let addr_u8 = &(addr as u8);

    for (line, range) in lines.iter() {
        if range.contains(addr_u8) {
            return *line;
        }
    }
    return 0;
}
