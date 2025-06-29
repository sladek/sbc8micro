mod disassembler;
mod memory;

use crate::disassembler::i8080_opcodes::OPCODES; // Use mos6502 opcodes
use crate::disassembler::opcode_viewer::view;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    view(OPCODES)
}

