mod cpu;
mod disassembler;
mod memory;
mod status;
use cpu::i8080;
use disassembler::i8080::{disassemble, load_opcodes_table};

use crate::disassembler::i8080_opcodes_const::*;

fn main() {
    let opcodes = load_opcodes_table();
    let mut cpu = i8080::Cpu::new();
    cpu.set_debug(true);
    cpu.memory.write_byte(0x1234, 0x12);
    let program: Vec<u8> = vec![MVI_A, 0x01, MVI_B, 0x05, CMP_B, HLT];
    let start_addr = 0x0200;
    let size = program.len();
    cpu.load_program(&program, start_addr);
    let disassembly = disassemble(&cpu.memory, start_addr, start_addr + size as u16, &opcodes);
    println!("---------------------------");
    println!("Main programm - disassembler");
    println!("---------------------------");
    for line in disassembly {
        println!("{}", line);
    }
    println!("---------------------------");
    println!("Debugger output");
    println!("---------------------------");

    cpu.pc = start_addr;
    cpu.psw.set_carry(true);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        // If HLT end simulation
        if opcode == 0x76 {
            println!("---------------------------");
            println!("End of simulation");
            break;
        }
    }
    print!("{}", cpu.print_registers());
    println!("Test area");
    cpu.memory.hex_dump(0x0200, 0x0200 + 31);
    println!("Upper stack:");
    cpu.memory.hex_dump(0xffff - 0x5f, 0xffff);
    //    log::set_max_level(log::LevelFilter::Debug);
    log::info!("Hahaha {:02X}", 0x34);
    log::debug!("I am here!");
}
