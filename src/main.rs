mod cpu;
mod disassembler;
mod memory;
mod status;

use cpu::mos6502;
use disassembler::mos6502::{disassemble, load_opcodes_table};

fn main() {
    let opcodes = load_opcodes_table();
    let mut cpu = mos6502::Cpu::new();

    let start_addr = 0x0200;
    let start_int = 0x55AA;
    let size = cpu.memory.load_program_from_acme_file("test.o").unwrap();
//    let size_int = cpu.memory.load_program_from_acme_file("int.o").unwrap();
    let disassembly = disassemble(&cpu.memory, start_addr, start_addr + size as u16, &opcodes);
//    cpu.memory.hex_dump(0x600, 0x64F);
    println!("---------------------------");
    println!("Main programm.");
    println!("---------------------------");
    for line in disassembly {
        println!("{}", line);
    }
//    cpu.memory.hex_dump(0x55AA, 0x55AA + 15);
/*
    println!("---------------------------");
    println!("Interrup subroutine.");
    println!("---------------------------");
    let disassembly_int = disassemble(&cpu.memory, start_int, start_int + size_int as u16, &opcodes);
    for line in disassembly_int {
        println!("{}", line);
    }
 */
    println!("---------------------------");
    println!("Debugger output");
    println!("---------------------------");
    cpu.pc = start_addr;
    // Set interrupt routine's address
 //   cpu.memory.write_word(0xaa53, 0xffffu16);
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        if opcode == 0xff {
            println!("---------------------------");
            println!("End of simulation");
            break;
        }
    }
    println!("---------------------------");
    println!(
        "A = 0x{:02X}, X = 0x{:02X}, Y = 0x{:02X}, P = 0x{:02X}, N = {}, V = {}, U = {}, B = {}, D = {}, I = {}, Z = {}, C = {}, SP = 0x{:04X}, PC = 0x{:04X}",
        cpu.a,
        cpu.x,
        cpu.y,
        cpu.p.value,
        cpu.p.is_negative(),
        cpu.p.is_overflow(),
        cpu.p.is_unused(),
        cpu.p.is_break(),
        cpu.p.is_decimal_mode(),
        cpu.p.is_interrupt_disable(),
        cpu.p.is_zero(),
        cpu.p.is_carry(),
        cpu.sp,
        cpu.pc
    );
    println!("Test zero page");
    cpu.memory.hex_dump(0x55, 0x55 + 31);
    println!("test area");
    cpu.memory.hex_dump(0x04D0, 0x04D0 + 31);
    println!("Upper stack:");
    cpu.memory.hex_dump(0x018f - 127, 0x018f);
}
