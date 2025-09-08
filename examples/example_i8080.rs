use sbc8micro::cpu::i8080;
use sbc8micro::disassembler::i8080::{disassemble, load_opcodes_table};
use sbc8micro::disassembler::i8080_opcode_consts::*;

fn main() {
    let opcodes = load_opcodes_table();
    let mut cpu = i8080::Cpu::new();
    cpu.set_debug(true);
    cpu.a = 0xff;
    cpu.status.clear_flags();
    cpu.h = 0x02;
    cpu.l = 0x10;
    cpu.b = 0x12;
    cpu.c = 0x34;
    cpu.d = 0x56;
    cpu.e = 0x78;
    cpu.sp = 0x0210;
    cpu.status.set_carry(true);
    let program: Vec<u8> = vec![
        LXI_D, 0x34, 0x12, LXI_H, 0x78, 0x56, PUSH_D, XTHL, PUSH_H, HLT,
    ];
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
    let mut max_op = 100; // Max number of instructions executed. It prevents never endin loops.
    loop {
        let opcode = cpu.memory.read_byte(cpu.pc);
        cpu.step();
        // If HLT end simulation
        if opcode == 0x76 {
            println!("---------------------------");
            println!("End of simulation");
            break;
        }
        max_op -= 1;
        if max_op == 0 {
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
