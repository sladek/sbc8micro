mod disassembler;
mod memory;

use disassembler::mos6502::{load_opcodes_table, disassemble};

fn main() {
    let opcodes = load_opcodes_table();

    let mut memory = memory::Memory::new();
    let program = vec![
        0xA9, 0x01,       // LDA #$01
        0x8D, 0x00, 0x02, // STA $0200
        0xE8,             // INX
        0xF0, 0xFD,       // BEQ $0600
        0x00              // BRK
    ];
    let start = 0x0600;
    memory.load_program(&program, start);
    let disassembly = disassemble(&memory, start, start + program.len() as u16, &opcodes);

    for line in disassembly {
        println!("{}", line);
    }
}
