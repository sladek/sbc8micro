use sbc8micro::disassembler::i8080_opcode::OpcodeView as op_i8080; // Use i8080 opcodes
use sbc8micro::disassembler::mos6502_opcode::OpcodeView as op_mos6502; // Use mos6502 opcodes
use sbc8micro::disassembler::opcode_viewer::view;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let i8080 = true;
    if i8080 {
        let op_view = op_i8080::new();
        view(&op_view)
    } else {
        let op_view = op_mos6502::new();
        view(&op_view)
    }
}
