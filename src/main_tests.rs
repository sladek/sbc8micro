mod cpu;
mod disassembler;
mod memory;
mod status;

use cpu::mos6502;
use disassembler::mos6502::{disassemble, load_opcodes_table};

fn main() {
    let opcodes = load_opcodes_table();
    let mut cpu = mos6502::Cpu::new();
    cpu.set_debug(true);

    let start_addr = 0x0200;
    let size = cpu.memory.load_program_from_acme_file("test.o").unwrap();
    let disassembly = disassemble(&cpu.memory, start_addr, start_addr + size as u16, &opcodes);
    println!("---------------------------");
    println!("Main programm.");
    println!("---------------------------");
    for line in disassembly {
        println!("{}", line);
    }
    println!("---------------------------");
    println!("Debugger output");
    println!("---------------------------");
    cpu.pc = start_addr;
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
    println!("Registers");
    print!("{}", cpu.print_registers());
    println!("Test zero page");
    cpu.memory.hex_dump(0x55, 0x55 + 31);
    println!("Test area");
    cpu.memory.hex_dump(0x04D0, 0x04D0 + 31);
    println!("Upper stack:");
    cpu.memory.hex_dump(0x018f - 0x5f, 0x018f);
    //    log::set_max_level(log::LevelFilter::Debug);
    log::info!("Hahaha {:02X}", 0x34);
    log::debug!("I am here!");
}
